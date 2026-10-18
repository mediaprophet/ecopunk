//! Dynamic Memory & GPU Compute Throughput Governor
//! Balances terrain simulation, voxel DAG editing, and Cook-Torrance raymarching
//! against concurrent local LLM inference and hardware limits.

use super::telemetry::{GpuTelemetrySample, HardwareMemoryClass, LlmWorkloadPhase};
use serde::{Deserialize, Serialize};

/// Mode of operation for the hardware governor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernorMode {
    /// Reactive closed-loop feedback controller (automated PID/VibeScript).
    AutomatedScripted,
    /// Direct user/operator manual override (fixed quotas and profiles).
    ManualOverride,
}

/// Dynamic resource budget allocated to the terrain and environmental simulation.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainBudget {
    /// Maximum byte budget for the HypermediaGpuRing (.e64 tile cache)
    pub gpu_ring_budget_bytes: u64,
    /// Maximum raymarching step count per pixel (32..256)
    pub max_raymarching_steps: u32,
    /// Maximum compute queue execution time per frame (milliseconds)
    pub compute_time_budget_ms: f32,
    /// Active radius in meters for high-detail Sparse Voxel DAG deformation
    pub svo_voxel_radius_m: f32,
    /// Whether real-time digging/soil displacement is permitted this frame
    pub allow_subsurface_voxel_edit: bool,
    /// Whether cache eviction is demanded due to high memory pressure
    pub demand_cache_flush: bool,
}

/// Hardware Governor configuration parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernorConfig {
    pub mode: GovernorMode,
    /// Target frame rate in Hz (default: 60.0)
    pub target_fps: f32,
    /// Explicit manual memory cap for terrain (bytes, used in ManualOverride mode)
    pub manual_terrain_vram_cap: u64,
    /// Explicit manual compute time budget (ms, used in ManualOverride mode)
    pub manual_compute_time_budget_ms: f32,
    /// Explicit manual raymarching step limit (used in ManualOverride mode)
    pub manual_raymarching_steps: u32,
}

impl Default for GovernorConfig {
    fn default() -> Self {
        Self {
            mode: GovernorMode::AutomatedScripted,
            target_fps: 60.0,
            manual_terrain_vram_cap: 512 * 1024 * 1024,
            manual_compute_time_budget_ms: 6.0,
            manual_raymarching_steps: 96,
        }
    }
}

/// Hardware Governor managing VRAM allocation and compute queue time-slicing.
#[derive(Debug, Clone)]
pub struct VramAndComputeGovernor {
    pub config: GovernorConfig,
    /// Hysteresis tracker for consecutive high-pressure frames
    pub high_pressure_streak: u32,
}

impl VramAndComputeGovernor {
    pub fn new(config: GovernorConfig) -> Self {
        Self {
            config,
            high_pressure_streak: 0,
        }
    }

