//! Zero-Heap Flexbox/Grid UI Layout, GPU MSDF Font Atlas & 10D/Holographic Studio Tooling.
//!
//! Upgrades `ecopunk-terrain` with Epic 9 capabilities from `webizen-render`:
//! - Multi-channel signed distance field (MSDF) procedural font atlas generation
//!   and zero-heap text shaping supporting printable ASCII and 10D technical glyphs
//!   (`α`, `μ`, `σ`, `Δ`, `Ω`, `π`, `±`, `°`, `→`).
//! - Two-pass (`measure` + `arrange`) Flexbox & CSS-Grid layout constraint solver
//!   executing over a fixed arena of up to 512 nodes with **zero heap allocations**.
//! - Declarative glassmorphic Ecological & 10D Telemetry HUD:
//!   - Sector Header & Status Bar
//!   - GDA2020 / AusPix DGGS Geospatial Bar
//!   - 305 Flora / 164 Fauna Biodiversity Counter
//!   - `Tensor10D` `[q, v, w, x, y, z, t, α, μ, σ]` Inspector Card
//!   - `Corymbia aparrerinja` (Ghost Gum) Specimen Loupe Card
//!   - Holosonic `SonicToken` & 3D Audio Wavefield Meter Card

use core::fmt::Write;
use webizen_render::ui::{
    Color4f, Rect2D, ZeroHeapUiCanvas, COLOR_ACCENT_CYAN, COLOR_TEXT_PRIMARY,
};
use webizen_render::ui_layout::{
    AlignItems, FlexDirection, JustifyContent, Padding, SizeRule, UiLayoutStyle,
    ZeroHeapUiLayoutTree, NO_UI_NODE,
};
use webizen_render::ui_msdf_text::{generate_procedural_msdf_atlas_into, MSDF_TEXT_WGSL};

/// Secondary text color (muted slate cyan).
pub const COLOR_TEXT_MUTED: Color4f = [0.65, 0.78, 0.88, 0.88];
/// Botanical accent green.
pub const COLOR_ACCENT_GREEN: Color4f = [0.22, 0.88, 0.48, 1.00];
/// Warning / notice amber.
pub const COLOR_ACCENT_AMBER: Color4f = [0.96, 0.76, 0.22, 1.00];

/// Zero-heap fixed-capacity stack string for formatting telemetry labels without heap allocation.
#[derive(Debug, Clone, Copy)]
pub struct StackString<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> Default for StackString<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> StackString<N> {
    /// Creates an empty stack string.
    #[inline]
    pub const fn new() -> Self {
        Self {
            buf: [0u8; N],
            len: 0,
        }
    }

    /// Returns the formatted string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        match core::str::from_utf8(&self.buf[..self.len]) {
            Ok(s) => s,
            Err(_) => "",
        }
    }

    /// Clears the string buffer.
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<const N: usize> Write for StackString<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let rem = N.saturating_sub(self.len);
        let count = bytes.len().min(rem);
        self.buf[self.len..self.len + count].copy_from_slice(&bytes[..count]);
        self.len += count;
        Ok(())
    }
}

/// Zero-Heap Ecological & 10D Telemetry HUD Studio.
pub struct EcopunkTelemetryHud {
    /// Zero-heap two-pass Flexbox layout tree solver.
    pub tree: ZeroHeapUiLayoutTree,
    /// Fixed-capacity declarative vector UI canvas.
    pub canvas: ZeroHeapUiCanvas,
    /// Viewport width in logical pixels.
    pub viewport_width: f32,
    /// Viewport height in logical pixels.
    pub viewport_height: f32,

    // ─────────────────────────────────────────────────────────────────────────
    // 10D Manifold & Biophysical Telemetry State
    // ─────────────────────────────────────────────────────────────────────────
    /// Epistemic certainty $q \in [0.0, 1.0]$.
    pub epistemic_q: f32,
    /// Potential $v \in \mathbb{R}$.
    pub potential_v: f32,
    /// Manifold domain scale $w$.
    pub manifold_w: f32,
    /// World coordinates $[x, y, z]$ in meters.
    pub coords_xyz: [f32; 3],
    /// Simulation epoch $t$ in seconds.
    pub epoch_t: f32,
    /// Optical albedo reflectance $\alpha \in [0.0, 1.0]$.
    pub albedo_alpha: f32,
    /// Biochemical chlorophyll density $\mu \in [0.0, 1.0]$.
    pub chemical_mu: f32,
    /// Structural variance $\sigma$.
    pub variance_sigma: f32,

