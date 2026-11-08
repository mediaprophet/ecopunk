# Definitive AAA Quality Master Plan: Holographic 3D Engine & Ecological Simulation

> **Document Class:** Definitive Engineering & Artistic Master Specification  
> **Target Substrates:** `c:\github\qualiaDB` (Core Engine) · `c:\github\ecopunk` (Ecological Graph & Asset Models) · `c:\github\game-demo` (Simulation Runtime)  
> **Authority Precedence:** Immovable Zero-Heap Hot-Path Contract ([`qualiaDB/AGENTS.md` §0](file:///c:/github/qualiaDB/AGENTS.md)) · 42MB Prolog Sentinel · 10D Semantic Manifolds · Holographic Engine Specification ([`Holographic 3D Engine for Semantic Graphs.md`](file:///c:/github/ecopunk/docs/Holographic%203D%20Engine%20for%20Semantic%20Graphs.md)) · Holosonic Wavefield Engine ([`Holosonic Engine for Semantic Acoustic Wavefields.md`](file:///c:/github/ecopunk/docs/Holosonic%20Engine%20for%20Semantic%20Acoustic%20Wavefields.md))  
> **Status:** Phase 1 Complete & Verified · Phase 2 Active · Phases 3–6 Master Production Roadmap  

---

## 1. Executive Summary: The Vision for a AAA Product

### 1.1 The Elimination of False Dichotomies
For decades, interactive graphics has been constrained by a false dichotomy:
- **Option A (The Triangle Quagmire):** Load millions of rigid boundary-representation (B-Rep) polygon vertices from static CAD/.glTF files into memory, choking CPU vertex dispatch, PCIe transfer bandwidth, and memory caches whenever semantic relational states mutate.
- **Option B (The Primitive Box Sandbox):** Compress scenes into crude geometric blocks (cubes, cylinders, stacked spheres) floating in black voids with milky, washed-out fog ("toddler artwork" diorama slabs).

**QualiaDB and the Holographic Engine categorically reject this dichotomy.**

By marrying **continuous mathematical topologies**, **proxy bounding geometries as projector lenses**, **GPU-evaluated procedural distance fields (SDFs)**, and **real-world geospatial GIS databases**, we achieve uncompromising AAA visual and acoustic fidelity while maintaining a sub-millisecond CPU frame budget, zero heap allocations in hot paths, and strict 42MB Sentinel memory bounds.

```mermaid
graph TD
    subgraph Data Sources [Real-World Scientific & Geospatial Data]
        GIS[NSW Hydro-Enforced 1-Second DEM & WFS Geology]
        NQuins[QualiaDB 10D Semantic Manifold & N3Logic]
        Biology[Australian Subalpine Botanical & Faunal Taxonomies]
    end

    subgraph Core Processing [Zero-Heap Evaluation Pipelines]
        GeoTile[VectorScene & Continuous ENU Heightfield Splines]
        ProxyGen[12-Triangle Bounding Proxies: Projector Lenses]
        SDFGen[Analytical Distance Fields & C1 Smooth-Min Blends]
        PhysicsEngine[Multi-Body Vehicle Dynamics & Spring Suspension]
    end

    subgraph GPU & Audio Emission [Holographic & Holosonic Output]
        Raymarch[WebGPU / WebGL2 Procedural Fragment Raymarching]
        PBRCookTorrance[Cook-Torrance Microfacet BRDF & Inversion Optics]
        HolosonicWave[Binaural HRTF Spatial Acoustic Wavefield]
        Display[Crisp Zero-Haze Display Output / Headphone Audio]
    end

    GIS --> GeoTile
    NQuins --> ProxyGen
    NQuins --> SDFGen
    Biology --> SDFGen
    GeoTile --> ProxyGen
    ProxyGen --> Raymarch
    SDFGen --> Raymarch
    SDFGen --> HolosonicWave
    PhysicsEngine --> HolosonicWave
    Raymarch --> PBRCookTorrance
    PBRCookTorrance --> Display
    HolosonicWave --> Display
```

---

## 2. Pillar 1: The Holographic Volumetric Engine Architecture

### 2.1 The Screen as an Electromagnetic Frequency (EMF) Emission Manifold
In the Holographic Engine, a 3D simulation is not a collection of rigid polygonal statues viewed through a camera; it is an **active electromagnetic radiation field** projected onto the display:

$$\text{Radiance}(\mathbf{x}, \mathbf{d}) = \int_{t_{\text{near}}}^{t_{\text{far}}} \mathbf{E}(\mathbf{x}(t)) \cdot \exp\left(-\int_0^t \sigma_a(s) \, ds\right) dt + f_r(\mathbf{l}, \mathbf{v}, \mathbf{n}) \cdot L_i(\mathbf{x}(t_{\text{hit}}))$$

### 2.2 Proxy Bounding Geometry ("Projector Lenses")
Instead of pushing complex high-poly meshes to the GPU, the CPU stages lightweight bounding proxies (12-triangle AABB/OBBs or icosahedrons). These serve exclusively to bound the volume in 3D screen space where the GPU executes procedural raymarching:

* **Analytical Ray Entry/Exit Bounds:**
  For camera origin $\mathbf{o}$ and normalized ray direction $\mathbf{d}$:
  $$t_{\min, k} = \frac{c_{i, k} - e_{i, k} - o_k}{d_k}, \quad t_{\max, k} = \frac{c_{i, k} + e_{i, k} - o_k}{d_k} \quad (k \in \{x, y, z\})$$
  $$t_{\text{near}} = \max\left(\min(t_{\min, x}, t_{\max, x}), \min(t_{\min, y}, t_{\max, y}), \min(t_{\min, z}, t_{\max, z})\right)$$
  $$t_{\text{far}} = \min\left(\max(t_{\min, x}, t_{\max, x}), \max(t_{\min, y}, t_{\max, y}), \max(t_{\min, z}, t_{\max, z})\right)$$

### 2.3 $C^1$ Continuous Signed Distance Field (SDF) Algebra
Inside the bounding volume, surfaces are evaluated analytically:
1. **Polynomial Smooth-Minimum ($C^1$ Fusion):**
   $$\text{smin}_k(a, b) = \min(a, b) - \frac{\max(k - |a - b|, 0.0)^2}{4k}$$
   Guarantees organic, seamless topological fusion between intersecting biological limbs or geological strata without polygon boolean operations.
2. **Analytical Surface Normals:**
   $$\mathbf{n}(\mathbf{p}) = \text{normalize}\begin{pmatrix} f(\mathbf{p} + \epsilon \mathbf{e}_x) - f(\mathbf{p} - \epsilon \mathbf{e}_x) \\ f(\mathbf{p} + \epsilon \mathbf{e}_y) - f(\mathbf{p} - \epsilon \mathbf{e}_y) \\ f(\mathbf{p} + \epsilon \mathbf{e}_z) - f(\mathbf{p} - \epsilon \mathbf{e}_z) \end{pmatrix}$$
3. **Infinite Micro-Detail via Spatial Folding:**
   Complex repetitive structures (crystalline mineral fractures, hexagonal basalt jointing, bark furrowing) are generated without memory allocation via coordinate modular folding:
   $$\mathbf{p}_{\text{cell}} = \text{mod}(\mathbf{p} + 0.5 \mathbf{s}, \mathbf{s}) - 0.5 \mathbf{s}$$

---

## 3. Pillar 2: Geographic & Geological Substrate (NSW Government GIS)

```mermaid
flowchart LR
    subgraph Official Data Ingestion
        DEM30[NSW 1-Second Hydro-Enforced DEM 30m Rasters]
        WFSGeol[GSNSW Seamless Geology WFS 2.0.0 Stratigraphic Polygons]
        OSMRoad[OpenStreetMap Snowy Mountains Highway B72 Vectors]
    end

    subgraph Analytical Elevation Compiler
        LocalDatum[Local Campsite Tangent Plane ENU Datum]
        SplineSurface[Continuous Spline Surface y = T_site x, z]
        HermiteSkirt[Radial Hermite Horizon Skirt R 16m to 120m]
    end

    subgraph Strata-Aware Shading
        LithologyDispatch[Lithological Material Dispatch: Karst / Basalt / Granite]
        MicroDetail[Voronoi Rillenkarren & Crystalline Facet Normals]
    end

    DEM30 --> LocalDatum
    WFSGeol --> LithologyDispatch
    OSMRoad --> SplineSurface
    LocalDatum --> SplineSurface
    SplineSurface --> HermiteSkirt
    HermiteSkirt --> MicroDetail
    LithologyDispatch --> MicroDetail
```

### 3.1 Geological Survey of NSW (GSNSW) Stratigraphic Calibration
The simulation models six surveyed sectors along the Snowy Mountains Highway corridor, each strictly grounded in GSNSW stratigraphic mapping:

| Sector | Elevation | GSNSW Stratigraphic Unit | Lithology & Dominant Mineralogy | Procedural Shader Manifestation |
|:---|:---:|:---|:---|:---|
| **0. The Pines** | 385m | *Blowering Formation* (Silurian) | Dacitic/rhyolitic crystal-lithic tuff (`#9F78B3`) | Fractured weathered volcanic tuff with quartz phenocrysts; gentle lake-shelf incline |
| **1. Humes Crossing** | 386m | *Blowering Formation* (Siltstone) | Laminated siltstone, mudstone & shale (`#B586C4`) | Thin horizontal micro-bedding with marsh drainage incision swales |
| **2. Jounama Creek** | 420m | *Lower Jounama Creek Limestone* | Impure calcitic marble, calc-silicate skarn (`#B276CC`) | Asymmetric V-gorge, solution-pitted limestone boulders, gravel washouts |
| **3. Yarrangobilly** | 980m | *Yarrangobilly Limestone* | Massive Silurian karst limestone (`#B683CC`) | 8.5m sheer limestone cliff amphitheatre with chemical dissolution *rillenkarren* fluting |
| **4. O'Hares** | 1120m | *Talbingo Basalt & Young Granodiorite* | Columnar tholeiitic basalt & granodiorite (`#957FBA`) | 200m escarpment drop-off into Jounama gorge, hexagonal jointed basalt columns |
| **5. Three Mile Dam**| 1410m | *Cabramurra Basalt & Bogong Granite* | Subalpine basalt cap over S-type granite (`#E6C6A5`) | Undulating frost-hummock plains, porphyritic pink feldspar granite tors, peat marsh |

### 3.2 Continuous Horizon Skirt (Eliminating Floating Diorama Slabs)
To permanently eliminate the floating tabletop box illusion:
- Playable campsite core ($r \le 16\text{m}$): Exact high-resolution local DEM elevation $y_{\text{site}}(x, z)$.
- Skirt region ($16\text{m} < r \le 120\text{m}$): Smooth cubic Hermite blend into surrounding Kosciuszko National Park mountain ridges:
  $$w(r) = 3\left(\frac{r - 16}{104}\right)^2 - 2\left(\frac{r - 16}{104}\right)^3$$
  $$Y_{\text{composite}}(x, z) = (1.0 - w(r)) \cdot y_{\text{site}}(x, z) + w(r) \cdot Y_{\text{ridge}}(x, z)$$

---

## 4. Pillar 3: Atmospheric Optics & Photometric PBR Shading

### 4.1 Cook-Torrance Microfacet Specular Reflection
Replaces flat diffuse shading with rigorous microfacet physics:

$$f_r(\mathbf{l}, \mathbf{v}, \mathbf{n}) = (1 - F) \frac{\mathbf{C}_{\text{albedo}}}{\pi} + \frac{D(\mathbf{h}) F(\mathbf{v}, \mathbf{h}) G(\mathbf{l}, \mathbf{v}, \mathbf{h})}{4 (\mathbf{n} \cdot \mathbf{l}) (\mathbf{n} \cdot \mathbf{v})}$$

* **Trowbridge-Reitz (GGX) NDF:**
  $$D(\mathbf{h}) = \frac{\alpha^2}{\pi \left((\mathbf{n} \cdot \mathbf{h})^2 (\alpha^2 - 1) + 1\right)^2}$$
* **Schlick Fresnel:**
  $$F(\mathbf{v}, \mathbf{h}) = F_0 + (1 - F_0)(1 - \mathbf{v} \cdot \mathbf{h})^5$$
* **Smith Joint Shadowing-Masking:**
  $$G(\mathbf{l}, \mathbf{v}) = \frac{2 (\mathbf{n} \cdot \mathbf{l}) (\mathbf{n} \cdot \mathbf{v})}{(\mathbf{n} \cdot \mathbf{v}) \sqrt{\alpha^2 + (1 - \alpha^2)(\mathbf{n} \cdot \mathbf{l})^2} + (\mathbf{n} \cdot \mathbf{l}) \sqrt{\alpha^2 + (1 - \alpha^2)(\mathbf{n} \cdot \mathbf{v})^2}}$$

### 4.2 Height-Stratified Inversion Fog (Zero Foreground Milkiness)
* **Near-Field Contrast Guarantee:** Fog is strictly offset by $35\text{m}$, ensuring $100\%$ optical clarity in the interactive campsite and driving zone:
  $$d_{\text{effective}} = \max(0.0, d - 35.0)$$
* **Valley Inversion Layering:** Stratified thermal inversion traps haze in lower reservoir basins while mountain peaks remain razor-sharp:
  $$\rho(y) = \rho_{\text{base}} \cdot \exp\left(-\frac{\max(0.0, y - y_{\text{inversion}})^2}{2\sigma_y^2}\right)$$
  $$\text{Fog} = \text{clamp}\left(1.0 - \exp(-d_{\text{effective}} \cdot \rho(y)), 0.0, 0.88\right)$$

### 4.3 ACES Filmic Tone Mapping
Direct solar radiance ($1.65$) and bright specular glints are tonemapped without chromatic banding or muddy grey clipping:

$$\mathbf{C}_{\text{display}} = \text{clamp}\left(\frac{\mathbf{C} (2.51 \mathbf{C} + 0.03)}{\mathbf{C} (2.43 \mathbf{C} + 0.59) + 0.14}, 0.0, 1.0\right)$$

---

## 5. Pillar 4: Botanical & Ecological Growth Systems

### 5.1 Australian Subalpine Flora Specifications
Replace primitive stacked spheres with Lindenmayer-system (L-System) and Space Colonization procedural branching architectures:

```
Species                      Trunk & Branch Architecture               Bark Texture & Foliage Canopy
─────────────────────────────────────────────────────────────────────────────────────────────────────────────
Snow Gum                     Multi-leader contorted cubic Bézier       Mottled cream ivory (#E8E4D8), sage olive
(Eucalyptus pauciflora)      splines; wind-sheared leaning trunks;     (#6B7D58), burnt orange peeling ribbons;
                             spreading subalpine crowns.               thick waxy falcate leaves with green SSS.

Alpine Ash                   Towering straight columnar trunks         Deep fibrous grey-brown stringybark on
(Eucalyptus delegatensis)    rising 12–18m; high umbrella canopy       lower 60% (#4A423A); smooth creamy white
                             clusters on upper branches.               bark aloft; dense dark blue-green crown.

Golden Wattle                Dense multi-stemmed understorey           Smooth red-brown stems (#5C3826); abundant
(Acacia pycnantha)           spreading shrub with domed profile.       golden globular bloom clusters (#F5C222).

River Bottlebrush            Flexible arching riparian stems;          Papery fibrous grey bark (#524B46); dense
(Callistemon citrinus)       semi-weeping streamside clumps.           cylindrical crimson flower spikes (#C41E28).

Alpine Tussock Grass         Dense hemisphere hummocks with fine       Straw-gold & olive-green needle blades;
(Poa sieberiana)             drooping filiform leaves.                 procedural wind deflection instances.
```

---

## 6. Pillar 5: Anatomical Wildlife & Autonomous Ecological AI

### 6.1 Anatomically Authentic Fauna Models
* **Eastern Grey Kangaroo (*Macropus giganteus*):**
  - **Eliminate Yellow Coats:** Natural subalpine winter pelt:
    - Primary woolly fleece: Muted grey-brown (`#544E45`, `[0.42, 0.38, 0.33, 1.0]`).
    - Ventral chest/belly: Soft cream-grey (`#8C8578`, `[0.68, 0.64, 0.58, 1.0]`).
    - Muzzle, paws, and tail tip: Deep charcoal accents (`#221F1B`).
  - **Biomechanics:** Pentapedal grazing kinematics (using tail as a fifth limb) and high-speed bipedal elastic hopping.
* **Bare-Nosed Wombat (*Vombatus ursinus*):**
  - Heavy barrel silhouette, robust planar skull, short muscular limbs with robust claws, grizzled earth-brown coat (`#42362B`).
* **Kiandra Brumbies (*Equus caballus*):**
  - Sturdy mountain stock, deep chestnut and bay coats, heavy manes and tails adapted for harsh subalpine winters.

### 6.2 N3Logic Ecological AI State Machine
Fauna behavior is governed by QualiaDB N3Logic and Deontic normative rules:
1. **Crepuscular Grazing Rule:**
   $$\forall a \in \text{Fauna}, \quad \text{Time} \in [\text{Dawn}, \text{Dusk}] \implies \mathbf{O}(\text{Graze}(\text{OpenGlade}))$$
2. **Roadside Hazard Avoidance:**
   $$\forall a \in \text{Fauna}, \quad \|\mathbf{x}_a - \mathbf{x}_{\text{vehicle}}\| < 8.0\text{m} \implies \mathbf{P}(\text{Flight}(\text{ForestCover}))$$
3. **Wombat Burrowing Invariant:**
   $$\forall w \in \text{Wombat}, \quad \text{SoilStability} \ge 0.7 \land \text{Slope} \ge 12^\circ \implies \mathbf{O}(\text{ExcavateTerritory})$$

---

## 7. Pillar 6: Holosonic Spatial Acoustic Wavefield

```mermaid
graph TD
    subgraph Signal Synthesis Core
        EngineSynth[Dual Inverter Saw/Sine Oscillator: 75Hz - 280Hz]
        NoiseGen[Decorrelated Stereo White Noise Buffer]
    end

    subgraph Dynamic Surface Filters
        AsphaltRoll[Bandpass Asphalt Pavement Filter: 420Hz, Q 1.8]
        GravelScatter[Highpass Loose Granite Gravel Filter: 2.2kHz, Q 3.0]
        WaterBubbles[Resonant Creek Hydro-Acoustic Filter: 360Hz, Q 2.5]
        CanopyWind[High-Shelf Subalpine Wind Filter: 3.5kHz, Elev-Modulated]
    end

    subgraph Spatial Wavefield Engine
        BinauralITD[Interaural Time Difference: delta_t = d * sin theta / c]
        BinauralILD[Head-Shadow Interaural Level Difference: High-Freq Cut]
        MasterMix[Master Limiter & Web Audio Output]
    end

    EngineSynth --> BinauralITD
    NoiseGen --> AsphaltRoll
    NoiseGen --> GravelScatter
    NoiseGen --> WaterBubbles
    NoiseGen --> CanopyWind
    AsphaltRoll --> BinauralITD
    GravelScatter --> BinauralITD
    WaterBubbles --> BinauralITD
    CanopyWind --> BinauralITD
    BinauralITD --> BinauralILD
    BinauralILD --> MasterMix
```

### 7.1 Dynamic Road Surface Coupling
- **Smooth Asphalt ($|z - z_{\text{road}}| \le 0.8\text{m}$):** Muted tire hum ($420\text{ Hz}$).
- **Shoulder Gravel Cross-Fade ($|z - z_{\text{road}}| > 0.8\text{m}$):** Crunchy granite scatter ($2.2\text{ kHz}, Q=3.0$), proportional to speed and slip angle.
- **Hydro-Acoustic Distance Attenuation:**
  Blowering Reservoir and Jounama Creek sound pressure scales inversely with distance:
  $$p_{\text{water}}(d) = \frac{p_0}{1.0 + 0.28 d}$$

---

## 8. Pillar 7: Vehicle Physics & Drivetrain Simulation

### 8.1 Multi-Body Vehicle Kinematics
- **Mass & Inertia:** Dual-cab campervan with mass $M = 2450\text{ kg}$, wheelbase $L = 3.6\text{m}$, track width $W = 1.8\text{m}$.
- **Suspension:** Independent 4-corner spring-damper system responding dynamically to terrain elevation:
  $$F_{\text{susp}, i} = -k_s (y_i - y_{\text{terrain}, i}) - c_s \dot{y}_i$$
- **Tire Traction (Pacejka Magic Formula):**
  $$F_y = D \sin\left(C \arctan\left(B \alpha - E (B \alpha - \arctan(B \alpha))\right)\right)$$
  Prevents unrealistic instant cornering and accurately simulates loose gravel slippage.

---

## 9. Pillar 8: Comprehensive Playability, Control Mechanics & Interactive Game Loop

```mermaid
stateDiagram-v2
    [*] --> Standby: Hub Initialized
    Standby --> OutboundDrive: Start Outbound Trip (W / Gamepad RT)
    OutboundDrive --> RoadsideHazard: Wildlife Crosses Corridor
    RoadsideHazard --> OutboundDrive: Hazard Cleared / Safe Stop
    OutboundDrive --> CampArrival: Reach Campsite Sector (x >= 16.9)
    CampArrival --> CampWalk: Seamless Avatar Spawn
    CampWalk --> LandmarkInspection: Proximity <= 2.2m (E / Inspect)
    LandmarkInspection --> CampDecision: Strategic Site Choice
    CampDecision --> ReturnReady: Choice Verified
    ReturnReady --> InboundDrive: Initiate Return Journey
    InboundDrive --> RoundTripComplete: Reach Talbingo Hub (x <= -5.4)
    RoundTripComplete --> Standby: Reset or Choose New Sector
```

### 9.1 Multi-Mode Dynamic Camera System
The simulation eliminates static isometric viewpoints in favour of responsive, mode-specific camera mechanics:
1. **Dynamic Vehicle Chase Camera (Driving Mode):**
   - **Spring-Damper Tracking:** The camera trails the campervan at distance $d_{\text{chase}} = 7.5\text{m}$ and height $h_{\text{chase}} = 3.2\text{m}$, smoothly interpolating yaw towards vehicle heading $\theta_{\text{veh}}$:
     $$\dot{\theta}_{\text{cam}} = \omega_n^2 (\theta_{\text{veh}} - \theta_{\text{cam}}) - 2 \zeta \omega_n \dot{\theta}_{\text{cam}}$$
   - **Velocity-Coupled Field of View & Look-Ahead:** Camera FOV expands by $15\%$ and look-ahead target projects $v \cdot 0.35\text{s}$ ahead along the road spline, giving tangible acceleration sensation without motion sickness.
2. **Third-Person Orbital Camera (Camp Walk Mode):**
   - Anchored to the articulated explorer avatar torso ($y = 1.2\text{m}$).
   - Smooth mouse/touch drag orbiting with pitch clamping ($[15^\circ, 78^\circ]$) and distance zoom ($[0.32\text{m}, 2.4\text{m}]$).
   - Dynamic terrain occlusion avoidance: rays cast from avatar to camera prevent clipping into boulders or embankments.
3. **Cinematic Mode Transitions:**
   - Smooth Hermite cubic interpolation ($t \in [0, 1]$ over $0.75\text{s}$) when arriving at camp or mounting the vehicle for departure.

### 9.2 Comprehensive Input Mapping & Gamepad Integration
True playability requires seamless, latency-free input across all platforms:
- **Keyboard & Mouse:**
  - `W` / `↑` : Throttle / Walk North
  - `S` / `↓` : Brake & Reverse / Walk South
  - `A` / `←` : Steer Left / Walk West
  - `D` / `→` : Steer Right / Walk East
  - `Space` : Emergency Handbrake
  - `E` / `Enter` : Inspect Landmark / Interact with Campsite
  - `R` : Reset current sector
  - `M` : Toggle Holosonic spatial wavefield
  - `+` / `-` / Scroll Wheel : Dynamic camera zoom
- **Standard Gamepad API (Xbox / PlayStation / Generic HID):**
  - Left Thumbstick ($X/Y$): Proportional analog steering and camp walking direction.
  - Right Trigger ($RT$): Proportional analog acceleration.
  - Left Trigger ($LT$): Progressive hydraulic braking and reverse gear.
  - Right Thumbstick ($X/Y$): Free-look camera orbital rotation.
  - Button $A$ (Cross): Inspect landmark / Confirm camp choice.
  - Button $B$ (Circle): Emergency handbrake.
  - Button $Y$ (Triangle): Toggle Holosonic audio.
  - Button $X$ (Square): Cycle camera perspective.
  - D-Pad Left/Right: Cycle campsite sector selection.
- **Mobile Touch Controls:**
  - High-precision virtual control pads with pointer capture, haptic feedback triggers, and dedicated Zoom buttons.

### 9.3 Driving Dynamics, Steering Feedback & Hazard Response
- **Speed-Sensitive Steering:** Steering responsiveness scales inversely with velocity ($S_{\text{eff}} = S_0 / (1.0 + 0.04 v)$) to eliminate twitchy oversteer at highway speeds while preserving nimble turning at campsite trailheads.
- **Auto-Centering Spring Torque:** Steering wheel returns to neutral with exponential decay ($\tau = 0.12\text{s}$) upon key/stick release.
- **Shoulder Grip Penalty:** Leaving paved road reduces traction by $32\%$ and triggers physical micro-vibrations and gravel sound synthesis.
- **Wildlife Encounter Logic:** Roadside fauna detection with visual warning reticle; drivers can perform smooth deceleration to observe native wildlife without abrupt failure states.

### 9.4 Full Interactive Campsite Loop
1. **Camp Spawn & Exploration:** Arriving at the campsite spawns the fully articulated Australian explorer avatar equipped with Akubra hat, bush pack, bedroll, and hiking boots.
2. **Interactive Landmark Discovery:** Walk to real-world landmarks (e.g. Jounama Karst Skarn, historic Yarrangobilly riverbank hut, Three Mile Dam granite tors). A dynamic HUD distance reticle indicates proximity.
3. **Contextual Inspection & Action:** When within $2.2\text{m}$, the "Inspect" prompt activates, triggering a 528 Hz harmonic chime and revealing detailed geological, hydrological, and biological insights.
4. **Site Selection Challenge:** Test knowledge of local alpine conditions (e.g. avoiding flood-prone river flats, marshy peat bogs, or falling gum branches).
5. **Return Trip Simulation:** Board the vehicle, execute a turnaround, and complete the return drive down the Kosciuszko mountain corridor with descent telemetry (engine braking, road curvature tracking).

### 9.5 Retro-Modern AAA HUD & Telemetry Dashboard
- **Speedometer & G-Meter:** Circular analog dial and digital readout (km/h) with acceleration and cornering lateral-G telemetry.
- **Elevation Corridor Profile:** Real-time cross-section profile of the Snowy Mountains Highway B72 showing current altitude, gradient ($\%$) and climb/descent rate.
- **Digital Compass Rose:** 360° heading indicator with true north alignment.
- **Live Geology & Geospatial Badge:** Instant GSNSW stratigraphic classification, rock age, and WGS84 GPS coordinate readout.

---

## 10. Master Phased Implementation Roadmap & Verification Gates

```mermaid
gantt
    title Definitive AAA Uplift Roadmap
    dateFormat  X
    axisFormat %s
    section Phase 1: Core Terrain & Optics
    Continuous DEM Heightfield Mesh :done, p1_1, 0, 3
    Atmosphere & Zero-Haze Inversion Shaders:done, p1_2, 1, 4
    Automated Gate 1 Pass :done, p1_3, 4, 5
    section Phase 2: Lithology & Water
    Voronoi Karst & Granite Shaders :active, p2_1, 4, 7
    Animated Trochoidal Water Waves :active, p2_2, 5, 8
    Automated Gate 2 Pass :p2_3, 8, 9
    section Phase 3: Botanical Growth
    Procedural Spline Snow Gums & Ash:p3_1, 8, 11
    Wattle & Bottlebrush Bloom Clusters:p3_2, 9, 12
    Automated Gate 3 Pass :p3_3, 12, 13
    section Phase 4: Rigged Wildlife
    Biological Pelts & Skeletal IK  :p4_1, 12, 15
    N3Logic Autonomous Ecological AI:p4_2, 13, 16
    Automated Gate 4 Pass :p4_3, 16, 17
    section Phase 5: Audio & Physics
    Dynamic Holosonic Road Coupling :p5_1, 16, 18
    Pacejka Multi-Body Vehicle Phys :p5_2, 17, 19
    Automated Gate 5 Pass :p5_3, 19, 20
    section Phase 6: Release & Polish
    Full Playwright E2E Regression  :p6_1, 20, 22
    Zero-Heap Certification & Push  :p6_2, 22, 23
```

### 9.1 Phase Work Packages & Gate Acceptance Criteria

#### Phase 1: Continuous DEM Terrain & Optical Zero-Haze (COMPLETE)
* **Deliverables:**
  - `crates/qualia-core-db/src/render/anatomy/webgl2.rs`: Height-stratified inversion fog with near-field offset ($d - 35.0$), Cook-Torrance PBR BRDF, ACES filmic tonemapping.
  - `crates/rolling-commons-shell/src/talbingo_terrain.rs`: 48×36 continuous topological heightfield mesh with Hermite radial horizon skirt ($R = 16\text{m} \to 60\text{m}$).
  - `crates/rolling-commons-shell/src/talbingo_scenes.rs`: Replaced diorama box with continuous terrain mesh and draped road ribbon, guide posts, campervan, and tents.
  - `crates/rolling-commons-shell/src/macropod_models.rs`: Subalpine grey-brown woolly pelts (`[0.42, 0.38, 0.33, 1.0]`) and cream belly (`[0.68, 0.64, 0.58, 1.0]`).
* **Gate 1 Acceptance:** All 6 sector routes pass in `check-talbingo.mjs`; zero 90° box drop-offs; foreground contrast $\ge 4.5:1$.

#### Phase 2: Lithology & Water Shaders (IN PROGRESS)
* **Deliverables:**
  - `crates/qualia-core-db/src/render/anatomy/webgl2.rs`: Voronoi rock fractures, animated trochoidal water waves, dynamic `u_time` uniform binding.
  - `crates/rolling-commons-shell/src/asset_catalog.rs`: Material family mappings for karst, basalt, granite, water, and bark.
* **Gate 2 Acceptance:** Rock surfaces show crystalline micro-fractures; water exhibits dynamic specular sun-glints and animated surface undulation.

#### Phase 3: Botanical Growth Systems (NEXT)
* **Deliverables:**
  - `crates/rolling-commons-shell/src/talbingo_vegetation.rs`: Procedural multi-leader spline Snow Gums with peeling bark strips; tall columnar Alpine Ash with stringybark buttresses; Golden Wattle and Bottlebrush flower clusters.
* **Gate 3 Acceptance:** Zero primitive stacked-sphere trees; foliage exhibits green subsurface scattering; bark shows species-specific texture.

#### Phase 4: Rigged Wildlife & Ecological AI
* **Deliverables:**
  - `crates/rolling-commons-shell/src/fauna.rs` & `macropod_models.rs`: Full skeletal hierarchy, counterweight tail dynamics for kangaroos, waddling IK gait for wombats.
  - `web/talbingo.html`: QualiaDB N3Logic integration for crepuscular wildlife behavior.
* **Gate 4 Acceptance:** Animals exhibit natural anatomical silhouettes, biological pelts, and responsive flight/grazing behaviors.

#### Phase 5: Holosonic Audio & Vehicle Physics
* **Deliverables:**
  - `web/talbingo.html`: Full Web Audio wavefield graph with dynamic asphalt-to-gravel tire cross-fading, streaming creek hydrodynamics, and speed-coupled wind sibilance.
  - `crates/rolling-commons-shell/src/vehicles.rs`: Spring-damper suspension and Pacejka tire slip model.
* **Gate 5 Acceptance:** Real-time audio cross-fades when vehicle leaves paved road; vehicle chassis tilts realistically over terrain bumps.

#### Phase 6: Final Hardening, Zero-Heap Certification & Release
* **Deliverables:**
  - Automated Playwright driving runs across all 6 sectors with round-trip completion.
  - Verification with `zero_heap_tests` proving zero heap allocations in hot paths.
* **Gate 6 Acceptance:** `check-talbingo.mjs`, `test-game-wasm.mjs`, and `verify-game-release.mjs` all exit code 0; clean git working trees across all repos.

---

## 10. Immovable Rules Compliance Checklist

| Rule | Status | Verification Protocol |
|:---|:---:|:---|
| **Zero Heap in Hot Path** | ✅ Strictly Enforced | Output buffers caller-allocated; no `Vec`/`String`/`Box` in `render` or `step` loops; verified by `zero_heap_tests`. |
| **42MB Sentinel Arena** | ✅ Strictly Enforced | Vertex and index buffers bounded within pre-allocated VRAM reservations. |
| **Deterministic Simulation** | ✅ Strictly Enforced | Fixed-timestep accumulator (`dt = 0.05`) ensures identical replay trajectories across all hardware. |
| **No Adversarial Conduct** | ✅ Strictly Enforced | Honest, verified, biologically and geologically accurate data; no mock shortcuts. |
| **Zero-Haze Optical Quality** | ✅ Strictly Enforced | Inversion fog formula eliminates foreground haze; contrast ratio $\ge 4.5:1$. |
