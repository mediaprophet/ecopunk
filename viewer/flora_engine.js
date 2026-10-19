/**
 * EcoPunk 3D Botanical Morphogenesis & Aeroelastic Flora Engine
 *
 * Synthesizes multi-component 3D botanical geometry for 100 East Coast Australian
 * flora species using Bishop-frame profile sweeps, Cescatti asymmetric crown
 * envelopes, phyllotactic leaf/frond clusters, reproductive inflorescences,
 * and GPU vertex-shader aeroelastic wind encoding.
 */

class FloraEngine {
  constructor() {
    this.meshCache = new Map();
  }

  /**
   * Deterministic PRNG from integer seed
   */
   mulberry32(a) {
    return function() {
      let t = (a += 0x6D2B79F5);
      t = Math.imul(t ^ (t >>> 15), t | 1);
      t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  }

  hashString(str) {
    let h = 2166136261 >>> 0;
    for (let i = 0; i < str.length; i++) {
      h ^= str.charCodeAt(i);
      h = Math.imul(h, 16777619);
    }
    return h >>> 0;
  }

  /**
   * Builds or retrieves cached 3D geometry buffers for a flora species.
   * Each vertex packs:
   *  - position: [x, y, z]
   *  - normal: [nx, ny, nz]
   *  - color: [r, g, b]
   *  - windParam: [heightNorm, branchFlex, flutterWeight, partType]
   *    (partType: 0=bark/trunk, 1=branch, 2=foliage/leaf, 3=flower/cone, 4=root/vascular)
   */
  getSpeciesGeometry(species, detailLevel = 'scene') {
    const cacheKey = `${species.id}_${detailLevel}`;
    if (this.meshCache.has(cacheKey)) {
      return this.meshCache.get(cacheKey);
    }

    const rng = this.mulberry32(this.hashString(species.id));
    const builder = {
      positions: [],
      normals: [],
      colors: [],
      windParams: [],
      indices: [],
      wirePositions: [] // Vascular / structural wireframe for Specimen Loupe X-Ray
    };

    const form = (species.morphology && species.morphology.growth_form) || 'spreading_eucalypt';
    const H = detailLevel === 'loupe'
      ? 10.0
      : Math.max(0.45, Math.min(24.0, (species.traits?.typical_height_m || 10.0) * 0.45));

    const barkCol = species.morphology?.bark_rgb || [0.58, 0.54, 0.48];
    const barkSecCol = species.morphology?.bark_secondary_rgb || [0.78, 0.74, 0.66];
    const leafCol = species.morphology?.foliage_rgb || [0.26, 0.44, 0.28];
    const flowerCol = species.morphology?.flower_rgb || [0.92, 0.82, 0.25];
    const isHero = detailLevel === 'loupe';

    switch (form) {
      case 'contorted_gum':
        this.buildContortedGum(builder, rng, H, barkCol, barkSecCol, leafCol, flowerCol, isHero);
        break;
      case 'columnar_ash':
        this.buildColumnarAsh(builder, rng, H, barkCol, barkSecCol, leafCol, flowerCol, isHero);
        break;
      case 'weeping_riparian':
        this.buildWeepingRiparian(builder, rng, H, barkCol, leafCol, flowerCol, isHero);
        break;
      case 'wattle_dome':
        this.buildWattleDome(builder, rng, H, barkCol, leafCol, flowerCol, isHero);
        break;
      case 'banksia_cone_shrub':
      case 'waratah_shrub':
        this.buildBanksiaOrWaratah(builder, rng, H, barkCol, leafCol, flowerCol, form === 'waratah_shrub', isHero);
        break;
      case 'conifer_pyramidal':
        this.buildConiferPyramidal(builder, rng, H, barkCol, leafCol, flowerCol, isHero);
        break;
      case 'rainforest_buttress':
        this.buildRainforestButtress(builder, rng, H, barkCol, barkSecCol, leafCol, flowerCol, isHero);
        break;
      case 'tree_fern':
        this.buildTreeFern(builder, rng, H, barkCol, leafCol, isHero);
        break;
      case 'grass_tree':
        this.buildGrassTree(builder, rng, H, barkCol, leafCol, flowerCol, isHero);
        break;
      case 'palm_rosette':
      case 'cycad_rosette':
        this.buildPalmOrCycad(builder, rng, H, barkCol, leafCol, form === 'cycad_rosette', isHero);
        break;
      case 'tussock_grass':
      case 'reed_sedge':
        this.buildTussockOrReed(builder, rng, H, leafCol, flowerCol, form === 'reed_sedge', isHero);
        break;
      case 'alpine_cushion':
      case 'wildflower_herb':
        this.buildAlpineHerbOrCushion(builder, rng, H, leafCol, flowerCol, form === 'alpine_cushion', isHero);
        break;
      case 'mangrove_prop':
        this.buildMangrove(builder, rng, H, barkCol, leafCol, isHero);
        break;
      default:
        this.buildSpreadingEucalypt(builder, rng, H, barkCol, barkSecCol, leafCol, flowerCol, isHero);
        break;
    }

    // Add subterranean root architecture wireframe for Loupe / Subsurface view
    this.buildRootVascularWireframe(builder, rng, H, species.traits?.root_depth_m || 4.0);

    const geom = {
      positions: new Float32Array(builder.positions),
      normals: new Float32Array(builder.normals),
      colors: new Float32Array(builder.colors),
      windParams: new Float32Array(builder.windParams),
      indices: new Uint16Array(builder.indices),
      wirePositions: new Float32Array(builder.wirePositions),
      vertexCount: builder.positions.length / 3,
      indexCount: builder.indices.length,
      height: H
    };

    this.meshCache.set(cacheKey, geom);
    return geom;
  }

  // --- Low-level Geometry Primitives ---

  addVertex(b, pos, norm, col, wind) {
    const idx = b.positions.length / 3;
    b.positions.push(pos[0], pos[1], pos[2]);
    b.normals.push(norm[0], norm[1], norm[2]);
    b.colors.push(col[0], col[1], col[2]);
    b.windParams.push(wind[0], wind[1], wind[2], wind[3]);
    return idx;
  }

  addWireLine(b, p0, p1) {
    b.wirePositions.push(p0[0], p0[1], p0[2], p1[0], p1[1], p1[2]);
  }

  vecSub(a, b) { return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]; }
  vecAdd(a, b, s = 1) { return [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]; }
  vecCross(a, b) {
    return [
      a[1] * b[2] - a[2] * b[1],
      a[2] * b[0] - a[0] * b[2],
      a[0] * b[1] - a[1] * b[0]
    ];
  }
  vecNorm(a) {
    const l = Math.hypot(a[0], a[1], a[2]) || 1;
    return [a[0] / l, a[1] / l, a[2] / l];
  }

