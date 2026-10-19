/**
 * EcoPunk 3D Holographic Earth-Terrain, Biosphere & Multi-View Rendering Engine
 *
 * Connects QualiaDB native substrates (.10d heightfield manifolds, .m64 6-layer
 * SLGA soil strata, 16-KB GPU ConstituentLUT, 100 Flora morphogenesis meshes,
 * and 64 Fauna articulated biomechanical agents) directly to WebGL.
 *
 * Supports:
 * - 6 Camera / Spatial View Modes:
 *     'aerial_orbit', 'ground_walk', 'fauna_tracker',
 *     'specimen_loupe', 'subsurface_cutaway', 'corridor_gis'
 * - 6 Normative Observer Shader Modes:
 *     0 = Spectral PBR (Cook-Torrance GGX + ACES Filmic + Zero-Haze Near-Field)
 *     1 = Cel Illustration (Botanical Art-Bible Stepped Shading + Edge Inking)
 *     2 = Namatjira Watercolor (Kubelka-Munk Pigment Wash + Paper Granulation)
 *     3 = Tactical GPR & Trophic Network HUD (Dielectric Grid + 3D Food-Web Arcs)
 *     4 = Planck Thermal IR & NDVI (Endotherm BMR Glow + Ectotherm / Transpiration)
 *     5 = Heliophysics & Bioacoustic Wavefield (Magnetosphere + Sonic Rings)
 */

class TerrainRenderer {
  constructor(canvas) {
    this.canvas = canvas;
    this.gl = null;
    this.uintExt = null;

    this.floraEngine = new window.FloraEngine();
    this.faunaEngine = new window.FaunaEngine();
    this.getHeightAt = (x, z) => this.sampleElevation(x, z);

    // Active Rendering & View Modes
    this.observerMode = 0;
    this.viewMode = 'aerial_orbit'; // aerial_orbit | ground_walk | fauna_tracker | specimen_loupe | subsurface_cutaway | corridor_gis
    this.showWireframeOverlay = true;
    this.showTrophicArcs = true;
    this.clipPlaneZ = 999.0; // Active during subsurface_cutaway

    // Environmental & Heliophysical State
    this.time = 0;
    this.timeOfDayHours = 10.5; // 0..24h
    this.windSpeedMs = 6.5;     // Beaufort wind speed (m/s)
    this.windDirRad = 0.6;
    this.kpIndex = 3.2;         // NOAA SWPC Planetary Kp (0..9)
    this.imfBz = -2.4;          // Interplanetary Magnetic Field Bz (nT)
    this.specimenGrowthScale = 1.0;

    // Terrain Grid Resolution (128x128 for crisp canyons, cliffs, and river channels)
    this.gridSize = 120;
    this.worldSize = 120.0;

    // Camera State
    this.camera = {
      rotX: 0.48,
      rotY: 0.65,
      dist: 62.0,
      target: [0, 6.0, 0],
      eye: [30, 32, 40],
      velocity: [0, 0, 0],
      lastPos: [0, 0, 0],
      walkPos: [8.0, 0.0, 18.0],
      walkYaw: -2.6,
      walkPitch: -0.08
    };

    this.keysPressed = {};
    this.isDragging = false;
    this.lastMouse = { x: 0, y: 0 };
    this.excavationMode = false;
    this.excavations = [];

    // Loaded Sector, Flora (100), and Fauna (64) Catalogs
    this.currentSector = null;
    this.binaryDem10d = null;
    this.constituentLut = null;
    this.floraCatalog = [];
    this.faunaCatalog = [];

    // Instanced Scene Entities
    this.floraInstances = [];
    this.faunaAgents = [];
    this.trackedFaunaIndex = 0;
    this.loupeTarget = { type: 'flora', species: null };

    this.activeIsogloss = null;
    this.sonicRipples = []; // { x, y, z, radius, maxRadius, color }

    // Primary Hardware Pipeline: WebGPU (navigator.gpu) + Naga-Certified WGSL Forge
    this.usingWebGPU = false;
    this.webgpuEngine = window.WebGpuForgeEngine ? new window.WebGpuForgeEngine() : null;
    if (this.webgpuEngine) {
      this.webgpuInitPromise = this.webgpuEngine.init(canvas).then((ok) => {
        this.usingWebGPU = Boolean(ok);
        if (!ok) {
          this.initWebGLFallback();
        }
        const badge = document.getElementById('gpu-backend-badge');
        if (badge) {
          badge.textContent = ok
            ? `WEBGPU · WGSL FORGE v${window.ECOPUNK_WGSL_FORGE?.forge_schema_version || 2} (NAGA)`
            : 'WEBGL · WGSL FORGE PARITY';
        }
        return ok;
      });
    } else {
      this.initWebGLFallback();
      this.webgpuInitPromise = Promise.resolve(false);
    }

    this.buildSkyAndWaterMeshes();
    this.buildTerrainMesh();
    this.setupInteractions();
    this.resize();
  }

  initWebGLFallback() {
    this.gl = this.canvas.getContext('webgl', { antialias: true, alpha: false, preserveDrawingBuffer: true }) ||
              this.canvas.getContext('experimental-webgl', { preserveDrawingBuffer: true });
    if (!this.gl) return;
    this.uintExt = this.gl.getExtension('OES_element_index_uint');
    this.initShaders();
    this.buildSkyAndWaterMeshes();
    this.buildTerrainMesh();
  }

  // ============================================================================
  // SHADER PIPELINE (Terrain/Strata, Flora Aeroelastic, Fauna Kinematic, Wire/Water)
  // ============================================================================