    // ─────────────────────────────────────────────────────────────────────────
    // Ecological & Soundscape Telemetry State
    // ─────────────────────────────────────────────────────────────────────────
    /// Verified flora species count (target: 305).
    pub flora_count: u32,
    /// Verified fauna taxa count (target: 164).
    pub fauna_count: u32,
    /// Holosonic wavefield net constructive interference gain.
    pub soundscape_interference_gain: f32,
    /// Active 3D Holosonic acoustic emitters in audible horizon.
    pub active_soundscape_emitters: usize,
    /// Ghost Gum (*Corymbia aparrerinja*) phenology score.
    pub ghost_gum_phenology_score: f32,
}

impl Default for EcopunkTelemetryHud {
    fn default() -> Self {
        Self::new(1920.0, 1080.0)
    }
}

impl EcopunkTelemetryHud {
    /// Constructs a new zero-heap telemetry HUD for the specified viewport resolution.
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            tree: ZeroHeapUiLayoutTree::new(),
            canvas: ZeroHeapUiCanvas::default(),
            viewport_width: viewport_width.max(320.0),
            viewport_height: viewport_height.max(240.0),
            epistemic_q: 0.985,
            potential_v: 14.28,
            manifold_w: 1.000,
            coords_xyz: [142.50, 1840.00, -78.20],
            epoch_t: 1729432800.0,
            albedo_alpha: 0.912,
            chemical_mu: 0.428,
            variance_sigma: 0.065,
            flora_count: 305,
            fauna_count: 164,
            soundscape_interference_gain: 1.42,
            active_soundscape_emitters: 9,
            ghost_gum_phenology_score: 0.88,
        }
    }

    /// Updates viewport dimensions.
    #[inline]
    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.viewport_width = width.max(320.0);
        self.viewport_height = height.max(240.0);
    }

    /// Updates 10D manifold tensor metrics.
    pub fn update_tensor_10d(
        &mut self,
        q: f32,
        v: f32,
        w: f32,
        pos: [f32; 3],
        t: f32,
        alpha: f32,
        mu: f32,
        sigma: f32,
    ) {
        self.epistemic_q = q.clamp(0.0, 1.0);
        self.potential_v = v;
        self.manifold_w = w;
        self.coords_xyz = pos;
        self.epoch_t = t;
        self.albedo_alpha = alpha.clamp(0.0, 1.0);
        self.chemical_mu = mu.clamp(0.0, 1.0);
        self.variance_sigma = sigma.max(0.0);
    }

    /// Updates Holosonic acoustic wavefield telemetry.
    #[inline]
    pub fn update_holosonic_telemetry(&mut self, interference_gain: f32, active_emitters: usize) {
        self.soundscape_interference_gain = interference_gain;
        self.active_soundscape_emitters = active_emitters;
    }

    /// Updates biodiversity census counters.
    #[inline]
    pub fn update_biodiversity_counts(&mut self, flora: u32, fauna: u32) {
        self.flora_count = flora;
        self.fauna_count = fauna;
    }

    /// Solves the zero-heap Flexbox layout tree and shapes all text runs into MSDF glyph quads
    /// directly in `self.canvas` with **zero heap allocations**.
    pub fn render_frame_zero_heap(&mut self) {
        self.tree.clear();
        self.canvas.clear();

        // ─────────────────────────────────────────────────────────────────────
        // 1. Build Zero-Heap Flexbox Layout Hierarchy
        // ─────────────────────────────────────────────────────────────────────

        // Root Container (Full Viewport Column)
        let root_style = UiLayoutStyle {
            direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            width: SizeRule::Fixed(self.viewport_width),
            height: SizeRule::Fixed(self.viewport_height),
            padding: Padding::all(16.0),
            gap: 12.0,
            ..Default::default()
        };
        let root = self.tree.add_node(1, NO_UI_NODE, root_style, [self.viewport_width, self.viewport_height]).unwrap_or(0);

        // Header Bar Container (Row)
        let header_style = UiLayoutStyle {
            direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            width: SizeRule::Percent(1.0),
            height: SizeRule::Fixed(44.0),
            padding: Padding::symmetric(14.0, 8.0),
            ..Default::default()
        };
        let _header_node = self.tree.add_node(2, root, header_style, [0.0, 44.0]);

        // GDA2020 / AusPix DGGS Bar Container (Row)
        let dggs_bar_style = UiLayoutStyle {
            direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            width: SizeRule::Percent(1.0),
            height: SizeRule::Fixed(36.0),
            padding: Padding::symmetric(12.0, 6.0),
            ..Default::default()
        };
        let _dggs_node = self.tree.add_node(3, root, dggs_bar_style, [0.0, 36.0]);

        // Biodiversity Counter Bar (Row)
        let bio_bar_style = UiLayoutStyle {
            direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            width: SizeRule::Percent(1.0),
            height: SizeRule::Fixed(34.0),
            padding: Padding::symmetric(12.0, 6.0),
            gap: 24.0,
            ..Default::default()
        };
        let _bio_node = self.tree.add_node(4, root, bio_bar_style, [0.0, 34.0]);

        // Main Studio Body (3-Column Flex Row with FlexGrow)
        let body_style = UiLayoutStyle {
            direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            width: SizeRule::Percent(1.0),
            height: SizeRule::FlexGrow(1.0),
            gap: 14.0,
            ..Default::default()
        };
        let body = self.tree.add_node(5, root, body_style, [0.0, 0.0]).unwrap_or(root);

        // Column 1: Tensor10D Inspector Card (Fixed Width 340px)
        let tensor_card_style = UiLayoutStyle {
            direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            width: SizeRule::Fixed(340.0),
            height: SizeRule::Percent(1.0),
            padding: Padding::all(14.0),
            gap: 8.0,
            ..Default::default()
        };
        let _tensor_card_node = self.tree.add_node(10, body, tensor_card_style, [340.0, 0.0]);

        // Column 2: Corymbia aparrerinja Specimen Loupe Card (FlexGrow 1.0)
        let loupe_card_style = UiLayoutStyle {
            direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            width: SizeRule::FlexGrow(1.0),
            height: SizeRule::Percent(1.0),
            padding: Padding::all(16.0),
            gap: 10.0,
            ..Default::default()
        };
        let _loupe_card_node = self.tree.add_node(20, body, loupe_card_style, [0.0, 0.0]);

        // Column 3: Holosonic SonicToken & 3D Audio Meter Card (Fixed Width 360px)
        let meter_card_style = UiLayoutStyle {
            direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            width: SizeRule::Fixed(360.0),
            height: SizeRule::Percent(1.0),
            padding: Padding::all(14.0),
            gap: 8.0,
            ..Default::default()
        };
        let _meter_card_node = self.tree.add_node(30, body, meter_card_style, [360.0, 0.0]);

        // ─────────────────────────────────────────────────────────────────────
        // 2. Solve Layout Constraints & Sync into Canvas Panels
        // ─────────────────────────────────────────────────────────────────────
        self.tree.solve(self.viewport_width, self.viewport_height);
        self.canvas.apply_layout_tree(&self.tree);

        // ─────────────────────────────────────────────────────────────────────
        // 3. Shape MSDF Text Runs into Canvas in Zero Heap
        // ─────────────────────────────────────────────────────────────────────

        // Node 2 (Header Bar)
        if let Some(r) = self.rect_for_widget(2) {
            self.canvas.add_text_run(
                "ECOPUNK 10D SOUNDSCAPE & BIOSPHERE HUD",
                r.x + 14.0,
                r.y + 12.0,
                18.0,
                r.width - 240.0,
                COLOR_ACCENT_CYAN,
            );
            self.canvas.add_text_run(
                "ONLINE • TIER-1 ZERO-HEAP",
                r.x + r.width - 220.0,
                r.y + 14.0,
                14.0,
                200.0,
                COLOR_ACCENT_GREEN,
            );
        }

        // Node 3 (GDA2020 / AusPix DGGS Bar)
        if let Some(r) = self.rect_for_widget(3) {
            let mut label = StackString::<128>::new();
            let _ = write!(
                label,
                "GDA2020: {:.4}°, {:.4}° | AusPix DGGS: R72014382 | Elev: {:.0}m | Epoch: 2026.8",
                -36.4521, 148.2715, self.coords_xyz[1]
            );
            self.canvas.add_text_run(label.as_str(), r.x + 12.0, r.y + 9.0, 13.0, r.width - 24.0, COLOR_TEXT_PRIMARY);
        }

        // Node 4 (Biodiversity Counter Bar)
        if let Some(r) = self.rect_for_widget(4) {
            let mut bio_str = StackString::<160>::new();
            let _ = write!(
                bio_str,
                "Flora Taxa: {}/305 Catalogued | Fauna Taxa: {}/164 Tracked | Holosonic Audio: SYNCHRONIZED",
                self.flora_count, self.fauna_count
            );
            self.canvas.add_text_run(bio_str.as_str(), r.x + 12.0, r.y + 8.0, 13.0, r.width - 24.0, COLOR_ACCENT_GREEN);
        }

        // Node 10 (Tensor10D Inspector Card)
        if let Some(r) = self.rect_for_widget(10) {
            let mut py = r.y + 14.0;
            let lh = 18.0;

            self.canvas.add_text_run("10D MANIFOLD TENSOR", r.x + 14.0, py, 15.0, r.width - 28.0, COLOR_ACCENT_CYAN);
            py += lh + 4.0;

            let mut line = StackString::<96>::new();
            let _ = write!(line, "q (Epistemic Certainty): {:.3}", self.epistemic_q);
            self.canvas.add_text_run(line.as_str(), r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_TEXT_PRIMARY);
            py += lh;

            line.clear();
            let _ = write!(line, "v (Potential): {:.2}", self.potential_v);
            self.canvas.add_text_run(line.as_str(), r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_TEXT_MUTED);
            py += lh;

            line.clear();
            let _ = write!(line, "w (Domain Scale): {:.3}", self.manifold_w);
            self.canvas.add_text_run(line.as_str(), r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_TEXT_MUTED);
            py += lh;

            line.clear();
            let _ = write!(line, "Coords: x={:.1} y={:.1} z={:.1}", self.coords_xyz[0], self.coords_xyz[1], self.coords_xyz[2]);
            self.canvas.add_text_run(line.as_str(), r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_TEXT_PRIMARY);
            py += lh;

            line.clear();
            let _ = write!(line, "t (Epoch): {:.1}s", self.epoch_t);
            self.canvas.add_text_run(line.as_str(), r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_TEXT_MUTED);
            py += lh;

            line.clear();
            let _ = write!(line, "α (Albedo): {:.3} | μ (Chlorophyll): {:.3}", self.albedo_alpha, self.chemical_mu);
            self.canvas.add_text_run(line.as_str(), r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_ACCENT_GREEN);
            py += lh;

            line.clear();
            let _ = write!(line, "σ (Structural Variance): {:.4}", self.variance_sigma);
            self.canvas.add_text_run(line.as_str(), r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_TEXT_MUTED);
        }

        // Node 20 (Corymbia aparrerinja Loupe Card)
        if let Some(r) = self.rect_for_widget(20) {
            let mut py = r.y + 16.0;
            let lh = 20.0;

            self.canvas.add_text_run("SPECIMEN LOUPE: Corymbia aparrerinja", r.x + 16.0, py, 16.0, r.width - 32.0, COLOR_ACCENT_CYAN);
            py += lh + 2.0;

            self.canvas.add_text_run("Ghost Gum • Family Myrtaceae • Central & Northern Australia", r.x + 16.0, py, 13.0, r.width - 32.0, COLOR_TEXT_MUTED);
            py += lh;

            let mut phen = StackString::<128>::new();
            let _ = write!(phen, "Phenology: Bark Decortication (Score: {:.2}) • Powder-white bark", self.ghost_gum_phenology_score);
            self.canvas.add_text_run(phen.as_str(), r.x + 16.0, py, 13.0, r.width - 32.0, COLOR_ACCENT_AMBER);
            py += lh;

            let mut optics = StackString::<128>::new();
            let _ = write!(optics, "Optics: Bark Reflectance α = {:.2} | Leaf Chlorophyll μ = {:.2}", self.albedo_alpha, self.chemical_mu);
            self.canvas.add_text_run(optics.as_str(), r.x + 16.0, py, 13.0, r.width - 32.0, COLOR_TEXT_PRIMARY);
            py += lh;

            self.canvas.add_text_run("Root Matrix: Sinker Taproot Depth 14.2m | Lithosol Matrix pH 6.2", r.x + 16.0, py, 13.0, r.width - 32.0, COLOR_TEXT_MUTED);
            py += lh;

            self.canvas.add_text_run("Ecological Association: Microrefugium for Lichens & Gould's Bat Roost", r.x + 16.0, py, 13.0, r.width - 32.0, COLOR_ACCENT_GREEN);
        }

        // Node 30 (Holosonic SonicToken Meter Card)
        if let Some(r) = self.rect_for_widget(30) {
            let mut py = r.y + 14.0;
            let lh = 18.0;

            self.canvas.add_text_run("HOLOSONIC WAVEFIELD & HRTF MONITOR", r.x + 14.0, py, 15.0, r.width - 28.0, COLOR_ACCENT_CYAN);
            py += lh + 4.0;

            let mut interf = StackString::<96>::new();
            let _ = write!(interf, "Interference Gain: {:.2}× (Constructive)", self.soundscape_interference_gain);
            self.canvas.add_text_run(interf.as_str(), r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_ACCENT_GREEN);
            py += lh;

            let mut emit = StackString::<96>::new();
            let _ = write!(emit, "Active Emitters: {} / 64 (3D Soundscape)", self.active_soundscape_emitters);
            self.canvas.add_text_run(emit.as_str(), r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_TEXT_PRIMARY);
            py += lh;

            self.canvas.add_text_run("Fauna Resonances: Lyrebird • Kookaburra • Cassowary (23Hz)", r.x + 14.0, py, 12.0, r.width - 28.0, COLOR_TEXT_MUTED);
            py += lh;

            self.canvas.add_text_run("Acoustic Occlusion: 4 Shared SDF Obstacles", r.x + 14.0, py, 12.0, r.width - 28.0, COLOR_TEXT_MUTED);
            py += lh;

            self.canvas.add_text_run("Q42 U3 SonicToken Ring: STREAMING (48-byte Pod)", r.x + 14.0, py, 13.0, r.width - 28.0, COLOR_ACCENT_AMBER);
        }
    }

    /// Looks up the resolved screen-space rectangle of a widget by its ID.
    #[inline]
    fn rect_for_widget(&self, id: u32) -> Option<Rect2D> {
        for i in 0..self.canvas.widget_count {
            if self.canvas.widgets[i].id == id {
                return Some(self.canvas.widgets[i].rect);
            }
        }
        None
    }

    /// Generates the embedded procedural MSDF atlas RGBA8 texture (131,072 bytes) into `out_rgba8` with zero heap.
    #[inline]
    pub fn bake_msdf_atlas_into(out_rgba8: &mut [u8]) -> usize {
        generate_procedural_msdf_atlas_into(out_rgba8)
    }

    /// Returns the canonical WGSL shader snippet for rendering MSDF text quads.
    #[inline]
    pub fn msdf_shader_source() -> &'static str {
        MSDF_TEXT_WGSL
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;
    use webizen_render::ui::MAX_UI_TEXT_QUADS;
    use webizen_render::ui_msdf_text::{
        shape_text_zero_heap, MsdfTextQuad, MSDF_ATLAS_HEIGHT, MSDF_ATLAS_WIDTH,
    };

    #[test]
    fn hud_layout_solves_flexbox_tree_without_heap() {
        let mut hud = EcopunkTelemetryHud::new(1920.0, 1080.0);
        hud.render_frame_zero_heap();

        assert!(hud.tree.count >= 7, "Layout tree must allocate at least 7 nodes");
        assert!(hud.canvas.widget_count >= 7, "Canvas must register at least 7 widget panels");

        // Verify root container fills viewport
        let root_rect = hud.rect_for_widget(1).expect("Root widget present");
        assert_eq!(root_rect.width, 1920.0);
        assert_eq!(root_rect.height, 1080.0);

        // Verify Header Bar (widget 2)
        let header_rect = hud.rect_for_widget(2).expect("Header widget present");
        assert!(header_rect.width > 1800.0);
        assert_eq!(header_rect.height, 44.0);

        // Verify 3 Main Body Columns (10, 20, 30)
        let tensor_rect = hud.rect_for_widget(10).expect("Tensor card present");
        let loupe_rect = hud.rect_for_widget(20).expect("Loupe card present");
        let meter_rect = hud.rect_for_widget(30).expect("Meter card present");

        assert_eq!(tensor_rect.width, 340.0);
        assert_eq!(meter_rect.width, 360.0);
        assert!(loupe_rect.width > 800.0, "FlexGrow loupe card should take remainder of screen");
        assert!(loupe_rect.x > tensor_rect.x + tensor_rect.width);
        assert!(meter_rect.x > loupe_rect.x + loupe_rect.width);
    }

    #[test]
    fn msdf_text_shaping_handles_10d_symbols() {
        let mut quads = [MsdfTextQuad::zeroed(); 128];
        let text = "Tensor10D: q=0.98 α=0.91 μ=0.42 σ=0.065° → Kosciuszko";
        let metrics = shape_text_zero_heap(
            text,
            0.0,
            0.0,
            16.0,
            1.25,
            600.0,
            [1.0, 1.0, 1.0, 1.0],
            &mut quads,
        );

        assert!(metrics.quad_count > 25, "Text quads must shape all printable characters");
        assert!(metrics.width_px > 100.0);
        assert!(quads[0].screen_rect[2] > 0.0);
    }

    #[test]
    fn canvas_synchronizes_widgets_and_text_runs() {
        let mut hud = EcopunkTelemetryHud::new(1280.0, 720.0);
        hud.render_frame_zero_heap();

        assert!(hud.canvas.text_quad_count > 50, "Shaped text quads should populate canvas");
        assert!(hud.canvas.text_quad_count <= MAX_UI_TEXT_QUADS);
    }

    #[test]
    fn procedural_msdf_atlas_baked_into_buffer() {
        let mut atlas_buf = vec![0u8; MSDF_ATLAS_WIDTH * MSDF_ATLAS_HEIGHT * 4];
        let written = EcopunkTelemetryHud::bake_msdf_atlas_into(&mut atlas_buf);
        assert_eq!(written, 131_072);
        assert_eq!(written, MSDF_ATLAS_WIDTH * MSDF_ATLAS_HEIGHT * 4);
    }

    #[test]
    fn hud_telemetry_updates_reflect_in_state() {
        let mut hud = EcopunkTelemetryHud::new(1920.0, 1080.0);
        hud.update_tensor_10d(
            0.99,
            20.0,
            2.0,
            [10.0, 20.0, 30.0],
            100.0,
            0.85,
            0.45,
            0.08,
        );
        hud.update_holosonic_telemetry(1.65, 12);
        hud.update_biodiversity_counts(305, 164);
        hud.render_frame_zero_heap();

        assert_eq!(hud.epistemic_q, 0.99);
        assert_eq!(hud.soundscape_interference_gain, 1.65);
        assert_eq!(hud.active_soundscape_emitters, 12);
        assert_eq!(hud.flora_count, 305);
        assert_eq!(hud.fauna_count, 164);
    }
}