  /**
   * Sweeps a multi-station tube along 3D spline points with two-tone bark height blending
   */
  addSweptTube(b, points, radii, colBottom, colTop, sides = 7, maxH = 10.0, partType = 0, roughness = 0.08) {
    const rings = [];
    for (let i = 0; i < points.length; i++) {
      if (i > 0) this.addWireLine(b, points[i - 1], points[i]);
      const pPrev = points[Math.max(0, i - 1)];
      const pNext = points[Math.min(points.length - 1, i + 1)];
      const tangent = this.vecNorm(this.vecSub(pNext, pPrev));
      const upGuide = Math.abs(tangent[1]) < 0.88 ? [0, 1, 0] : [1, 0, 0];
      const u = this.vecNorm(this.vecCross(tangent, upGuide));
      const v = this.vecNorm(this.vecCross(tangent, u));

      const tHeight = Math.max(0, Math.min(1, points[i][1] / maxH));
      const col = [
        colBottom[0] + (colTop[0] - colBottom[0]) * tHeight,
        colBottom[1] + (colTop[1] - colBottom[1]) * tHeight,
        colBottom[2] + (colTop[2] - colBottom[2]) * tHeight
      ];

      const ring = [];
      for (let j = 0; j < sides; j++) {
        const ang = (2 * Math.PI * j) / sides;
        const c = Math.cos(ang), s = Math.sin(ang);
        const barkFurrow = 1.0 + roughness * Math.sin(ang * 5.0 + i * 1.7);
        const r = radii[i] * barkFurrow;
        const n = this.vecNorm([u[0] * c + v[0] * s, u[1] * c + v[1] * s, u[2] * c + v[2] * s]);
        const pos = [
          points[i][0] + n[0] * r,
          points[i][1] + n[1] * r,
          points[i][2] + n[2] * r
        ];
        // Ribbon / mottled bark variation
        const mottle = 0.92 + 0.16 * Math.sin(ang * 3.0 + points[i][1] * 2.3);
        const vCol = [
          Math.min(1, col[0] * mottle),
          Math.min(1, col[1] * mottle),
          Math.min(1, col[2] * mottle)
        ];
        const wind = [tHeight, partType === 0 ? tHeight * 0.6 : 0.85, partType === 0 ? 0.05 : 0.25, partType];
        ring.push(this.addVertex(b, pos, n, vCol, wind));
      }
      rings.push(ring);
    }

    for (let i = 0; i < rings.length - 1; i++) {
      for (let j = 0; j < sides; j++) {
        const jNext = (j + 1) % sides;
        const a = rings[i][j], c = rings[i][jNext];
        const d = rings[i + 1][j], e = rings[i + 1][jNext];
        b.indices.push(a, d, c, c, d, e);
      }
    }
  }