    /// Evaluates live telemetry and returns the optimal terrain execution budget.
    pub fn evaluate_budget(
        &mut self,
        telemetry: &GpuTelemetrySample,
        last_frame_time_ms: f32,
    ) -> TerrainBudget {
        if self.config.mode == GovernorMode::ManualOverride {
            return TerrainBudget {
                gpu_ring_budget_bytes: self.config.manual_terrain_vram_cap,
                max_raymarching_steps: self.config.manual_raymarching_steps,
                compute_time_budget_ms: self.config.manual_compute_time_budget_ms,
                svo_voxel_radius_m: 16.0,
                allow_subsurface_voxel_edit: true,
                demand_cache_flush: telemetry.vram_pressure_ratio() > 0.95,
            };
        }

        // --- Automated Scripted Mode ---
        let mem_class = telemetry.memory_class();
        let pressure = telemetry.vram_pressure_ratio();

        if pressure > 0.90 {
            self.high_pressure_streak = self.high_pressure_streak.saturating_add(1);
        } else {
            self.high_pressure_streak = 0;
        }

        // 1. Base GPU Ring Budget based on hardware memory tier
        let base_ring_bytes: u64 = match mem_class {
            HardwareMemoryClass::IntegratedEntry => 64 * 1024 * 1024,      // 64 MiB
            HardwareMemoryClass::MainstreamDesktop => 256 * 1024 * 1024,   // 256 MiB
            HardwareMemoryClass::EnthusiastWorkstation => 1024 * 1024 * 1024, // 1024 MiB
            HardwareMemoryClass::EnterpriseMultiGpu => 8192 * 1024 * 1024,  // 8192 MiB
        };

        // 2. Modulate based on LLM Phase and VRAM Pressure
        let (gpu_ring_budget_bytes, max_raymarching_steps, compute_time_budget_ms, svo_voxel_radius_m) =
            match telemetry.llm_phase {
                LlmWorkloadPhase::ActiveDecode => {
                    // LLM is actively streaming tokens: Protect memory bus and compute queues
                    let ring = (base_ring_bytes / 2).max(64 * 1024 * 1024);
                    let steps = match mem_class {
                        HardwareMemoryClass::IntegratedEntry => 32,
                        HardwareMemoryClass::MainstreamDesktop => 64,
                        HardwareMemoryClass::EnthusiastWorkstation => 96,
                        HardwareMemoryClass::EnterpriseMultiGpu => 128,
                    };
                    // Cap compute time strictly to <= 2.5ms to avoid stalling LLM tokens
                    let compute_ms = 2.5f32;
                    let svo_radius = 8.0f32;
                    (ring, steps, compute_ms, svo_radius)
                }
                LlmWorkloadPhase::PromptProcessing => {
                    // LLM is ingesting prompt: Peak compute spike, minimize graphics compute
                    let ring = (base_ring_bytes / 2).max(64 * 1024 * 1024);
                    let steps = 48;
                    let compute_ms = 1.0f32;
                    let svo_radius = 4.0f32;
                    (ring, steps, compute_ms, svo_radius)
                }
                LlmWorkloadPhase::Idle => {
                    // LLM is idle: Full terrain fidelity and hydraulic simulation
                    let ring = base_ring_bytes;
                    let steps = match mem_class {
                        HardwareMemoryClass::IntegratedEntry => 48,
                        HardwareMemoryClass::MainstreamDesktop => 96,
                        HardwareMemoryClass::EnthusiastWorkstation => 160,
                        HardwareMemoryClass::EnterpriseMultiGpu => 256,
                    };
                    let compute_ms = 12.0f32;
                    let svo_radius = match mem_class {
                        HardwareMemoryClass::IntegratedEntry => 0.0, // No SVO on iGPU
                        HardwareMemoryClass::MainstreamDesktop => 12.0,
                        HardwareMemoryClass::EnthusiastWorkstation => 24.0,
                        HardwareMemoryClass::EnterpriseMultiGpu => 48.0,
                    };
                    (ring, steps, compute_ms, svo_radius)
                }
            };

        // 3. Frame-pacing feedback: Drop compute time if exceeding frame deadline
        let target_frame_ms = 1000.0 / self.config.target_fps;
        let adjusted_compute_ms = if last_frame_time_ms > target_frame_ms * 1.1 {
            (compute_time_budget_ms * 0.75).max(1.5)
        } else {
            compute_time_budget_ms
        };

        let demand_cache_flush = self.high_pressure_streak >= 3 || pressure > 0.94;
        let allow_subsurface_voxel_edit =
            svo_voxel_radius_m > 0.0 && telemetry.llm_phase != LlmWorkloadPhase::PromptProcessing;

        TerrainBudget {
            gpu_ring_budget_bytes,
            max_raymarching_steps,
            compute_time_budget_ms: adjusted_compute_ms,
            svo_voxel_radius_m,
            allow_subsurface_voxel_edit,
            demand_cache_flush,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_active_decode_throttling() {
        let mut governor = VramAndComputeGovernor::new(GovernorConfig::default());
        let sample = GpuTelemetrySample {
            vram_total_bytes: 16 * 1024 * 1024 * 1024, // 16 GB RTX 4090
            vram_used_bytes: 8 * 1024 * 1024 * 1024,
            vram_budget_bytes: 14 * 1024 * 1024 * 1024,
            compute_utilization_pct: 70,
            temperature_celsius: 65,
            board_power_watts: 240.0,
            llm_phase: LlmWorkloadPhase::ActiveDecode,
            llm_decode_rate_toks: 42.0,
        };

        let budget = governor.evaluate_budget(&sample, 16.0);
        // Compute budget must be capped at 2.5 ms during active LLM decode
        assert!(budget.compute_time_budget_ms <= 2.5);
        assert!(!budget.demand_cache_flush);
    }

    #[test]
    fn test_llm_idle_headroom_expansion() {
        let mut governor = VramAndComputeGovernor::new(GovernorConfig::default());
        let sample = GpuTelemetrySample {
            vram_total_bytes: 16 * 1024 * 1024 * 1024,
            vram_used_bytes: 4 * 1024 * 1024 * 1024,
            vram_budget_bytes: 14 * 1024 * 1024 * 1024,
            compute_utilization_pct: 15,
            temperature_celsius: 52,
            board_power_watts: 90.0,
            llm_phase: LlmWorkloadPhase::Idle,
            llm_decode_rate_toks: 0.0,
        };

        let budget = governor.evaluate_budget(&sample, 15.0);
        // Idle allows high compute budget and full raymarching steps
        assert!(budget.compute_time_budget_ms >= 10.0);
        assert!(budget.max_raymarching_steps >= 128);
        assert!(budget.gpu_ring_budget_bytes >= 1024 * 1024 * 1024);
    }
}