  initShaders() {
    const gl = this.gl;

    // 1. Unified Entity & Terrain Shader (supports Terrain, Strata, Flora wind, Fauna gait)
    const vsMain = `
      precision highp float;
      precision highp int;

      attribute vec3 aPosition;
      attribute vec3 aNormal;
      attribute vec3 aColor;
      attribute vec4 aMeta; // Terrain: [elevNorm, stratumIdx, isStrataWall, moisture]
                            // Flora:   [heightNorm, branchFlex, flutterWeight, partType]
                            // Fauna:   [partId, sideSign, distAlong, thermalNorm]

      uniform mat4 uProjection;
      uniform mat4 uView;
      uniform mat4 uModel;
      uniform int uEntityKind; // 0=Terrain, 1=Flora, 2=Fauna, 3=Water, 4=Sky
      uniform float uTime;
      uniform vec2 uWindVec;   // [windX, windZ] in m/s
      uniform vec4 uFaunaAnim; // [phase, speed, gaitCode, hopOffset]

      varying vec3 vNormal;
      varying vec3 vWorldPos;
      varying vec3 vColor;
      varying vec4 vMeta;

      void main() {
        vec3 pos = aPosition;
        vec3 norm = aNormal;

        // GPU Aeroelastic Wind Deformation for Flora (uEntityKind == 1)
        if (uEntityKind == 1) {
          float hNorm = aMeta.x;
          float branchFlex = aMeta.y;
          float flutter = aMeta.z;
          float windMag = length(uWindVec);

          // Primary low-frequency trunk & branch sway (Bishop-frame bending)
          float swayPhase = uTime * 1.65 + uModel[3][0] * 0.35 + uModel[3][2] * 0.25;
          float swayAmp = (0.045 * windMag) * pow(max(hNorm, 0.0), 1.6) * branchFlex;
          pos.x += uWindVec.x * 0.12 * swayAmp * (0.65 + 0.35 * sin(swayPhase));
          pos.z += uWindVec.y * 0.12 * swayAmp * (0.65 + 0.35 * cos(swayPhase * 0.9));

          // Secondary high-frequency leaf/frond flutter
          if (flutter > 0.2) {
            float flPhase = uTime * (4.5 + windMag * 0.35) + pos.x * 3.1 + pos.y * 2.7;
            float flAmp = 0.025 * min(windMag, 18.0) * flutter;
            pos.x += sin(flPhase) * flAmp;
            pos.y += cos(flPhase * 1.3) * flAmp * 0.6;
            pos.z += cos(flPhase * 0.8) * flAmp;
          }
        }
        // GPU Articulated Biomechanical Kinematics for Fauna (uEntityKind == 2)
        else if (uEntityKind == 2) {
          float partId = aMeta.x;
          float side = aMeta.y;
          float distAlong = aMeta.z;
          float phase = uFaunaAnim.x;
          float speed = uFaunaAnim.y;
          float gait = uFaunaAnim.z;
          float moveFactor = clamp(speed / 3.0, 0.15, 1.4);

          // Head & neck bobbing (partId == 1)
          if (partId > 0.5 && partId < 1.5) {
            pos.y += sin(phase * 2.0) * 0.045 * moveFactor * distAlong;
            pos.z += cos(phase) * 0.03 * moveFactor * distAlong;
          }
          // Forelimbs or Wings (partId == 2)
          else if (partId > 1.5 && partId < 2.5) {
            if (gait > 5.5 && gait < 7.5) {
              // Avian / Bat wing flapping articulation
              float flap = sin(phase * 1.8) * (gait > 6.5 ? 0.55 : 0.18);
              float radial = abs(pos.x);
              pos.y += radial * sin(flap);
            } else {
              // Terrestrial forelimb stride swing
              float swing = sin(phase + side * 1.57) * 0.22 * moveFactor * distAlong;
              pos.z += swing;
              pos.y += max(0.0, cos(phase + side * 1.57)) * 0.08 * moveFactor * distAlong;
            }
          }
          // Hindlimbs (partId == 3)
          else if (partId > 2.5 && partId < 3.5) {
            if (gait < 1.5) {
              // Synchronous bipedal macropod hop kick
              float kick = sin(phase) * 0.28 * moveFactor * distAlong;
              pos.z -= kick;
              pos.y += max(0.0, cos(phase)) * 0.14 * moveFactor * distAlong;
            } else {
              // Alternating quadrupedal / bipedal stride
              float swing = sin(phase - side * 1.57) * 0.25 * moveFactor * distAlong;
              pos.z += swing;
              pos.y += max(0.0, -cos(phase - side * 1.57)) * 0.09 * moveFactor * distAlong;
            }
          }
          // Tail / Body wave (partId == 4)
          else if (partId > 3.5) {
            float wave = sin(phase * 1.5 - distAlong * 3.5) * 0.18 * distAlong;
            if (gait > 8.5 && gait < 11.5) {
              // Sinusoidal snake / fish / lizard lateral undulation
              pos.x += sin(phase * 2.0 - pos.z * 2.5) * 0.16 * (0.3 + 0.7 * distAlong);
            } else {
              pos.x += wave * 0.6;
              pos.y += cos(phase - distAlong * 2.0) * 0.08 * distAlong;
            }
          }
        }
        // Animated Trochoidal Water Waves (uEntityKind == 3)
        else if (uEntityKind == 3) {
          float w1 = sin(pos.x * 0.45 + uTime * 2.2) * cos(pos.z * 0.35 + uTime * 1.7) * 0.14;
          float w2 = sin((pos.x * 0.8 - pos.z * 0.6) + uTime * 3.4) * 0.06;
          pos.y += w1 + w2;
          norm = normalize(vec3(-0.22 * cos(pos.x * 0.45 + uTime * 2.2), 1.0, -0.18 * sin(pos.z * 0.35 + uTime * 1.7)));
        }

        vec4 worldPos = uModel * vec4(pos, 1.0);
        vWorldPos = worldPos.xyz;
        vNormal = normalize((uModel * vec4(norm, 0.0)).xyz);
        vColor = aColor;
        vMeta = aMeta;
        gl_Position = uProjection * uView * worldPos;
      }
    `;

    const fsMain = `
      precision highp float;
      precision highp int;

      varying vec3 vNormal;
      varying vec3 vWorldPos;
      varying vec3 vColor;
      varying vec4 vMeta;

      uniform vec3 uCameraPos;
      uniform vec3 uSunDir;
      uniform int uObserverMode;
      uniform int uEntityKind; // 0=Terrain, 1=Flora, 2=Fauna, 3=Water, 4=Sky
      uniform float uTime;
      uniform float uClipPlaneZ;
      uniform float uWaterLevel;
      uniform float uKarstFluting;
      uniform vec4 uIsoglossParam; // [centerX, radius, centerZ, active]
      uniform vec2 uSpaceWeather;  // [kpIndex, imfBz]

      // ACES Filmic Tone Mapping (Pillar 3 §4.3)
      vec3 acesFilm(vec3 x) {
        float a = 2.51;
        float b = 0.03;
        float c = 2.43;
        float d = 0.59;
        float e = 0.14;
        return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
      }

      // Procedural 2D hash for Voronoi micro-fractures & paper grain
      float hash21(vec2 p) {
        p = fract(p * vec2(234.34, 435.345));
        p += dot(p, p + 34.23);
        return fract(p.x * p.y);
      }

      void main() {
        // Subsurface Cross-Section Clip Plane support
        if (vWorldPos.z > uClipPlaneZ && uEntityKind != 4) {
          discard;
        }

        vec3 n = normalize(vNormal);
        vec3 viewDir = normalize(uCameraPos - vWorldPos);
        vec3 lightDir = normalize(uSunDir);
        float NdotL = max(dot(n, lightDir), 0.0);
        float NdotV = max(dot(n, viewDir), 0.0);

        // Sky Dome Rendering (uEntityKind == 4)
        if (uEntityKind == 4) {
          vec3 rayDir = normalize(vWorldPos - uCameraPos);
          float sunElev = lightDir.y;
          float horizon = clamp(rayDir.y, 0.0, 1.0);
          vec3 dayZenith = vec3(0.11, 0.28, 0.56);
          vec3 dayHorizon = vec3(0.55, 0.74, 0.88);
          vec3 duskHorizon = vec3(0.88, 0.45, 0.24);
          float dayFactor = clamp(sunElev * 1.8, 0.05, 1.0);
          vec3 skyCol = mix(
            mix(duskHorizon * dayFactor, dayHorizon * dayFactor, clamp(sunElev + 0.2, 0.0, 1.0)),
            dayZenith * dayFactor,
            pow(horizon, 0.55)
          );
          // Solar disc & Mie corona
          float sunDot = max(dot(rayDir, lightDir), 0.0);
          skyCol += vec3(1.0, 0.92, 0.72) * pow(sunDot, 180.0) * 1.4;
          skyCol += vec3(0.95, 0.75, 0.45) * pow(sunDot, 12.0) * 0.25;

          // Southern Hemisphere Aurora Australis ribbon (driven by Kp index & Mode 5)
          float kp = uSpaceWeather.x;
          if (kp > 3.5 || uObserverMode == 5) {
            float auroraBand = sin(rayDir.x * 6.0 + uTime * 0.8) * cos(rayDir.z * 5.0 - uTime * 0.5);
            float curtain = smoothstep(0.15, 0.55, rayDir.y) * (1.0 - smoothstep(0.55, 0.88, rayDir.y));
            float intensity = (uObserverMode == 5 ? 0.65 : clamp((kp - 3.5) / 5.5, 0.0, 0.55));
            vec3 auroraGreen = vec3(0.1, 0.95, 0.55);
            vec3 auroraMagenta = vec3(0.85, 0.15, 0.65);
            vec3 auroraCol = mix(auroraGreen, auroraMagenta, clamp(rayDir.y * 1.4, 0.0, 1.0));
            skyCol += auroraCol * curtain * (0.5 + 0.5 * auroraBand) * intensity;
          }

          gl_FragColor = vec4(acesFilm(skyCol), 1.0);
          return;
        }

        vec3 baseCol = vColor;

        // Terrain Micro-Detail: Voronoi rock fractures, rillenkarren karst fluting & Bachmann shoreline wetting
        if (uEntityKind == 0) {
          float isWall = vMeta.z;
          float slope = 1.0 - max(n.y, 0.0);
          if (isWall < 0.5) {
            // Karst rillenkarren vertical dissolution flutes on steep cliffs
            if (slope > 0.35) {
              float flute = sin(vWorldPos.x * 3.8 + vWorldPos.z * 3.8) * 0.5 + 0.5;
              float fracture = hash21(floor(vWorldPos.xz * 2.5));
              baseCol *= (0.82 + 0.24 * mix(fracture, flute, uKarstFluting));
            }
            // Bachmann optical wetting darkening near water shoreline
            float distAboveWater = vWorldPos.y - uWaterLevel;
            if (distAboveWater >= -0.5 && distAboveWater < 1.35) {
              float wetFactor = 1.0 - clamp(distAboveWater / 1.35, 0.0, 1.0);
              baseCol *= (1.0 - 0.38 * wetFactor);
            }
          } else {
            // Subsurface SLGA Stratigraphic Cutaway Wall banding
            float strataGrain = sin(vWorldPos.y * 14.0 + vWorldPos.x * 0.8) * 0.06;
            baseCol += vec3(strataGrain);
          }
        }

        // Water Surface Shading (uEntityKind == 3): Fresnel + Beer-Lambert + Specular Sun Glint
        if (uEntityKind == 3) {
          vec3 halfVec = normalize(lightDir + viewDir);
          float spec = pow(max(dot(n, halfVec), 0.0), 96.0) * 1.35;
          float fresnel = 0.04 + 0.96 * pow(1.0 - NdotV, 4.0);
          vec3 deepWater = baseCol * 0.55;
          vec3 skyRefl = vec3(0.42, 0.65, 0.85) * max(0.25, lightDir.y);
          vec3 waterCol = mix(deepWater * (0.45 + 0.55 * NdotL), skyRefl, fresnel * 0.65) + vec3(1.0, 0.96, 0.85) * spec;
          if (uObserverMode == 4) {
            // Cool thermal water signature in IR mode
            waterCol = vec3(0.05, 0.22, 0.68);
          }
          gl_FragColor = vec4(acesFilm(waterCol), 1.0);
          return;
        }

        vec3 finalCol = baseCol;

        // ====================================================================
        // 6 NORMATIVE OBSERVER PROJECTION KERNELS
        // ====================================================================

        // Mode 0: Photometric Spectral PBR (Cook-Torrance Microfacet + ACES)
        if (uObserverMode == 0) {
          vec3 H = normalize(lightDir + viewDir);
          float NdotH = max(dot(n, H), 0.0);
          float roughness = (uEntityKind == 1) ? 0.55 : (uEntityKind == 2 ? 0.65 : 0.78);
          float alpha = roughness * roughness;
          float alpha2 = alpha * alpha;
          float denom = (NdotH * NdotH * (alpha2 - 1.0) + 1.0);
          float D = alpha2 / (3.14159 * denom * denom + 0.0001);
          float F = 0.04 + 0.96 * pow(1.0 - max(dot(viewDir, H), 0.0), 5.0);
          float spec = (D * F) * 0.25;

          // Hemisphere sky/ground ambient + Subsurface Scattering for foliage
          vec3 skyAmb = mix(vec3(0.18, 0.16, 0.14), vec3(0.26, 0.34, 0.46), 0.5 + 0.5 * n.y);
          vec3 sss = vec3(0.0);
          if (uEntityKind == 1 && vMeta.w > 1.5) {
            float backLight = max(dot(viewDir, -lightDir), 0.0);
            sss = baseCol * vec3(0.35, 0.55, 0.18) * pow(backLight, 2.0) * 0.45;
          }

          vec3 sunRadiance = vec3(1.55, 1.46, 1.32) * max(0.15, lightDir.y);
          vec3 lit = baseCol * (skyAmb + sunRadiance * NdotL * 0.78) + sunRadiance * spec + sss;

          // Height-Stratified Valley Inversion Fog with strict 35m Near-Field Clarity Offset (Pillar 3 §4.2)
          float dist = length(vWorldPos - uCameraPos);
          float dEff = max(0.0, dist - 38.0);
          float heightFactor = exp(-max(0.0, vWorldPos.y - uWaterLevel) * 0.08);
          float fogFactor = clamp(1.0 - exp(-dEff * 0.0035 * (0.4 + 0.6 * heightFactor)), 0.0, 0.65);
          vec3 fogCol = vec3(0.58, 0.72, 0.84) * max(0.25, lightDir.y);
          finalCol = acesFilm(mix(lit, fogCol, fogFactor));
        }
        // Mode 1: Cel-Shaded Botanical Illustration (Toon + Edge Inking)
        else if (uObserverMode == 1) {
          float cel = 0.38;
          if (NdotL > 0.68) cel = 1.12;
          else if (NdotL > 0.32) cel = 0.78;
          float rim = smoothstep(0.18, 0.28, NdotV);
          finalCol = baseCol * cel * (0.15 + 0.85 * rim);
        }
        // Mode 2: Namatjira Storybook Watercolor (Kubelka-Munk Wash + Paper Grain)
        else if (uObserverMode == 2) {
          float grain = sin(vWorldPos.x * 4.5) * cos(vWorldPos.z * 4.5) * 0.07;
          float bleed = smoothstep(0.08, 0.92, NdotL + grain);
          vec3 warmPaper = vec3(0.98, 0.95, 0.88);
          vec3 pigment = mix(baseCol * 0.68, min(vec3(1.0), baseCol * 1.28 + vec3(0.06, 0.03, 0.0)), bleed);
          float edgeGranulation = 1.0 - 0.32 * pow(1.0 - NdotV, 2.2);
          finalCol = mix(warmPaper, pigment * edgeGranulation, 0.88);
        }
        // Mode 3: Tactical GPR / Subsurface LiDAR & Trophic Network HUD
        else if (uObserverMode == 3) {
          float grid = step(0.93, fract(vWorldPos.x * 0.5)) + step(0.93, fract(vWorldPos.z * 0.5));
          float contour = step(0.92, fract(vWorldPos.y * 0.8));
          float pulse = sin(vWorldPos.y * 6.0 - uTime * 5.0) * 0.5 + 0.5;
          vec3 backing = vec3(0.02, 0.06, 0.11);
          if (uEntityKind == 2) {
            // Highlight fauna in glowing magenta-amber on tactical radar
            finalCol = mix(vec3(0.95, 0.2, 0.6), vec3(1.0, 0.85, 0.2), NdotL);
          } else if (uEntityKind == 1) {
            finalCol = mix(backing, vec3(0.1, 0.92, 0.55), 0.35 + 0.45 * NdotL + grid * 0.3);
          } else {
            finalCol = mix(backing, vec3(0.0, 0.88, 1.0), clamp(grid * 0.65 + contour * 0.45 + pulse * 0.15, 0.0, 1.0));
          }
        }
        // Mode 4: Planck Blackbody Thermal IR & Multispectral NDVI
        else if (uObserverMode == 4) {
          float heat = 0.35;
          if (uEntityKind == 2) {
            // Fauna thermal signature from AnAge BMR & endotherm body temperature
            heat = clamp(vMeta.w + 0.15 * NdotL, 0.0, 1.0);
          } else if (uEntityKind == 1) {
            // Foliar transpiration cooling vs sunlit crown
            heat = 0.28 + 0.24 * NdotL;
          } else {
            heat = clamp(0.18 + NdotL * 0.42 + (vWorldPos.y / 30.0) * 0.15, 0.0, 0.85);
          }
          vec3 cCold = vec3(0.04, 0.08, 0.52);
          vec3 cMid  = vec3(0.62, 0.05, 0.78);
          vec3 cWarm = vec3(0.98, 0.48, 0.04);
          vec3 cHot  = vec3(1.0, 0.98, 0.86);
          if (heat < 0.33) finalCol = mix(cCold, cMid, heat * 3.0);
          else if (heat < 0.66) finalCol = mix(cMid, cWarm, (heat - 0.33) * 3.0);
          else finalCol = mix(cWarm, cHot, (heat - 0.66) * 3.0);
        }
        // Mode 5: Heliophysics & Bioacoustic Wavefield Projection
        else if (uObserverMode == 5) {
          float magField = abs(sin(vWorldPos.x * 0.25 + vWorldPos.z * 0.15 + uTime * 0.8));
          vec3 bioCol = baseCol * (0.45 + 0.65 * NdotL);
          vec3 waveTint = mix(vec3(0.0, 0.95, 0.75), vec3(0.65, 0.25, 1.0), magField);
          finalCol = mix(bioCol, waveTint, 0.28);
        }

        // Diachronic Isogloss & Sonic Toponym Wave Ring Overlay
        if (uIsoglossParam.w > 0.5 && uEntityKind == 0) {
          float distIso = length(vWorldPos.xz - vec2(uIsoglossParam.x, uIsoglossParam.z));
          float ringDist = abs(distIso - uIsoglossParam.y);
          if (ringDist < 0.55) {
            float glow = 1.0 - (ringDist / 0.55);
            finalCol = mix(finalCol, vec3(0.0, 0.96, 0.82), glow * 0.75);
          }
        }

        gl_FragColor = vec4(finalCol, 1.0);
      }
    `;

    // 2. Simple Line / Wireframe Shader (Osteological Skeleton, Vascular Roots, Trophic Arcs, Sonic Rings)
    const vsLine = `
      attribute vec3 aPosition;
      uniform mat4 uProjection;
      uniform mat4 uView;
      uniform mat4 uModel;
      void main() {
        gl_Position = uProjection * uView * uModel * vec4(aPosition, 1.0);
      }
    `;
    const fsLine = `
      precision mediump float;
      uniform vec4 uLineColor;
      void main() {
        gl_FragColor = uLineColor;
      }
    `;

    this.program = this.createProgram(vsMain, fsMain);
    this.lineProgram = this.createProgram(vsLine, fsLine);

    this.attribs = {
      position: gl.getAttribLocation(this.program, 'aPosition'),
      normal: gl.getAttribLocation(this.program, 'aNormal'),
      color: gl.getAttribLocation(this.program, 'aColor'),
      meta: gl.getAttribLocation(this.program, 'aMeta')
    };

    this.uniforms = {
      projection: gl.getUniformLocation(this.program, 'uProjection'),
      view: gl.getUniformLocation(this.program, 'uView'),
      model: gl.getUniformLocation(this.program, 'uModel'),
      cameraPos: gl.getUniformLocation(this.program, 'uCameraPos'),
      sunDir: gl.getUniformLocation(this.program, 'uSunDir'),
      observerMode: gl.getUniformLocation(this.program, 'uObserverMode'),
      entityKind: gl.getUniformLocation(this.program, 'uEntityKind'),
      time: gl.getUniformLocation(this.program, 'uTime'),
      windVec: gl.getUniformLocation(this.program, 'uWindVec'),
      faunaAnim: gl.getUniformLocation(this.program, 'uFaunaAnim'),
      clipPlaneZ: gl.getUniformLocation(this.program, 'uClipPlaneZ'),
      waterLevel: gl.getUniformLocation(this.program, 'uWaterLevel'),
      karstFluting: gl.getUniformLocation(this.program, 'uKarstFluting'),
      isoglossParam: gl.getUniformLocation(this.program, 'uIsoglossParam'),
      spaceWeather: gl.getUniformLocation(this.program, 'uSpaceWeather')
    };

    this.lineAttribs = {
      position: gl.getAttribLocation(this.lineProgram, 'aPosition')
    };
    this.lineUniforms = {
      projection: gl.getUniformLocation(this.lineProgram, 'uProjection'),
      view: gl.getUniformLocation(this.lineProgram, 'uView'),
      model: gl.getUniformLocation(this.lineProgram, 'uModel'),
      lineColor: gl.getUniformLocation(this.lineProgram, 'uLineColor')
    };

    this.dynamicLineBuffer = gl.createBuffer();
  }