  /**
   * Cescatti asymmetric foliage cluster + phyllotactic leaf blades
   */
  addCanopyCluster(b, rng, center, rx, ry, rz, leafCol, maxH, isHero = false) {
    const lat = isHero ? 6 : 4;
    const lon = isHero ? 8 : 6;
    const rings = [];
    const hNorm = Math.max(0.1, Math.min(1.0, center[1] / maxH));

    for (let i = 0; i <= lat; i++) {
      const theta = (Math.PI * i) / lat;
      const sinT = Math.sin(theta), cosT = Math.cos(theta);
      const ring = [];
      for (let j = 0; j < lon; j++) {
        const phi = (2 * Math.PI * j) / lon;
        const lobe = 0.86 + 0.22 * Math.sin(phi * 3 + theta * 2) + 0.12 * (rng() - 0.5);
        const nx = sinT * Math.cos(phi);
        const ny = cosT;
        const nz = sinT * Math.sin(phi);
        const pos = [
          center[0] + nx * rx * lobe,
          center[1] + ny * ry * (ny > 0 ? 1.05 : 0.65) * lobe,
          center[2] + nz * rz * lobe
        ];
        const shade = 0.78 + 0.28 * (0.5 + 0.5 * ny) + 0.08 * (rng() - 0.5);
        const col = [
          Math.min(1, leafCol[0] * shade),
          Math.min(1, leafCol[1] * shade),
          Math.min(1, leafCol[2] * shade)
        ];
        ring.push(this.addVertex(b, pos, this.vecNorm([nx, ny, nz]), col, [hNorm, 0.9, 0.85, 2]));
      }
      rings.push(ring);
    }

    for (let i = 0; i < lat; i++) {
      for (let j = 0; j < lon; j++) {
        const jn = (j + 1) % lon;
        b.indices.push(
          rings[i][j], rings[i + 1][j], rings[i + 1][jn],
          rings[i][j], rings[i + 1][jn], rings[i][jn]
        );
      }
    }

    // Add drooping falcate leaf blades around cluster perimeter
    const leafCount = isHero ? 18 : 6;
    for (let k = 0; k < leafCount; k++) {
      const ang = (k * 2.39996) + rng() * 0.5;
      const rDist = (0.65 + 0.35 * rng());
      const base = [
        center[0] + Math.cos(ang) * rx * rDist,
        center[1] + (rng() - 0.35) * ry * 0.8,
        center[2] + Math.sin(ang) * rz * rDist
      ];
      const len = (isHero ? 0.45 : 0.55) * (0.8 + 0.4 * rng());
      const tip = [
        base[0] + Math.cos(ang) * len * 0.45,
        base[1] - len * 0.85,
        base[2] + Math.sin(ang) * len * 0.45
      ];
      this.addLeafBlade(b, base, tip, len * 0.22, leafCol, hNorm);
    }
  }

  addLeafBlade(b, base, tip, width, col, hNorm) {
    const axis = this.vecNorm(this.vecSub(tip, base));
    let side = this.vecCross(axis, [0, 1, 0]);
    if (Math.hypot(...side) < 0.2) side = [1, 0, 0];
    side = this.vecNorm(side);
    const n = this.vecNorm(this.vecCross(side, axis));
    const mid = [
      base[0] + (tip[0] - base[0]) * 0.45,
      base[1] + (tip[1] - base[1]) * 0.45,
      base[2] + (tip[2] - base[2]) * 0.45
    ];
    const wind = [hNorm, 0.95, 1.0, 2];
    const p0 = this.addVertex(b, base, n, col, wind);
    const p1 = this.addVertex(b, this.vecAdd(mid, side, width), n, [col[0] * 1.08, col[1] * 1.08, col[2] * 1.05], wind);
    const p2 = this.addVertex(b, tip, n, [col[0] * 0.9, col[1] * 0.95, col[2] * 0.9], wind);
    const p3 = this.addVertex(b, this.vecAdd(mid, side, -width), n, col, wind);
    b.indices.push(p0, p1, p2, p0, p2, p3, p2, p1, p0, p3, p2, p0);
  }

