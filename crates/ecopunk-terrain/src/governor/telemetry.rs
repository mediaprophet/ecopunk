//! GPU Telemetry & System Resource Sampling
//! Tracks physical memory, compute queue utilization, thermal status, and concurrent LLM phases.

use serde::{Deserialize, Serialize};

/// Execution state of the concurrent local LLM inference engine.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LlmWorkloadPhase {
    /// LLM is loaded in mmap cache, but no active generation or prompt ingestion is occurring.
    Idle = 0,
    /// LLM is ingesting prompt tokens: high compute saturation and memory bandwidth.
    PromptProcessing = 1,
    /// LLM is actively generating tokens autoregressively (e.g. 30–60 tok/s).
    /// Bound by memory bus bandwidth; periodic DRAM fetches every 16–30 ms.
    ActiveDecode = 2,
}

/// System hardware memory class based on total accessible VRAM.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HardwareMemoryClass {
    /// 2–6 GB (Shared system memory / iGPU / Mobile)
    IntegratedEntry,
    /// 8–12 GB (Mainstream discrete GPU / Laptop, e.g. RTX 4060)
    MainstreamDesktop,
    /// 16–24 GB (Enthusiast workstation, e.g. RTX 4090, Apple M3 Max)
    EnthusiastWorkstation,
    /// 48–192+ GB (Multi-GPU workstation / Server cluster / Apple M2 Ultra)
    EnterpriseMultiGpu,
}

/// Telemetry snapshot sampled from physical GPU hardware (NVML, DXGI, Vulkan, Metal, or WebGPU).
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpuTelemetrySample {
    /// Total installed physical VRAM in bytes
    pub vram_total_bytes: u64,
    /// Currently allocated VRAM across all OS processes in bytes
    pub vram_used_bytes: u64,
    /// Safe OS-recommended memory budget ceiling in bytes
    pub vram_budget_bytes: u64,
    /// GPU compute engine utilization percentage [0..100]
    pub compute_utilization_pct: u8,
    /// GPU junction temperature in degrees Celsius
    pub temperature_celsius: u8,
    /// Total instantaneous board power consumption in Watts
    pub board_power_watts: f32,
    /// Current execution phase of the local LLM inference engine
    pub llm_phase: LlmWorkloadPhase,
    /// Current token generation speed (tokens/sec)
    pub llm_decode_rate_toks: f32,
}

impl GpuTelemetrySample {
    /// Classifies the host hardware into its memory capability class.
    pub fn memory_class(&self) -> HardwareMemoryClass {
        const GB: u64 = 1024 * 1024 * 1024;
        if self.vram_total_bytes < 7 * GB {
            HardwareMemoryClass::IntegratedEntry
        } else if self.vram_total_bytes < 15 * GB {
            HardwareMemoryClass::MainstreamDesktop
        } else if self.vram_total_bytes < 32 * GB {
            HardwareMemoryClass::EnthusiastWorkstation
        } else {
            HardwareMemoryClass::EnterpriseMultiGpu
        }
    }

    /// Computes current VRAM pressure as a ratio [0.0..1.0+].
    #[inline]
    pub fn vram_pressure_ratio(&self) -> f32 {
        if self.vram_budget_bytes == 0 {
            0.0
        } else {
            self.vram_used_bytes as f32 / self.vram_budget_bytes as f32
        }
    }
}