  createProgram(vsSource, fsSource) {
    const gl = this.gl;
    const vs = this.compileShader(gl.VERTEX_SHADER, vsSource);
    const fs = this.compileShader(gl.FRAGMENT_SHADER, fsSource);
    const prog = gl.createProgram();
    gl.attachShader(prog, vs);
    gl.attachShader(prog, fs);
    gl.linkProgram(prog);
    if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
      console.error('Program link error:', gl.getProgramInfoLog(prog));
    }
    return prog;
  }

  compileShader(type, source) {
    const gl = this.gl;
    const shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      console.error('Shader compile error:', gl.getShaderInfoLog(shader));
    }
    return shader;
  }

  // ============================================================================
  // GEOSPATIAL & ECOLOGICAL SUBSTRATE LOADING
  // ============================================================================

  /**
   * Loads compiled .10d binary heightfield & 16-KB ConstituentLUT if served over HTTP
   */
  async loadBinarySubstrates() {
    try {
      const resp10d = await fetch('../assets/terrain/talbingo_heightfield.10d');
      if (resp10d.ok) {
        const buf = await resp10d.arrayBuffer();
        if (buf.byteLength >= 96 + 64 * 64 * 4) {
          this.binaryDem10d = new Float32Array(buf, 96, 64 * 64);
        }
      }
      const respLut = await fetch('../assets/terrain/constituent_lut_16kb.bin');
      if (respLut.ok) {
        this.constituentLut = new Uint8Array(await respLut.arrayBuffer());
      }
    } catch (e) {
      // Fallback to in-memory sector DEM synthesis when opened via file://
    }
  }

  /**
   * Configures the active sector, rebuilds the 3D terrain + subsurface strata skirt,
   * and populates all sector-matching Flora (from 100 species) and Fauna (from 64 species).
   */
  loadSectorAndEcosystem(sector, floraList, faunaList) {
    this.currentSector = sector;
    this.floraCatalog = floraList || [];
    this.faunaCatalog = faunaList || [];

    if (sector?.climate_ozflux) {
      this.windSpeedMs = sector.climate_ozflux.wind_speed_ms || 6.5;
      this.windDirRad = ((sector.climate_ozflux.wind_dir_deg || 240) * Math.PI) / 180;
    }

    this.excavations = [];
    this.buildTerrainMesh();
    this.buildSkyAndWaterMeshes();
    this.populateEcosystem();
  }

  // ============================================================================
  // REALISTIC TOPOGRAPHIC HEIGHTFIELD, RIVER INCISION & 6-LAYER SLGA SKIRT
  // ============================================================================

  /**
   * Computes realistic world-space elevation Y at (x, z) in [-60, 60] for the active sector
   */
  sampleElevation(x, z) {
    const sec = this.currentSector;
    const secId = sec?.id || 'talbingo_pines';
    const nx = x / 60.0; // -1..1
    const nz = z / 60.0; // -1..1

    let h = 0.0;

    if (secId === 'talbingo_pines') {
      // Deep Tumut River / Blowering Reservoir canyon along river_spline + Mount Talbingo escarpment on East (x > 0)
      const riverCenterX = -4.5 + Math.sin(nz * 2.2) * 3.5;
      const distRiver = Math.abs(x - riverCenterX);
      const canyon = Math.tanh(distRiver / 16.0) * 14.0 - 3.2;
      const eastEscarpment = Math.max(0, nx + 0.15) * 16.5;
      const spurs = Math.sin(x * 0.14) * Math.cos(z * 0.16) * 2.8 + Math.sin(x * 0.32 - z * 0.28) * 1.1;
      h = canyon + eastEscarpment + spurs;
    } else if (secId === 'jounama_karst') {
      // Asymmetric limestone V-gorge + karst boulder benches
      const riverX = 2.0 * Math.sin(nz * 2.6) - 2.0;
      const dR = Math.abs(x - riverX);
      const gorge = Math.pow(Math.min(1.0, dR / 28.0), 1.35) * 18.0 - 2.4;
      const karstSteps = Math.floor((gorge + 4.0) * 0.6) * 0.55 + Math.sin(x * 0.25) * Math.cos(z * 0.22) * 2.2;
      h = gorge * 0.75 + karstSteps * 0.45;
    } else if (secId === 'yarrangobilly_cliff') {
      // Dramatic sheer vertical Silurian limestone cliff amphitheatre + deep thermal pool gorge
      const riverX = -8.0 + Math.cos(nz * 1.8) * 4.0;
      const dR = x - riverX;
      let cliff = 0.0;
      if (dR > 6.0) {
        // Steep limestone cliff wall rising on the east side
        cliff = 14.5 / (1.0 + Math.exp(-(dR - 14.0) * 0.45)) + Math.sin(z * 0.25) * 1.8;
      } else if (dR < -6.0) {
        cliff = Math.min(12.0, Math.abs(dR + 6.0) * 0.42);
      } else {
        cliff = -2.6 + (dR * dR) * 0.06;
      }
      const microKarst = Math.sin(x * 0.4) * Math.cos(z * 0.35) * 0.9;
      h = cliff + microKarst;
    } else if (secId === 'three_mile_dam') {
      // High subalpine cold-air drainage frost hollow basin surrounded by granite tor ridges
      const rBasin = Math.hypot(nx * 1.1, nz * 0.95);
      const frostHollow = Math.pow(Math.max(0, rBasin - 0.22), 1.6) * 16.0 - 1.8;
      const tors = Math.pow(Math.max(0, Math.sin(x * 0.22) * Math.cos(z * 0.24)), 3.0) * 4.5;
      h = frostHollow + tors;
    } else if (secId === 'sydney_sandstone') {
      // Triassic Hawkesbury Sandstone stepped cliff benches & slot canyon
      const dCanyon = Math.abs(x - Math.sin(nz * 2.0) * 5.0);
      const rawSlope = Math.min(1.0, dCanyon / 38.0) * 20.0;
      // Stepped sandstone benches
      const benches = Math.floor(rawSlope / 3.5) * 3.2 + (rawSlope % 3.5) * 0.35;
      h = benches - 2.5 + Math.sin(x * 0.18 + z * 0.15) * 1.2;
    } else if (secId === 'noosa_headland') {
      // Coastal basalt headland on East + parabolic sand dunes + wallum creek estuary on West
      const headland = (nx > 0.1 && nz < 0.2)
        ? Math.exp(-Math.pow((nx - 0.55) * 2.2, 2.0) - Math.pow((nz + 0.25) * 2.0, 2.0)) * 16.0
        : 0.0;
      const dunes = Math.sin(x * 0.16 + 1.2) * Math.cos(z * 0.12) * 2.2;
      const estuary = nx < -0.15 ? -2.2 : 1.2;
      h = headland + dunes + estuary;
    } else {
      h = Math.sin(x * 0.12) * Math.cos(z * 0.12) * 5.0;
    }

    // Blend subtle real binary .10d DEM micro-topography if loaded
    if (this.binaryDem10d) {
      const gx = Math.min(63, Math.max(0, Math.floor(((nx + 1) * 0.5) * 63)));
      const gz = Math.min(63, Math.max(0, Math.floor(((nz + 1) * 0.5) * 63)));
      const demM = this.binaryDem10d[gz * 64 + gx];
      h += ((demM - 650.0) / 400.0) * 1.2;
    }

    // Apply CSG Excavations
    for (const exc of this.excavations) {
      const d = Math.hypot(x - exc.x, z - exc.z);
      if (d < exc.radius) {
        h -= (1.0 - Math.pow(d / exc.radius, 2.0)) * exc.depth;
      }
    }

    return h;
  }

  getWaterLevelWorld() {
    return -1.15;
  }

  buildTerrainMesh() {
    const gl = this.gl;
    const N = this.gridSize;
    const size = this.worldSize;
    const half = size * 0.5;

    const positions = [];
    const normals = [];
    const colors = [];
    const metas = [];
    const indices = [];

    const geol = this.currentSector?.geology || {};
    const rockCol = geol.rock_rgb || [0.34, 0.31, 0.28];
    const cliffCol = geol.cliff_rgb || [0.46, 0.40, 0.35];
    const soilCol = geol.soil_rgb || [0.42, 0.32, 0.22];
    const sedCol = geol.sediment_rgb || [0.62, 0.56, 0.44];
    const waterY = this.getWaterLevelWorld();

    this.getHeightAt = (x, z) => this.sampleElevation(x, z);

    // 1. Surface Topographic Mesh ((N+1) x (N+1))
    for (let j = 0; j <= N; j++) {
      const z = (j / N) * size - half;
      for (let i = 0; i <= N; i++) {
        const x = (i / N) * size - half;
        const y = this.sampleElevation(x, z);

        // Central finite-difference normal
        const eps = 0.45;
        const hL = this.sampleElevation(x - eps, z);
        const hR = this.sampleElevation(x + eps, z);
        const hD = this.sampleElevation(x, z - eps);
        const hU = this.sampleElevation(x, z + eps);
        let nx = -(hR - hL) / (2 * eps);
        let ny = 1.0;
        let nz = -(hU - hD) / (2 * eps);
        const len = Math.hypot(nx, ny, nz) || 1;
        nx /= len; ny /= len; nz /= len;

        // Biome & lithological vertex coloring
        const slope = 1.0 - Math.max(0, ny);
        const grassRiparian = [0.24, 0.42, 0.22];
        const grassSubalpine = [0.44, 0.48, 0.28];
        const alpineRidge = [0.58, 0.56, 0.52];

        let col = [...soilCol];
        // Blend riparian sediment near water
        if (y < waterY + 0.9) {
          const tSed = Math.max(0, Math.min(1, (waterY + 0.9 - y) / 1.8));
          col = [
            grassRiparian[0] * (1 - tSed) + sedCol[0] * tSed,
            grassRiparian[1] * (1 - tSed) + sedCol[1] * tSed,
            grassRiparian[2] * (1 - tSed) + sedCol[2] * tSed
          ];
        } else {
          const tAlt = Math.max(0, Math.min(1, (y - waterY) / 16.0));
          col = [
            grassRiparian[0] * (1 - tAlt) + grassSubalpine[0] * tAlt,
            grassRiparian[1] * (1 - tAlt) + grassSubalpine[1] * tAlt,
            grassRiparian[2] * (1 - tAlt) + grassSubalpine[2] * tAlt
          ];
          if (y > 10.5) {
            const tHigh = Math.min(1, (y - 10.5) / 6.0);
            col = [
              col[0] * (1 - tHigh) + alpineRidge[0] * tHigh,
              col[1] * (1 - tHigh) + alpineRidge[1] * tHigh,
              col[2] * (1 - tHigh) + alpineRidge[2] * tHigh
            ];
          }
        }

        // Steep cliff lithology exposure
        if (slope > 0.26) {
          const tCliff = Math.min(1, (slope - 0.26) / 0.35);
          const cTarget = slope > 0.52 ? cliffCol : rockCol;
          col = [
            col[0] * (1 - tCliff) + cTarget[0] * tCliff,
            col[1] * (1 - tCliff) + cTarget[1] * tCliff,
            col[2] * (1 - tCliff) + cTarget[2] * tCliff
          ];
        }

        positions.push(x, y, z);
        normals.push(nx, ny, nz);
        colors.push(col[0], col[1], col[2]);
        metas.push((y + 4.0) / 24.0, 0.0, 0.0, y < waterY + 1.5 ? 0.85 : 0.35);
      }
    }

    for (let j = 0; j < N; j++) {
      for (let i = 0; i < N; i++) {
        const r1 = j * (N + 1);
        const r2 = (j + 1) * (N + 1);
        indices.push(r1 + i, r2 + i, r1 + i + 1);
        indices.push(r1 + i + 1, r2 + i, r2 + i + 1);
      }
    }

    // 2. 6-Layer CSIRO SLGA Subsurface Stratigraphic Cutaway Skirt Walls
    // Renders along the 4 edges of the terrain block AND along the active Cross-Section Z plane
    const strataColors = [
      [0.24, 0.18, 0.14], // L0: 0-5cm Organic Humus
      [0.38, 0.27, 0.18], // L1: 5-15cm A-Horizon Topsoil
      [0.52, 0.38, 0.25], // L2: 15-30cm Transition Zone
      [0.64, 0.44, 0.28], // L3: 30-60cm B-Horizon Illuvial Clay
      [0.48, 0.44, 0.39], // L4: 60-100cm Weathered Saprolite
      rockCol             // L5: 100-200cm Deep Regolith & GSNSW Bedrock
    ];
    const baseDepthY = -9.5;

    const addStrataWallStrip = (samplePtFn, wallNorm) => {
      const steps = 60;
      const layers = 6;
      const startVert = positions.length / 3;

      for (let s = 0; s <= steps; s++) {
        const t = s / steps;
        const [wx, wz] = samplePtFn(t);
        const topY = this.sampleElevation(wx, wz);

        for (let l = 0; l <= layers; l++) {
          const frac = l / layers;
          const wy = topY * (1.0 - frac) + baseDepthY * frac;
          const sCol = strataColors[Math.min(layers - 1, l)];
          positions.push(wx, wy, wz);
          normals.push(wallNorm[0], wallNorm[1], wallNorm[2]);
          colors.push(sCol[0], sCol[1], sCol[2]);
          metas.push(1.0 - frac, l, 1.0, 0.5);
        }
      }

      const stride = layers + 1;
      for (let s = 0; s < steps; s++) {
        for (let l = 0; l < layers; l++) {
          const a = startVert + s * stride + l;
          const b = startVert + (s + 1) * stride + l;
          const c = a + 1;
          const d = b + 1;
          indices.push(a, c, b, b, c, d);
        }
      }
    };

    // Front Z=+half, Back Z=-half, Left X=-half, Right X=+half skirt walls
    addStrataWallStrip(t => [-half + t * size, half], [0, 0, 1]);
    addStrataWallStrip(t => [half - t * size, -half], [0, 0, -1]);
    addStrataWallStrip(t => [-half, -half + t * size], [-1, 0, 0]);
    addStrataWallStrip(t => [half, half - t * size], [1, 0, 0]);

    // Internal Cross-Section Wall at Z = 0 (visible when subsurface_cutaway clips Z > 0)
    addStrataWallStrip(t => [-half + t * size, 0.0], [0, 0, 1]);

    this.terrainGeom = this.uploadGeometry({
      positions: new Float32Array(positions),
      normals: new Float32Array(normals),
      colors: new Float32Array(colors),
      metas: new Float32Array(metas),
      indices: this.uintExt ? new Uint32Array(indices) : new Uint16Array(indices),
      useUint32: Boolean(this.uintExt)
    });
  }

  buildSkyAndWaterMeshes() {
    const half = this.worldSize * 0.5;
    const waterY = this.getWaterLevelWorld();

    // 1. Animated Water Mesh (48x48 grid at waterY)
    const WN = 48;
    const wPos = [], wNorm = [], wCol = [], wMeta = [], wIdx = [];
    const secId = this.currentSector?.id || 'talbingo_pines';
    const waterTint = secId === 'yarrangobilly_cliff'
      ? [0.12, 0.58, 0.56] // Geothermal mineral turquoise
      : secId === 'noosa_headland'
        ? [0.08, 0.44, 0.58] // Subtropical coastal teal
        : [0.10, 0.34, 0.48]; // Deep alpine reservoir / river blue

    for (let j = 0; j <= WN; j++) {
      const z = (j / WN) * this.worldSize - half;
      for (let i = 0; i <= WN; i++) {
        const x = (i / WN) * this.worldSize - half;
        wPos.push(x, waterY, z);
        wNorm.push(0, 1, 0);
        wCol.push(waterTint[0], waterTint[1], waterTint[2]);
        wMeta.push(0, 0, 0, 1);
      }
    }
    for (let j = 0; j < WN; j++) {
      for (let i = 0; i < WN; i++) {
        const r1 = j * (WN + 1), r2 = (j + 1) * (WN + 1);
        wIdx.push(r1 + i, r2 + i, r1 + i + 1, r1 + i + 1, r2 + i, r2 + i + 1);
      }
    }
    this.waterGeom = this.uploadGeometry({
      positions: new Float32Array(wPos),
      normals: new Float32Array(wNorm),
      colors: new Float32Array(wCol),
      metas: new Float32Array(wMeta),
      indices: new Uint16Array(wIdx),
      useUint32: false
    });

    // 2. Sky Dome Hemisphere (radius = 190)
    const sPos = [], sNorm = [], sCol = [], sMeta = [], sIdx = [];
    const lat = 12, lon = 24, R = 190.0;
    for (let i = 0; i <= lat; i++) {
      const theta = (Math.PI * 0.55 * i) / lat;
      for (let j = 0; j <= lon; j++) {
        const phi = (2 * Math.PI * j) / lon;
        const x = R * Math.sin(theta) * Math.cos(phi);
        const y = R * Math.cos(theta);
        const z = R * Math.sin(theta) * Math.sin(phi);
        sPos.push(x, y, z);
        sNorm.push(0, -1, 0);
        sCol.push(0.2, 0.4, 0.8);
        sMeta.push(0, 0, 0, 0);
      }
    }
    for (let i = 0; i < lat; i++) {
      for (let j = 0; j < lon; j++) {
        const a = i * (lon + 1) + j;
        const b = (i + 1) * (lon + 1) + j;
        // Inside-facing winding
        sIdx.push(a, a + 1, b, a + 1, b + 1, b);
      }
    }
    this.skyGeom = this.uploadGeometry({
      positions: new Float32Array(sPos),
      normals: new Float32Array(sNorm),
      colors: new Float32Array(sCol),
      metas: new Float32Array(sMeta),
      indices: new Uint16Array(sIdx),
      useUint32: false
    });
  }

  uploadGeometry(data) {
    const gl = this.gl;
    if (!gl) {
      return {
        indexCount: data.indices.length,
        wirePositions: data.wirePositions || data.skeletonLines || null,
        rawData: data
      };
    }
    const posBuf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, posBuf);
    gl.bufferData(gl.ARRAY_BUFFER, data.positions, gl.STATIC_DRAW);

    const normBuf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, normBuf);
    gl.bufferData(gl.ARRAY_BUFFER, data.normals, gl.STATIC_DRAW);

    const colBuf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, colBuf);
    gl.bufferData(gl.ARRAY_BUFFER, data.colors, gl.STATIC_DRAW);

    const metaBuf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, metaBuf);
    gl.bufferData(gl.ARRAY_BUFFER, data.metas || data.windParams || data.animParams, gl.STATIC_DRAW);

    const idxBuf = gl.createBuffer();
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, idxBuf);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, data.indices, gl.STATIC_DRAW);

    return {
      posBuf,
      normBuf,
      colBuf,
      metaBuf,
      idxBuf,
      indexCount: data.indices.length,
      indexType: data.useUint32 ? gl.UNSIGNED_INT : gl.UNSIGNED_SHORT,
      wirePositions: data.wirePositions || data.skeletonLines || null,
      rawData: data
    };
  }

  getUploadedFloraMesh(species, detail = 'scene') {
    const key = `gpu_flora_${species.id}_${detail}`;
    if (!this[key]) {
      const raw = this.floraEngine.getSpeciesGeometry(species, detail);
      this[key] = this.uploadGeometry(raw);
    }
    return this[key];
  }

  getUploadedFaunaMesh(species, isLoupe = false) {
    const key = `gpu_fauna_${species.id}_${isLoupe ? 'loupe' : 'scene'}`;
    if (!this[key]) {
      const raw = this.faunaEngine.getSpeciesGeometry(species, isLoupe);
      this[key] = this.uploadGeometry(raw);
    }
    return this[key];
  }

  // ============================================================================
  // POPULATING 100 FLORA & 64 FAUNA ACROSS THE TERRAIN BIOME ZONES
  // ============================================================================

  populateEcosystem() {
    this.floraInstances = [];
    this.faunaAgents = [];

    if (!this.floraCatalog.length || !this.faunaCatalog.length) return;

    const secId = this.currentSector?.id || 'talbingo_pines';
    const waterY = this.getWaterLevelWorld();

    // Filter sector-primary flora, plus ensure all 100 flora species are represented in the regional corridor
    const sectorFlora = this.floraCatalog.filter(f => !f.sectors || f.sectors.includes(secId));
    const poolFlora = sectorFlora.length >= 12 ? sectorFlora : this.floraCatalog;

    // Place 180 flora instances across the terrain matching moisture/elevation zones
    const totalFloraInstances = 180;
    for (let i = 0; i < totalFloraInstances; i++) {
      // Ensure every species in poolFlora and across the 100 catalog gets placed
      const sp = i < this.floraCatalog.length
        ? this.floraCatalog[i]
        : poolFlora[i % poolFlora.length];

      // Deterministic Poisson-like golden spiral + jitter distribution
      const r = 6.0 + Math.sqrt((i + 0.5) / totalFloraInstances) * 50.0;
      const theta = i * 2.399963 + (i % 7) * 0.15;
      let x = Math.cos(theta) * r;
      let z = Math.sin(theta) * r;

      let y = this.sampleElevation(x, z);
      // Keep terrestrial plants above water level; riparian plants near water edge
      if (y < waterY + 0.15) {
        x = (x >= 0 ? 1 : -1) * Math.max(18.0, Math.abs(x) + 14.0);
        x = Math.max(-54, Math.min(54, x));
        y = Math.max(waterY + 0.15, this.sampleElevation(x, z));
      }

      const scale = 0.78 + ((i * 17) % 10) * 0.045;
      const rotY = (i * 1.17) % (Math.PI * 2);

      this.floraInstances.push({
        species: sp,
        x,
        y,
        z,
        scale,
        rotY
      });
    }

    // Spawn 64 active Fauna Agents (every single one of the 64 fauna species active & inspectable!)
    for (let i = 0; i < this.faunaCatalog.length; i++) {
      const sp = this.faunaCatalog[i];
      const habitat = sp.habitat_stratum || 'terrestrial';
      const gait = sp.kinematics?.locomotion_gait || 'bipedal_hop';

      const ang = i * 2.399963;
      const rad = 8.0 + ((i * 13) % 42);
      let x = Math.cos(ang) * rad;
      let z = Math.sin(ang) * rad;

      if (habitat === 'riparian_aquatic' || habitat === 'benthic_stream' || gait === 'sub_aquatic_swim') {
        // Place aquatic / riparian fauna (Platypus, Murray Cod, Macquarie Perch, Turtle, Swan, Water Dragon) along river channel
        z = -48.0 + (i % 16) * 6.0;
        x = -4.0 + Math.sin(z * 0.04) * 3.0 + ((i % 3) - 1) * 2.5;
      } else {
        // Ensure terrestrial fauna don't spawn deep underwater
        const testY = this.sampleElevation(x, z);
        if (testY < waterY + 0.3) {
          x = Math.max(-50, Math.min(50, x + (x >= 0 ? 14 : -14)));
        }
      }

      const y = this.sampleElevation(x, z);
      const states = sp.ethology?.states || ['FORAGING', 'MOVING', 'VIGILANT_SCAN'];

      this.faunaAgents.push({
        index: i,
        species: sp,
        x,
        y,
        z,
        homeX: x,
        homeZ: z,
        yaw: (i * 0.9) % (Math.PI * 2),
        targetYaw: (i * 0.9) % (Math.PI * 2),
        speed: sp.kinematics?.velocity_walk_ms || 1.2,
        phase: i * 0.7,
        seed: i + 1,
        stateIndex: i % states.length,
        behaviorState: states[i % states.length],
        stateTimer: 2.0 + (i % 5)
      });
    }

    if (!this.loupeTarget.species && this.floraCatalog.length) {
      this.loupeTarget = { type: 'flora', species: this.floraCatalog[0] };
    }
  }

  // ============================================================================
  // VIEW MODES, LOUPE SELECTION & INTERACTION HANDLERS
  // ============================================================================

  setObserverMode(mode) {
    this.observerMode = mode;
  }

  setViewMode(mode) {
    this.viewMode = mode;
    if (mode === 'subsurface_cutaway') {
      this.clipPlaneZ = 0.0;
      this.camera.target = [0, -1.0, -4.0];
      this.camera.rotX = 0.28;
      this.camera.rotY = 0.32;
      this.camera.dist = 46.0;
    } else {
      this.clipPlaneZ = 999.0;
    }

    if (mode === 'corridor_gis') {
      this.camera.rotX = 1.42; // Near top-down orthographic survey
      this.camera.rotY = 0.0;
      this.camera.dist = 95.0;
      this.camera.target = [0, 0, 0];
    } else if (mode === 'aerial_orbit') {
      this.camera.rotX = 0.48;
      this.camera.rotY = 0.65;
      this.camera.dist = 62.0;
      this.camera.target = [0, 4.0, 0];
    }
  }

  setLoupeSpecimen(type, species) {
    this.loupeTarget = { type, species };
  }

  focusFaunaAgent(agentIndex) {
    if (agentIndex >= 0 && agentIndex < this.faunaAgents.length) {
      this.trackedFaunaIndex = agentIndex;
      const ag = this.faunaAgents[agentIndex];
      this.loupeTarget = { type: 'fauna', species: ag.species };
    }
  }

  triggerSonicRipple(x, y, z, color = [0.0, 0.95, 1.0]) {
    this.sonicRipples.push({ x, y: Math.max(y, 1.0), z, radius: 0.5, maxRadius: 22.0, color });
  }

  setExcavationMode(active) {
    this.excavationMode = active;
  }

  digExcavation(worldX, worldZ) {
    this.excavations.push({
      x: worldX,
      z: worldZ,
      radius: 6.5,
      depth: 4.5
    });
    this.buildTerrainMesh();
  }

  setupInteractions() {
    const canvas = this.canvas;

    window.addEventListener('keydown', (e) => {
      this.keysPressed[e.key.toLowerCase()] = true;
    });
    window.addEventListener('keyup', (e) => {
      this.keysPressed[e.key.toLowerCase()] = false;
    });

    canvas.addEventListener('mousedown', (e) => {
      this.isDragging = true;
      this.lastMouse = { x: e.clientX, y: e.clientY };

      if (this.excavationMode && this.viewMode !== 'specimen_loupe') {
        const rect = canvas.getBoundingClientRect();
        const ndcX = ((e.clientX - rect.left) / canvas.width) * 2 - 1;
        const ndcY = -(((e.clientY - rect.top) / canvas.height) * 2 - 1);
        const pickX = this.camera.target[0] + ndcX * 26.0;
        const pickZ = this.camera.target[2] - ndcY * 26.0;
        this.digExcavation(pickX, pickZ);
        this.triggerSonicRipple(pickX, this.sampleElevation(pickX, pickZ), pickZ, [1.0, 0.2, 0.65]);
        if (window.onExcavationDone) {
          window.onExcavationDone(pickX, pickZ, 4.5);
        }
      }
    });

    window.addEventListener('mouseup', () => {
      this.isDragging = false;
    });

    window.addEventListener('mousemove', (e) => {
      if (!this.isDragging || this.excavationMode) return;
      const dx = e.clientX - this.lastMouse.x;
      const dy = e.clientY - this.lastMouse.y;
      this.lastMouse = { x: e.clientX, y: e.clientY };

      if (this.viewMode === 'ground_walk') {
        this.camera.walkYaw -= dx * 0.006;
        this.camera.walkPitch = Math.max(-0.8, Math.min(0.8, this.camera.walkPitch - dy * 0.005));
      } else if (e.buttons === 1) {
        this.camera.rotY += dx * 0.007;
        this.camera.rotX = Math.max(0.08, Math.min(1.52, this.camera.rotX + dy * 0.007));
      } else if (e.buttons === 2) {
        this.camera.target[0] -= dx * 0.06;
        this.camera.target[2] -= dy * 0.06;
      }
    });

    canvas.addEventListener('wheel', (e) => {
      e.preventDefault();
      this.camera.dist = Math.max(6.0, Math.min(130.0, this.camera.dist + e.deltaY * 0.05));
    }, { passive: false });

    canvas.addEventListener('contextmenu', (e) => e.preventDefault());
  }

  resize() {
    const w = this.canvas.clientWidth || 1280;
    const h = this.canvas.clientHeight || 720;
    if (this.canvas.width !== w || this.canvas.height !== h) {
      this.canvas.width = w;
      this.canvas.height = h;
      if (this.gl) {
        this.gl.viewport(0, 0, w, h);
      }
    }
  }

  // ============================================================================
  // MAIN RENDER LOOP
  // ============================================================================

  render() {
    this.resize();
    const gl = this.gl;
    const dt = 0.016;
    this.time += dt;

    // Update Sun Direction from timeOfDayHours (6:00 sunrise -> 12:00 zenith -> 18:30 sunset)
    const dayAngle = ((this.timeOfDayHours - 6.0) / 12.5) * Math.PI;
    const sunDir = [
      Math.cos(dayAngle) * 0.85,
      Math.max(0.08, Math.sin(dayAngle)),
      0.35
    ];

    const windVec = [
      Math.cos(this.windDirRad) * this.windSpeedMs,
      Math.sin(this.windDirRad) * this.windSpeedMs
    ];

    // Advance all 64 Fauna Autonomous Agents
    for (const ag of this.faunaAgents) {
      this.faunaEngine.updateAgent(ag, dt, this.time, this.getHeightAt, this.camera.eye);
    }

    // Update Camera Matrices based on active View Mode
    let eyePos = [0, 25, 45];
    let targetPos = [0, 3, 0];

    if (this.viewMode === 'specimen_loupe') {
      const d = Math.max(6.0, Math.min(28.0, this.camera.dist * 0.24));
      const rx = Math.max(0.12, Math.min(1.1, this.camera.rotX * 0.6));
      const ry = this.camera.rotY + this.time * 0.25;
      targetPos = [0, 3.2, 0];
      eyePos = [
        targetPos[0] + d * Math.cos(rx) * Math.sin(ry),
        targetPos[1] + d * Math.sin(rx),
        targetPos[2] + d * Math.cos(rx) * Math.cos(ry)
      ];
    } else if (this.viewMode === 'ground_walk') {
      const moveSpeed = 12.0 * dt;
      const forwardX = Math.sin(this.camera.walkYaw);
      const forwardZ = Math.cos(this.camera.walkYaw);
      const rightX = Math.cos(this.camera.walkYaw);
      const rightZ = -Math.sin(this.camera.walkYaw);

      if (this.keysPressed['w'] || this.keysPressed['arrowup']) {
        this.camera.walkPos[0] += forwardX * moveSpeed;
        this.camera.walkPos[2] += forwardZ * moveSpeed;
      }
      if (this.keysPressed['s'] || this.keysPressed['arrowdown']) {
        this.camera.walkPos[0] -= forwardX * moveSpeed;
        this.camera.walkPos[2] -= forwardZ * moveSpeed;
      }
      if (this.keysPressed['a'] || this.keysPressed['arrowleft']) {
        this.camera.walkPos[0] += rightX * moveSpeed;
        this.camera.walkPos[2] += rightZ * moveSpeed;
      }
      if (this.keysPressed['d'] || this.keysPressed['arrowright']) {
        this.camera.walkPos[0] -= rightX * moveSpeed;
        this.camera.walkPos[2] -= rightZ * moveSpeed;
      }

      this.camera.walkPos[0] = Math.max(-52, Math.min(52, this.camera.walkPos[0]));
      this.camera.walkPos[2] = Math.max(-52, Math.min(52, this.camera.walkPos[2]));
      const groundH = Math.max(this.getWaterLevelWorld() + 0.5, this.sampleElevation(this.camera.walkPos[0], this.camera.walkPos[2]));
      eyePos = [this.camera.walkPos[0], groundH + 2.1, this.camera.walkPos[2]];
      targetPos = [
        eyePos[0] + forwardX * Math.cos(this.camera.walkPitch) * 10.0,
        eyePos[1] + Math.sin(this.camera.walkPitch) * 10.0,
        eyePos[2] + forwardZ * Math.cos(this.camera.walkPitch) * 10.0
      ];
    } else if (this.viewMode === 'fauna_tracker' && this.faunaAgents.length) {
      const ag = this.faunaAgents[this.trackedFaunaIndex % this.faunaAgents.length];
      targetPos = [ag.x, ag.y + 1.2, ag.z];
      const d = Math.max(5.5, Math.min(24.0, this.camera.dist * 0.22));
      const ry = this.camera.rotY;
      const rx = Math.max(0.2, Math.min(0.85, this.camera.rotX));
      eyePos = [
        targetPos[0] + d * Math.cos(rx) * Math.sin(ry),
        targetPos[1] + d * Math.sin(rx),
        targetPos[2] + d * Math.cos(rx) * Math.cos(ry)
      ];
    } else {
      // aerial_orbit, subsurface_cutaway, corridor_gis
      targetPos = [...this.camera.target];
      eyePos = [
        targetPos[0] + this.camera.dist * Math.cos(this.camera.rotX) * Math.sin(this.camera.rotY),
        targetPos[1] + this.camera.dist * Math.sin(this.camera.rotX),
        targetPos[2] + this.camera.dist * Math.cos(this.camera.rotX) * Math.cos(this.camera.rotY)
      ];
    }

    this.camera.eye = eyePos;
    this.camera.velocity = [
      eyePos[0] - this.camera.lastPos[0],
      eyePos[1] - this.camera.lastPos[1],
      eyePos[2] - this.camera.lastPos[2]
    ];
    this.camera.lastPos = [...eyePos];

    // Primary Hardware Path: Execute WebGPU Compute + Render Pass via WGSL Forge
    if (this.webgpuEngine && this.webgpuEngine.ready) {
      this.webgpuEngine.renderFrame(this, eyePos, targetPos, sunDir, windVec);
      return;
    }
    if (!gl) return;

    // Clear & Setup Depth
    gl.clearColor(0.04, 0.07, 0.12, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    gl.enable(gl.DEPTH_TEST);

    const aspect = this.canvas.width / Math.max(1, this.canvas.height);
    const projMat = this.createPerspective(46 * Math.PI / 180, aspect, 0.4, 380.0);
    const viewMat = this.createLookAt(eyePos, targetPos, [0, 1, 0]);
    const identMat = this.createIdentity();

    gl.useProgram(this.program);
    gl.uniformMatrix4fv(this.uniforms.projection, false, projMat);
    gl.uniformMatrix4fv(this.uniforms.view, false, viewMat);
    gl.uniform3fv(this.uniforms.cameraPos, eyePos);
    gl.uniform3fv(this.uniforms.sunDir, sunDir);
    gl.uniform1i(this.uniforms.observerMode, this.observerMode);
    gl.uniform1f(this.uniforms.time, this.time);
    gl.uniform2fv(this.uniforms.windVec, windVec);
    gl.uniform1f(this.uniforms.clipPlaneZ, this.clipPlaneZ);
    gl.uniform1f(this.uniforms.waterLevel, this.getWaterLevelWorld());
    gl.uniform1f(this.uniforms.karstFluting, this.currentSector?.geology?.karst_fluting || 0.15);
    gl.uniform2f(this.uniforms.spaceWeather, this.kpIndex, this.imfBz);

    if (this.activeIsogloss) {
      gl.uniform4f(
        this.uniforms.isoglossParam,
        this.activeIsogloss.center_x || 0,
        this.activeIsogloss.radius || 16,
        this.activeIsogloss.center_z || 0,
        1.0
      );
    } else {
      gl.uniform4f(this.uniforms.isoglossParam, 0, 0, 0, 0);
    }

    // 1. Draw Sky Dome
    gl.uniform1i(this.uniforms.entityKind, 4);
    gl.uniformMatrix4fv(this.uniforms.model, false, this.createTransform(eyePos[0], eyePos[1] - 20.0, eyePos[2], 0, 1.0));
    this.drawGeometry(this.skyGeom);

    // Dedicated 3D Specimen Studio & Anatomical Loupe View Mode
    if (this.viewMode === 'specimen_loupe' && this.loupeTarget.species) {
      this.renderLoupeStage(projMat, viewMat);
      return;
    }

    // 2. Draw Terrain & 6-Layer CSIRO SLGA Subsurface Stratigraphic Cutaway
    gl.uniform1i(this.uniforms.entityKind, 0);
    gl.uniformMatrix4fv(this.uniforms.model, false, identMat);
    this.drawGeometry(this.terrainGeom);

    // 3. Draw Animated Hydrological Water Surface
    gl.uniform1i(this.uniforms.entityKind, 3);
    gl.uniformMatrix4fv(this.uniforms.model, false, identMat);
    this.drawGeometry(this.waterGeom);

    // 4. Draw All Flora Instances with GPU Aeroelastic Wind Motion
    gl.uniform1i(this.uniforms.entityKind, 1);
    for (const inst of this.floraInstances) {
      if (inst.z > this.clipPlaneZ) continue;
      const mesh = this.getUploadedFloraMesh(inst.species, 'scene');
      const modelMat = this.createTransform(inst.x, inst.y, inst.z, inst.rotY, inst.scale);
      gl.uniformMatrix4fv(this.uniforms.model, false, modelMat);
      this.drawGeometry(mesh);
    }

    // 5. Draw All 64 Fauna Agents with Articulated Biomechanical Gait Kinematics
    gl.uniform1i(this.uniforms.entityKind, 2);
    for (const ag of this.faunaAgents) {
      if (ag.z > this.clipPlaneZ) continue;
      const mesh = this.getUploadedFaunaMesh(ag.species, false);
      const gaitCode = this.getGaitNumericCode(ag.species.kinematics?.locomotion_gait);
      gl.uniform4f(this.uniforms.faunaAnim, ag.phase, ag.speed, gaitCode, 0.0);
      const modelMat = this.createTransform(ag.x, ag.y, ag.z, ag.yaw, 1.0);
      gl.uniformMatrix4fv(this.uniforms.model, false, modelMat);
      this.drawGeometry(mesh);
    }

    // 6. Draw Trophic Interaction Arcs, Sonic Wavefront Rings & Subsurface Root Wireframes
    this.renderOverlayLines(projMat, viewMat, identMat);
  }

  /**
   * Renders the dedicated 3D Specimen Studio & Anatomical Loupe turntable
   */
  renderLoupeStage(projMat, viewMat) {
    const gl = this.gl;
    const target = this.loupeTarget;
    const s = this.specimenGrowthScale;

    // Pedestal disc
    gl.uniform1i(this.uniforms.entityKind, 0);
    gl.uniformMatrix4fv(this.uniforms.model, false, this.createTransform(0, -1.5, 0, 0, 0.14));
    this.drawGeometry(this.terrainGeom);

    let wirePoints = null;
    const modelMat = this.createTransform(0, 0, 0, 0, s);

    if (target.type === 'flora') {
      const mesh = this.getUploadedFloraMesh(target.species, 'loupe');
      gl.uniform1i(this.uniforms.entityKind, 1);
      gl.uniformMatrix4fv(this.uniforms.model, false, modelMat);
      this.drawGeometry(mesh);
      wirePoints = mesh.wirePositions;
    } else {
      const mesh = this.getUploadedFaunaMesh(target.species, true);
      const gaitCode = this.getGaitNumericCode(target.species.kinematics?.locomotion_gait);
      gl.uniform1i(this.uniforms.entityKind, 2);
      gl.uniform4f(this.uniforms.faunaAnim, this.time * 4.5, 2.8, gaitCode, 0.0);
      gl.uniformMatrix4fv(this.uniforms.model, false, modelMat);
      this.drawGeometry(mesh);
      wirePoints = mesh.wirePositions;
    }

    // Draw Anatomical Osteology / Vascular Root Wireframe Overlay if enabled
    if (this.showWireframeOverlay && wirePoints && wirePoints.length > 0) {
      gl.useProgram(this.lineProgram);
      gl.disable(gl.DEPTH_TEST);
      gl.uniformMatrix4fv(this.lineUniforms.projection, false, projMat);
      gl.uniformMatrix4fv(this.lineUniforms.view, false, viewMat);
      gl.uniformMatrix4fv(this.lineUniforms.model, false, modelMat);
      gl.uniform4f(this.lineUniforms.lineColor, 0.0, 0.96, 1.0, 1.0);

      gl.bindBuffer(gl.ARRAY_BUFFER, this.dynamicLineBuffer);
      gl.bufferData(gl.ARRAY_BUFFER, wirePoints, gl.DYNAMIC_DRAW);
      gl.enableVertexAttribArray(this.lineAttribs.position);
      gl.vertexAttribPointer(this.lineAttribs.position, 3, gl.FLOAT, false, 0, 0);
      gl.drawArrays(gl.LINES, 0, wirePoints.length / 3);
      gl.enable(gl.DEPTH_TEST);
    }
  }

  /**
   * Collects 3D Trophic Food-Web Arcs & Bioacoustic Sonic Wavefront Rings
   */
  collectOverlayLines() {
    const lines = [];

    // 1. Trophic Interaction Arcs (in Tactical GPR HUD mode 3, Heliophysics mode 5, or Fauna Tracker)
    if (this.observerMode === 3 || this.observerMode === 5 || this.viewMode === 'fauna_tracker' || this.viewMode === 'corridor_gis') {
      const limit = Math.min(24, this.faunaAgents.length);
      for (let i = 0; i < limit; i++) {
        const ag = this.faunaAgents[i];
        const nearFlora = this.floraInstances[(i * 7) % Math.max(1, this.floraInstances.length)];
        if (nearFlora) {
          const midX = (ag.x + nearFlora.x) * 0.5;
          const midY = Math.max(ag.y, nearFlora.y) + 4.5;
          const midZ = (ag.z + nearFlora.z) * 0.5;
          lines.push(ag.x, ag.y + 0.8, ag.z, midX, midY, midZ);
          lines.push(midX, midY, midZ, nearFlora.x, nearFlora.y + 1.5, nearFlora.z);
        }
      }
    }

    // 2. Expanding 3D Sonic Wavefront Rings (Bioacoustics & Sonic Toponyms)
    for (let r = this.sonicRipples.length - 1; r >= 0; r--) {
      const rip = this.sonicRipples[r];
      rip.radius += 0.22;
      if (rip.radius > rip.maxRadius) {
        this.sonicRipples.splice(r, 1);
        continue;
      }
      const segs = 24;
      for (let s = 0; s < segs; s++) {
        const a0 = (2 * Math.PI * s) / segs;
        const a1 = (2 * Math.PI * (s + 1)) / segs;
        lines.push(
          rip.x + Math.cos(a0) * rip.radius, rip.y + 0.6, rip.z + Math.sin(a0) * rip.radius,
          rip.x + Math.cos(a1) * rip.radius, rip.y + 0.6, rip.z + Math.sin(a1) * rip.radius
        );
      }
    }

    return lines;
  }

  /**
   * Renders 3D Trophic Food-Web Arcs, Bioacoustic Sonic Ripples & Subsurface Root Networks
   */
  renderOverlayLines(projMat, viewMat, identMat) {
    const gl = this.gl;
    const lines = this.collectOverlayLines();

    if (lines.length > 0) {
      gl.useProgram(this.lineProgram);
      gl.uniformMatrix4fv(this.lineUniforms.projection, false, projMat);
      gl.uniformMatrix4fv(this.lineUniforms.view, false, viewMat);
      gl.uniformMatrix4fv(this.lineUniforms.model, false, identMat);
      gl.uniform4f(this.lineUniforms.lineColor, 0.0, 0.95, 0.88, 1.0);

      gl.bindBuffer(gl.ARRAY_BUFFER, this.dynamicLineBuffer);
      gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(lines), gl.DYNAMIC_DRAW);
      gl.enableVertexAttribArray(this.lineAttribs.position);
      gl.vertexAttribPointer(this.lineAttribs.position, 3, gl.FLOAT, false, 0, 0);
      gl.drawArrays(gl.LINES, 0, lines.length / 3);
    }
  }

  getGaitNumericCode(gait) {
    switch (gait) {
      case 'bipedal_hop': return 1.0;
      case 'pentapedal_graze': return 2.0;
      case 'fossorial_waddle': return 3.0;
      case 'arboreal_climb': return 4.0;
      case 'quadrupedal_trot': return 5.0;
      case 'soaring_flight': return 6.0;
      case 'flapping_flight':
      case 'patagial_glide':
      case 'insect_flutter': return 7.0;
      case 'avian_display_walk': return 8.0;
      case 'sub_aquatic_swim': return 9.0;
      case 'sinusoidal_slither': return 10.0;
      case 'sprawling_lizard': return 11.0;
      case 'saltatorial_frog': return 12.0;
      default: return 1.0;
    }
  }

  drawGeometry(geom) {
    if (!geom) return;
    const gl = this.gl;

    gl.bindBuffer(gl.ARRAY_BUFFER, geom.posBuf);
    gl.enableVertexAttribArray(this.attribs.position);
    gl.vertexAttribPointer(this.attribs.position, 3, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ARRAY_BUFFER, geom.normBuf);
    gl.enableVertexAttribArray(this.attribs.normal);
    gl.vertexAttribPointer(this.attribs.normal, 3, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ARRAY_BUFFER, geom.colBuf);
    gl.enableVertexAttribArray(this.attribs.color);
    gl.vertexAttribPointer(this.attribs.color, 3, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ARRAY_BUFFER, geom.metaBuf);
    gl.enableVertexAttribArray(this.attribs.meta);
    gl.vertexAttribPointer(this.attribs.meta, 4, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, geom.idxBuf);
    gl.drawElements(gl.TRIANGLES, geom.indexCount, geom.indexType, 0);
  }

  // ============================================================================
  // MATRIX & LINEAR ALGEBRA KERNELS
  // ============================================================================

  createIdentity() {
    return new Float32Array([
      1, 0, 0, 0,
      0, 1, 0, 0,
      0, 0, 1, 0,
      0, 0, 0, 1
    ]);
  }

  createTransform(tx, ty, tz, rotY, scale) {
    const c = Math.cos(rotY) * scale;
    const s = Math.sin(rotY) * scale;
    return new Float32Array([
      c, 0, -s, 0,
      0, scale, 0, 0,
      s, 0, c, 0,
      tx, ty, tz, 1
    ]);
  }

  createPerspective(fovy, aspect, near, far) {
    const f = 1.0 / Math.tan(fovy / 2);
    const nf = 1 / (near - far);
    return new Float32Array([
      f / aspect, 0, 0, 0,
      0, f, 0, 0,
      0, 0, (far + near) * nf, -1,
      0, 0, (2 * far * near) * nf, 0
    ]);
  }

  createLookAt(eye, center, up) {
    const z0 = eye[0] - center[0];
    const z1 = eye[1] - center[1];
    const z2 = eye[2] - center[2];
    let len = 1 / (Math.hypot(z0, z1, z2) || 1);
    const zx = z0 * len, zy = z1 * len, zz = z2 * len;

    const x0 = up[1] * zz - up[2] * zy;
    const x1 = up[2] * zx - up[0] * zz;
    const x2 = up[0] * zy - up[1] * zx;
    len = 1 / (Math.hypot(x0, x1, x2) || 1);
    const xx = x0 * len, xy = x1 * len, xz = x2 * len;

    const yx = zy * xz - zz * xy;
    const yy = zz * xx - zx * xz;
    const yz = zx * xy - zy * xx;

    return new Float32Array([
      xx, yx, zx, 0,
      xy, yy, zy, 0,
      xz, yz, zz, 0,
      -(xx * eye[0] + xy * eye[1] + xz * eye[2]),
      -(yx * eye[0] + yy * eye[1] + yz * eye[2]),
      -(zx * eye[0] + zy * eye[1] + zz * eye[2]),
      1
    ]);
  }
}

window.TerrainRenderer = TerrainRenderer;