  addFlowerSpikeOrCluster(b, center, radius, height, col, hNorm) {
    const pBot = [center[0], center[1] - height * 0.5, center[2]];
    const pTop = [center[0], center[1] + height * 0.5, center[2]];
    this.addSweptTube(b, [pBot, center, pTop], [radius * 0.7, radius, radius * 0.6], col, [
      Math.min(1, col[0] * 1.15),
      Math.min(1, col[1] * 1.15),
      Math.min(1, col[2] * 1.15)
    ], 6, 10.0, 3, 0.15);
  }

  // --- 18 Botanical Architecture Generators ---

  buildContortedGum(b, rng, H, barkCol, barkSecCol, leafCol, flowerCol, isHero) {
    // Snow Gum (Eucalyptus pauciflora): multi-leader wind-sheared contorted trunks
    const stems = 2 + Math.floor(rng() * 2);
    const sides = isHero ? 10 : 6;
    for (let s = 0; s < stems; s++) {
      const baseAng = (2 * Math.PI * s) / stems + rng() * 0.4;
      const lean = (0.18 + rng() * 0.16) * H;
      const p0 = [Math.cos(baseAng) * 0.03 * H, 0, Math.sin(baseAng) * 0.03 * H];
      const p1 = [Math.cos(baseAng) * lean * 0.4, H * 0.22, Math.sin(baseAng) * lean * 0.4];
      const p2 = [Math.cos(baseAng + 0.5) * lean * 0.85, H * 0.48, Math.sin(baseAng + 0.5) * lean * 0.85];
      const p3 = [Math.cos(baseAng + 0.2) * lean * 1.1, H * 0.72, Math.sin(baseAng + 0.2) * lean * 1.1];
      this.addSweptTube(b, [p0, p1, p2, p3], [0.055 * H, 0.042 * H, 0.028 * H, 0.014 * H], barkCol, barkSecCol, sides, H, 0, 0.12);

      // Primary twisted sub-branches
      const bCount = isHero ? 4 : 3;
      for (let k = 0; k < bCount; k++) {
        const bAng = baseAng + (k - 1) * 1.1 + (rng() - 0.5) * 0.4;
        const fork = k === 0 ? p2 : p3;
        const reach = H * (0.22 + 0.12 * rng());
        const tip = [
          fork[0] + Math.cos(bAng) * reach,
          fork[1] + H * (0.12 + 0.08 * rng()),
          fork[2] + Math.sin(bAng) * reach
        ];
        const mid = [
          (fork[0] + tip[0]) * 0.5,
          (fork[1] + tip[1]) * 0.5 + H * 0.05,
          (fork[2] + tip[2]) * 0.5
        ];
        this.addSweptTube(b, [fork, mid, tip], [0.018 * H, 0.011 * H, 0.005 * H], barkSecCol, barkCol, Math.max(5, sides - 2), H, 1, 0.08);
        this.addCanopyCluster(b, rng, tip, H * 0.19, H * 0.09, H * 0.18, leafCol, H, isHero);
        if (k % 2 === 0) {
          this.addFlowerSpikeOrCluster(b, [tip[0], tip[1] + H * 0.05, tip[2]], H * 0.025, H * 0.04, flowerCol, 0.9);
        }
      }
    }
  }

  buildColumnarAsh(b, rng, H, barkCol, barkSecCol, leafCol, flowerCol, isHero) {
    // Alpine Ash / Mountain Ash (E. delegatensis / E. regnans): tall straight buttressed trunk
    const sides = isHero ? 12 : 7;
    const trunkPts = [
      [0, 0, 0],
      [0.008 * H, 0.20 * H, 0],
      [-0.006 * H, 0.45 * H, 0.008 * H],
      [0.010 * H, 0.68 * H, -0.005 * H],
      [0.0, 0.86 * H, 0.0]
    ];
    const radii = [0.068 * H, 0.044 * H, 0.034 * H, 0.024 * H, 0.011 * H];
    this.addSweptTube(b, trunkPts, radii, barkCol, barkSecCol, sides, H, 0, 0.10);

    const bCount = isHero ? 8 : 5;
    for (let k = 0; k < bCount; k++) {
      const ang = k * 2.39996 + rng() * 0.3;
      const yFork = H * (0.55 + 0.055 * k);
      const reach = H * (0.18 + 0.10 * ((k % 3) / 2));
      const fork = [0, yFork, 0];
      const elbow = [Math.cos(ang) * reach * 0.55, yFork + H * 0.06, Math.sin(ang) * reach * 0.55];
      const tip = [Math.cos(ang) * reach, yFork + H * 0.12, Math.sin(ang) * reach];
      this.addSweptTube(b, [fork, elbow, tip], [0.016 * H, 0.010 * H, 0.004 * H], barkSecCol, barkSecCol, 5, H, 1, 0.05);
      this.addCanopyCluster(b, rng, tip, H * 0.16, H * 0.08, H * 0.15, leafCol, H, isHero);
    }
  }

  buildSpreadingEucalypt(b, rng, H, barkCol, barkSecCol, leafCol, flowerCol, isHero) {
    const sides = isHero ? 10 : 6;
    const trunkPts = [
      [0, 0, 0],
      [0.02 * H, 0.22 * H, 0.01 * H],
      [0.03 * H, 0.44 * H, -0.01 * H],
      [0.01 * H, 0.62 * H, 0.01 * H]
    ];
    this.addSweptTube(b, trunkPts, [0.062 * H, 0.046 * H, 0.034 * H, 0.018 * H], barkCol, barkSecCol, sides, H, 0, 0.09);

    const bCount = isHero ? 7 : 5;
    for (let k = 0; k < bCount; k++) {
      const ang = k * 2.39996 + rng() * 0.4;
      const yFork = H * (0.36 + 0.05 * k);
      const reach = H * (0.24 + 0.10 * rng());
      const fork = [0.02 * H, yFork, 0];
      const elbow = [Math.cos(ang) * reach * 0.5, yFork + H * 0.10, Math.sin(ang) * reach * 0.5];
      const tip = [Math.cos(ang) * reach, yFork + H * 0.18, Math.sin(ang) * reach];
      this.addSweptTube(b, [fork, elbow, tip], [0.020 * H, 0.012 * H, 0.005 * H], barkSecCol, barkCol, 5, H, 1, 0.06);
      this.addCanopyCluster(b, rng, tip, H * 0.20, H * 0.10, H * 0.19, leafCol, H, isHero);
      if (k % 2 === 0) {
        this.addFlowerSpikeOrCluster(b, [tip[0], tip[1] + H * 0.04, tip[2]], H * 0.028, H * 0.05, flowerCol, 0.85);
      }
    }
  }

  buildWeepingRiparian(b, rng, H, barkCol, leafCol, flowerCol, isHero) {
    // Callistemon / Melaleuca / Leptospermum: arching multi-stem riparian shrub with crimson/gold bottlebrush spikes
    const stems = isHero ? 6 : 4;
    for (let s = 0; s < stems; s++) {
      const ang = (2 * Math.PI * s) / stems + rng() * 0.4;
      const reach = H * (0.35 + 0.15 * rng());
      const p0 = [0, 0, 0];
      const p1 = [Math.cos(ang) * reach * 0.4, H * 0.42, Math.sin(ang) * reach * 0.4];
      const p2 = [Math.cos(ang) * reach * 0.85, H * 0.72, Math.sin(ang) * reach * 0.85];
      const p3 = [Math.cos(ang) * reach * 1.15, H * 0.58, Math.sin(ang) * reach * 1.15]; // weeping tip
      this.addSweptTube(b, [p0, p1, p2, p3], [0.032 * H, 0.022 * H, 0.012 * H, 0.005 * H], barkCol, barkCol, 5, H, 1, 0.08);
      this.addCanopyCluster(b, rng, p2, H * 0.18, H * 0.12, H * 0.18, leafCol, H, isHero);
      this.addFlowerSpikeOrCluster(b, p3, H * 0.035, H * 0.11, flowerCol, 0.9);
      this.addFlowerSpikeOrCluster(b, [p2[0] * 1.05, p2[1] + H * 0.06, p2[2] * 1.05], H * 0.032, H * 0.10, flowerCol, 0.9);
    }
  }

  buildWattleDome(b, rng, H, barkCol, leafCol, flowerCol, isHero) {
    // Acacia: domed multi-branch shrub/tree bursting with golden globular racemes
    this.addSweptTube(b, [[0, 0, 0], [0, H * 0.28, 0], [0.02 * H, H * 0.52, 0]], [0.045 * H, 0.032 * H, 0.018 * H], barkCol, barkCol, 6, H, 0, 0.05);
    const branches = isHero ? 7 : 5;
    for (let k = 0; k < branches; k++) {
      const ang = k * 2.39996;
      const reach = H * (0.28 + 0.12 * (k % 3));
      const y = H * (0.42 + 0.07 * k);
      const fork = [0, Math.min(H * 0.5, y * 0.7), 0];
      const tip = [Math.cos(ang) * reach, y, Math.sin(ang) * reach];
      this.addSweptTube(b, [fork, tip], [0.016 * H, 0.006 * H], barkCol, barkCol, 5, H, 1, 0.04);
      this.addCanopyCluster(b, rng, tip, H * 0.20, H * 0.13, H * 0.20, leafCol, H, isHero);
      // Golden wattle puffball clusters
      for (let f = 0; f < 3; f++) {
        const fa = ang + (f - 1) * 0.5;
        const fp = [
          tip[0] + Math.cos(fa) * H * 0.12,
          tip[1] + H * (0.04 + 0.03 * f),
          tip[2] + Math.sin(fa) * H * 0.12
        ];
        this.addFlowerSpikeOrCluster(b, fp, H * 0.045, H * 0.05, flowerCol, 0.95);
      }
    }
  }

  buildBanksiaOrWaratah(b, rng, H, barkCol, leafCol, flowerCol, isWaratah, isHero) {
    // Gnarled Proteaceae shrub/tree with upright serotinous cones or crimson waratah heads
    const p0 = [0, 0, 0];
    const p1 = [0.03 * H, 0.30 * H, -0.02 * H];
    const p2 = [-0.02 * H, 0.58 * H, 0.03 * H];
    this.addSweptTube(b, [p0, p1, p2], [0.055 * H, 0.040 * H, 0.022 * H], barkCol, barkCol, 6, H, 0, 0.14);

    const bCount = isHero ? 6 : 4;
    for (let k = 0; k < bCount; k++) {
      const ang = (2 * Math.PI * k) / bCount + 0.3;
      const reach = H * (0.22 + 0.10 * rng());
      const y = H * (0.45 + 0.08 * k);
      const tip = [Math.cos(ang) * reach, y, Math.sin(ang) * reach];
      this.addSweptTube(b, [p1, tip], [0.018 * H, 0.008 * H], barkCol, barkCol, 5, H, 1, 0.12);
      this.addCanopyCluster(b, rng, tip, H * 0.16, H * 0.09, H * 0.16, leafCol, H, isHero);
      // Upright cylindrical Banksia candle cone or Waratah dome
      const spikeH = isWaratah ? H * 0.09 : H * 0.14;
      const spikeR = isWaratah ? H * 0.055 : H * 0.038;
      this.addFlowerSpikeOrCluster(b, [tip[0], tip[1] + H * 0.08, tip[2]], spikeR, spikeH, flowerCol, 0.92);
    }
  }

  buildConiferPyramidal(b, rng, H, barkCol, leafCol, flowerCol, isHero) {
    // Wollemi Pine / Hoop Pine / Bunya Pine / Casuarina
    this.addSweptTube(b, [[0, 0, 0], [0, H * 0.5, 0], [0, H * 0.96, 0]], [0.055 * H, 0.035 * H, 0.008 * H], barkCol, barkCol, 7, H, 0, 0.08);
    const tiers = isHero ? 6 : 4;
    for (let t = 0; t < tiers; t++) {
      const y = H * (0.25 + (t / tiers) * 0.68);
      const tierRad = H * (0.32 * (1.0 - (t / tiers) * 0.65));
      this.addCanopyCluster(b, rng, [0, y, 0], tierRad, H * 0.08, tierRad, leafCol, H, isHero);
    }
  }

  buildRainforestButtress(b, rng, H, barkCol, barkSecCol, leafCol, flowerCol, isHero) {
    // Moreton Bay Fig / Nothofagus / Coachwood with flanged plank buttress roots
    this.addSweptTube(b, [[0, 0, 0], [0, H * 0.35, 0], [0, H * 0.65, 0]], [0.095 * H, 0.055 * H, 0.030 * H], barkCol, barkSecCol, 8, H, 0, 0.15);
    for (let r = 0; r < 5; r++) {
      const ang = (2 * Math.PI * r) / 5;
      const pButt = [Math.cos(ang) * H * 0.18, 0, Math.sin(ang) * H * 0.18];
      const pJoin = [Math.cos(ang) * H * 0.04, H * 0.26, Math.sin(ang) * H * 0.04];
      this.addSweptTube(b, [pButt, pJoin], [0.025 * H, 0.015 * H], barkCol, barkSecCol, 4, H, 0, 0.05);
    }
    const crowns = isHero ? 6 : 4;
    for (let k = 0; k < crowns; k++) {
      const ang = (2 * Math.PI * k) / crowns;
      const reach = H * 0.28;
      const tip = [Math.cos(ang) * reach, H * (0.68 + 0.05 * (k % 2)), Math.sin(ang) * reach];
      this.addSweptTube(b, [[0, H * 0.48, 0], tip], [0.024 * H, 0.010 * H], barkSecCol, barkCol, 5, H, 1, 0.06);
      this.addCanopyCluster(b, rng, tip, H * 0.24, H * 0.12, H * 0.24, leafCol, H, isHero);
    }
    this.addCanopyCluster(b, rng, [0, H * 0.85, 0], H * 0.28, H * 0.14, H * 0.28, leafCol, H, isHero);
  }

  buildTreeFern(b, rng, H, barkCol, leafCol, isHero) {
    // Dicksonia antarctica / Cyathea australis: fibrous caudex + radial arching frond crown
    const trunkH = H * 0.68;
    this.addSweptTube(b, [[0, 0, 0], [0, trunkH * 0.5, 0], [0, trunkH, 0]], [0.065 * H, 0.055 * H, 0.050 * H], barkCol, barkCol, 7, H, 0, 0.18);
    const frondCount = isHero ? 14 : 9;
    for (let f = 0; f < frondCount; f++) {
      const ang = (2 * Math.PI * f) / frondCount + (rng() - 0.5) * 0.15;
      const len = H * (0.42 + 0.08 * (f % 2));
      const p0 = [0, trunkH, 0];
      const p1 = [Math.cos(ang) * len * 0.5, trunkH + H * 0.18, Math.sin(ang) * len * 0.5];
      const p2 = [Math.cos(ang) * len, trunkH + H * 0.05, Math.sin(ang) * len];
      this.addSweptTube(b, [p0, p1, p2], [0.012 * H, 0.008 * H, 0.002 * H], leafCol, leafCol, 4, H, 2, 0.02);
      this.addLeafBlade(b, p0, p2, len * 0.24, leafCol, 0.9);
    }
  }

  buildGrassTree(b, rng, H, barkCol, leafCol, flowerCol, isHero) {
    // Xanthorrhoea australis: charred black caudex + dense grassy skirt + tall kangaroo-tail spike
    const caudexH = H * 0.42;
    this.addSweptTube(b, [[0, 0, 0], [0, caudexH, 0]], [0.075 * H, 0.065 * H], [0.14, 0.12, 0.11], barkCol, 7, H, 0, 0.12);
    this.addCanopyCluster(b, rng, [0, caudexH + H * 0.06, 0], H * 0.28, H * 0.18, H * 0.28, leafCol, H, isHero);
    // Tall central flowering spear
    this.addSweptTube(b, [[0, caudexH, 0], [0, H * 0.98, 0]], [0.018 * H, 0.012 * H], barkCol, flowerCol, 6, H, 3, 0.04);
  }

  buildPalmOrCycad(b, rng, H, barkCol, leafCol, isCycad, isHero) {
    const trunkH = isCycad ? H * 0.32 : H * 0.75;
    const trunkR = isCycad ? 0.085 * H : 0.038 * H;
    this.addSweptTube(b, [[0, 0, 0], [0, trunkH * 0.5, 0], [0, trunkH, 0]], [trunkR * 1.15, trunkR, trunkR * 0.9], barkCol, barkCol, 7, H, 0, 0.10);
    const fronds = isHero ? 12 : 8;
    for (let f = 0; f < fronds; f++) {
      const ang = (2 * Math.PI * f) / fronds;
      const len = H * (isCycad ? 0.52 : 0.36);
      const base = [0, trunkH, 0];
      const tip = [Math.cos(ang) * len, trunkH + H * (f % 2 === 0 ? 0.12 : -0.04), Math.sin(ang) * len];
      this.addLeafBlade(b, base, tip, len * 0.22, leafCol, 0.92);
    }
  }

  buildTussockOrReed(b, rng, H, leafCol, flowerCol, isReed, isHero) {
    const blades = isHero ? 26 : 12;
    const hScale = Math.max(0.7, H * 1.8);
    for (let i = 0; i < blades; i++) {
      const ang = (2 * Math.PI * i) / blades + (rng() - 0.5) * 0.3;
      const r0 = rng() * 0.12 * hScale;
      const spread = (isReed ? 0.18 : 0.55) * hScale * (0.6 + 0.5 * rng());
      const bh = hScale * (0.65 + 0.45 * rng());
      const base = [Math.cos(ang) * r0, 0, Math.sin(ang) * r0];
      const tip = [Math.cos(ang) * spread, bh, Math.sin(ang) * spread];
      this.addLeafBlade(b, base, tip, hScale * (isReed ? 0.06 : 0.08), leafCol, 0.95);
      if (i % 4 === 0) {
        this.addFlowerSpikeOrCluster(b, [tip[0], tip[1] + 0.08 * hScale, tip[2]], 0.03 * hScale, 0.14 * hScale, flowerCol, 1.0);
      }
    }
  }

  buildAlpineHerbOrCushion(b, rng, H, leafCol, flowerCol, isCushion, isHero) {
    const hScale = Math.max(0.65, H * 2.0);
    // Basal rosette or cushion mound
    this.addCanopyCluster(b, rng, [0, hScale * 0.18, 0], hScale * 0.42, hScale * 0.20, hScale * 0.42, leafCol, hScale, isHero);
    const stalks = isCushion ? 5 : 4;
    for (let s = 0; s < stalks; s++) {
      const ang = (2 * Math.PI * s) / stalks + 0.4;
      const r = s === 0 ? 0 : hScale * 0.22;
      const x = Math.cos(ang) * r, z = Math.sin(ang) * r;
      const topY = hScale * (0.55 + 0.15 * (s % 2));
      this.addSweptTube(b, [[x, hScale * 0.1, z], [x, topY, z]], [0.025 * hScale, 0.015 * hScale], leafCol, leafCol, 4, hScale, 2, 0.0);
      this.addFlowerSpikeOrCluster(b, [x, topY + 0.04 * hScale, z], 0.09 * hScale, 0.07 * hScale, flowerCol, 1.0);
    }
  }

  buildMangrove(b, rng, H, barkCol, leafCol, isHero) {
    // Prop roots + dense salt-tolerant crown
    this.addSweptTube(b, [[0, H * 0.2, 0], [0, H * 0.65, 0]], [0.048 * H, 0.025 * H], barkCol, barkCol, 6, H, 0, 0.08);
    for (let r = 0; r < 6; r++) {
      const ang = (2 * Math.PI * r) / 6;
      this.addSweptTube(b, [[Math.cos(ang) * H * 0.22, 0, Math.sin(ang) * H * 0.22], [0, H * 0.28, 0]], [0.015 * H, 0.018 * H], barkCol, barkCol, 4, H, 0, 0.04);
    }
    this.addCanopyCluster(b, rng, [0, H * 0.72, 0], H * 0.34, H * 0.18, H * 0.34, leafCol, H, isHero);
  }

  buildRootVascularWireframe(b, rng, H, rootDepthM) {
    const depth = Math.min(H * 0.45, Math.max(0.8, rootDepthM * 0.35));
    // Taproot
    this.addWireLine(b, [0, 0, 0], [0, -depth, 0]);
    // Lateral sinker roots & mycorrhizal feeder fan
    for (let r = 0; r < 6; r++) {
      const ang = (2 * Math.PI * r) / 6 + 0.2;
      const pMid = [Math.cos(ang) * H * 0.14, -depth * 0.3, Math.sin(ang) * H * 0.14];
      const pEnd = [Math.cos(ang) * H * 0.26, -depth * 0.75, Math.sin(ang) * H * 0.26];
      this.addWireLine(b, [0, -0.05 * H, 0], pMid);
      this.addWireLine(b, pMid, pEnd);
    }
  }
}

window.FloraEngine = FloraEngine;
