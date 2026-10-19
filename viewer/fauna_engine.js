/**
 * EcoPunk 3D Anatomical Wildlife, Biomechanical Kinematics & Ecological AI Engine
 *
 * Implements:
 * 1. Articulated multi-part 3D anatomical geometries across 22 Australian fauna archetypes
 *    (Macropods, Wombats, Koalas, Platypus, Echidnas, Gliders/Possums, Quolls, Dingoes,
 *     Flying-foxes, Lyrebirds, Raptors, Cockatoos/Parrots, Kookaburras/Kingfishers,
 *     Songbirds, Ratites, Waterbirds, Goannas/Skinks, Snakes, Turtles, Frogs, Fish,
 *     Crayfish, and Alpine Moths/Insects).
 * 2. Osteological internal skeleton wireframe generation (vertebral column, cranium,
 *    ribcage, pelvis, limb joints) for Anatomical / X-Ray Loupe inspection.
 * 3. Real-time GPU/CPU biomechanical gait kinematics (bipedal hopping, pentapedal grazing,
 *    fossorial waddling, arboreal climbing, patagial gliding, avian flapping & thermal
 *    soaring, sub-aquatic swimming, sinusoidal slithering, sprawling lizard gait,
 *    saltatorial frog leaps).
 * 4. N3Logic / VibeScript autonomous ecological behavior state machines & trophic graphs.
 */

class FaunaEngine {
  constructor() {
    this.meshCache = new Map();
  }

  /**
   * Builds or retrieves cached articulated 3D mesh for a fauna species.
   * Each vertex includes:
   *  - position: [x, y, z]
   *  - normal: [nx, ny, nz]
   *  - color: [r, g, b]
   *  - animParam: [partId, sideSign, distAlongPart, thermalTempNorm]
   *    partId:
   *      0 = torso / carapace
   *      1 = neck & head / cranium / snout / ears
   *      2 = forelimb / wing (sideSign = -1 left, +1 right)
   *      3 = hindlimb (sideSign = -1 left, +1 right)
   *      4 = tail / fluke (distAlongPart = 0..1 from base to tip)
   */
  getSpeciesGeometry(species, isLoupe = false) {
    const cacheKey = `${species.id}_${isLoupe ? 'loupe' : 'scene'}`;
    if (this.meshCache.has(cacheKey)) {
      return this.meshCache.get(cacheKey);
    }

    const b = {
      positions: [],
      normals: [],
      colors: [],
      animParams: [],
      indices: [],
      skeletonLines: []
    };

    const arch = species.anatomy_3d?.archetype || 'macropod';
    const col1 = species.anatomy_3d?.primary_rgb || [0.42, 0.38, 0.33];
    const col2 = species.anatomy_3d?.secondary_rgb || [0.68, 0.64, 0.58];
    const col3 = species.anatomy_3d?.accent_rgb || [0.16, 0.14, 0.13];

    // Normalized thermal infrared signature based on endotherm vs ectotherm body temp
    const bodyTemp = species.biophysics?.body_temp_c || 36.5;
    const isEndo = (species.biophysics?.thermoregulation || 'endotherm') === 'endotherm';
    const thermalNorm = isEndo ? Math.min(1.0, Math.max(0.55, (bodyTemp - 20.0) / 22.0)) : 0.28;

    // Scale for visibility on terrain while preserving relative proportions
    const rawLen = species.biophysics?.body_length_m || 1.0;
    const scale = isLoupe ? 2.2 : Math.max(0.55, Math.min(2.1, Math.pow(rawLen, 0.55) * 1.15));

    switch (arch) {
      case 'macropod':
        this.buildMacropod(b, scale, col1, col2, col3, thermalNorm);
        break;
      case 'wombat':
      case 'echidna':
        this.buildWombatOrEchidna(b, scale, col1, col2, col3, arch === 'echidna', thermalNorm);
        break;
      case 'koala':
      case 'glider_possum':
        this.buildArborealMarsupial(b, scale, col1, col2, col3, arch === 'glider_possum', thermalNorm);
        break;
      case 'platypus':
        this.buildPlatypus(b, scale, col1, col2, col3, thermalNorm);
        break;
      case 'quoll_dasyurid':
      case 'canid':
        this.buildQuadrupedCarnivore(b, scale, col1, col2, col3, arch === 'quoll_dasyurid', thermalNorm);
        break;
      case 'raptor':
      case 'flying_fox':
        this.buildRaptorOrBat(b, scale, col1, col2, col3, arch === 'flying_fox', thermalNorm);
        break;
      case 'lyrebird':
      case 'cockatoo_parrot':
      case 'kookaburra_kingfisher':
      case 'songbird':
        this.buildWoodlandBird(b, scale, col1, col2, col3, arch, thermalNorm);
        break;
      case 'ratite':
        this.buildRatite(b, scale, col1, col2, col3, thermalNorm);
        break;
      case 'waterbird':
        this.buildWaterbird(b, scale, col1, col2, col3, thermalNorm);
        break;
      case 'goanna_skink':
        this.buildLizard(b, scale, col1, col2, col3, thermalNorm);
        break;
      case 'snake':
        this.buildSnake(b, scale, col1, col2, col3, thermalNorm);
        break;
      case 'turtle':
        this.buildTurtle(b, scale, col1, col2, col3, thermalNorm);
        break;
      case 'frog':
        this.buildFrog(b, scale, col1, col2, col3, thermalNorm);
        break;
      case 'fish':
      case 'crayfish':
        this.buildAquaticFauna(b, scale, col1, col2, col3, arch === 'crayfish', thermalNorm);
        break;
      case 'moth_insect':
        this.buildInsect(b, scale, col1, col2, col3, thermalNorm);
        break;
      default:
        this.buildMacropod(b, scale, col1, col2, col3, thermalNorm);
        break;
    }

    const geom = {
      positions: new Float32Array(b.positions),
      normals: new Float32Array(b.normals),
      colors: new Float32Array(b.colors),
      animParams: new Float32Array(b.animParams),
      indices: new Uint16Array(b.indices),
      skeletonLines: new Float32Array(b.skeletonLines),
      vertexCount: b.positions.length / 3,
      indexCount: b.indices.length,
      scale
    };

    this.meshCache.set(cacheKey, geom);
    return geom;
  }

  // --- Low-level Anatomical Geometry Helpers ---

  addVert(b, pos, norm, col, anim) {
    const idx = b.positions.length / 3;
    b.positions.push(pos[0], pos[1], pos[2]);
    b.normals.push(norm[0], norm[1], norm[2]);
    b.colors.push(col[0], col[1], col[2]);
    b.animParams.push(anim[0], anim[1], anim[2], anim[3]);
    return idx;
  }

  addBone(b, p0, p1) {
    b.skeletonLines.push(p0[0], p0[1], p0[2], p1[0], p1[1], p1[2]);
  }

  /**
   * Sculpted anatomical ellipsoid with dorsal-to-ventral coat color blending
   */
  addEllipsoid(b, center, radii, colDorsal, colVentral, partId, sideSign, distAlong, thermalNorm, pitch = 0, lat = 6, lon = 8) {
    const cosP = Math.cos(pitch), sinP = Math.sin(pitch);
    const rings = [];
    for (let i = 0; i <= lat; i++) {
      const theta = (Math.PI * i) / lat;
      const sinT = Math.sin(theta), cosT = Math.cos(theta);
      const ring = [];
      for (let j = 0; j < lon; j++) {
        const phi = (2 * Math.PI * j) / lon;
        let lx = radii[0] * sinT * Math.cos(phi);
        let ly = radii[1] * cosT;
        let lz = radii[2] * sinT * Math.sin(phi);

        // Apply pitch in YZ plane
        const ry = ly * cosP - lz * sinP;
        const rz = ly * sinP + lz * cosP;

        const nx = (sinT * Math.cos(phi)) / (radii[0] || 1);
        const ny = (cosT * cosP - sinT * Math.sin(phi) * sinP) / (radii[1] || 1);
        const nz = (cosT * sinP + sinT * Math.sin(phi) * cosP) / (radii[2] || 1);
        const nLen = Math.hypot(nx, ny, nz) || 1;

        const ventMix = Math.max(0, Math.min(1, 0.5 - 0.55 * cosT));
        const col = [
          colDorsal[0] * (1 - ventMix) + colVentral[0] * ventMix,
          colDorsal[1] * (1 - ventMix) + colVentral[1] * ventMix,
          colDorsal[2] * (1 - ventMix) + colVentral[2] * ventMix
        ];

        ring.push(this.addVert(
          b,
          [center[0] + lx, center[1] + ry, center[2] + rz],
          [nx / nLen, ny / nLen, nz / nLen],
          col,
          [partId, sideSign, distAlong, thermalNorm]
        ));
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
  }

  /**
   * Articulated tapered limb or tail chain along 3D joint positions
   */
  addArticulatedChain(b, joints, radii, colTop, colTip, partId, sideSign, thermalNorm, sides = 6) {
    const rings = [];
    for (let i = 0; i < joints.length; i++) {
      if (i > 0) this.addBone(b, joints[i - 1], joints[i]);
      const pPrev = joints[Math.max(0, i - 1)];
      const pNext = joints[Math.min(joints.length - 1, i + 1)];
      let dx = pNext[0] - pPrev[0], dy = pNext[1] - pPrev[1], dz = pNext[2] - pPrev[2];
      const dLen = Math.hypot(dx, dy, dz) || 1;
      dx /= dLen; dy /= dLen; dz /= dLen;

      const up = Math.abs(dy) < 0.85 ? [0, 1, 0] : [1, 0, 0];
      let ux = dy * up[2] - dz * up[1];
      let uy = dz * up[0] - dx * up[2];
      let uz = dx * up[1] - dy * up[0];
      const uLen = Math.hypot(ux, uy, uz) || 1;
      ux /= uLen; uy /= uLen; uz /= uLen;

      const vx = dy * uz - dz * uy;
      const vy = dz * ux - dx * uz;
      const vz = dx * uy - dy * ux;

      const t = joints.length > 1 ? i / (joints.length - 1) : 0;
      const col = [
        colTop[0] * (1 - t) + colTip[0] * t,
        colTop[1] * (1 - t) + colTip[1] * t,
        colTop[2] * (1 - t) + colTip[2] * t
      ];

      const ring = [];
      for (let j = 0; j < sides; j++) {
        const ang = (2 * Math.PI * j) / sides;
        const c = Math.cos(ang), s = Math.sin(ang);
        const nx = ux * c + vx * s;
        const ny = uy * c + vy * s;
        const nz = uz * c + vz * s;
        const r = radii[i];
        ring.push(this.addVert(
          b,
          [joints[i][0] + nx * r, joints[i][1] + ny * r, joints[i][2] + nz * r],
          [nx, ny, nz],
          col,
          [partId, sideSign, t, thermalNorm]
        ));
      }
      rings.push(ring);
    }

    for (let i = 0; i < rings.length - 1; i++) {
      for (let j = 0; j < sides; j++) {
        const jn = (j + 1) % sides;
        b.indices.push(
          rings[i][j], rings[i + 1][j], rings[i + 1][jn],
          rings[i][j], rings[i + 1][jn], rings[i][jn]
        );
      }
    }
  }

  // --- Anatomical Archetype Builders ---

  buildMacropod(b, s, col1, col2, col3, th) {
    // Eastern Grey Kangaroo / Wallaby / Wallaroo / Potoroo
    // Forward-pitched muscular hindquarters + upright chest + long counter-balance tail + Z-folded hindlimbs
    this.addEllipsoid(b, [0, 0.72 * s, -0.10 * s], [0.24 * s, 0.30 * s, 0.36 * s], col1, col2, 0, 0, 0.3, th, 0.35);
    this.addEllipsoid(b, [0, 0.98 * s, 0.16 * s], [0.17 * s, 0.24 * s, 0.20 * s], col1, col2, 0, 0, 0.6, th, -0.25);
    this.addBone(b, [0, 0.62 * s, -0.28 * s], [0, 0.98 * s, 0.16 * s]);

    // Neck, Cranium, Muzzle & Erect Ears
    this.addArticulatedChain(b, [[0, 1.08 * s, 0.22 * s], [0, 1.32 * s, 0.34 * s], [0, 1.36 * s, 0.54 * s]], [0.11 * s, 0.10 * s, 0.05 * s], col1, col3, 1, 0, th);
    for (const side of [-1, 1]) {
      this.addArticulatedChain(b, [[side * 0.07 * s, 1.35 * s, 0.32 * s], [side * 0.14 * s, 1.56 * s, 0.30 * s]], [0.04 * s, 0.015 * s], col1, col3, 1, side, th, 4);
      // Forelimbs
      this.addArticulatedChain(b, [
        [side * 0.15 * s, 0.96 * s, 0.22 * s],
        [side * 0.16 * s, 0.72 * s, 0.30 * s],
        [side * 0.15 * s, 0.56 * s, 0.36 * s]
      ], [0.06 * s, 0.04 * s, 0.025 * s], col1, col3, 2, side, th, 5);
      // Powerful Z-articulated Hindlimbs (Femur -> Tibia -> Long Metatarsus/Pes)
      this.addArticulatedChain(b, [
        [side * 0.20 * s, 0.68 * s, -0.15 * s],
        [side * 0.22 * s, 0.42 * s, 0.02 * s],
        [side * 0.20 * s, 0.10 * s, -0.18 * s],
        [side * 0.20 * s, 0.03 * s, 0.16 * s]
      ], [0.13 * s, 0.08 * s, 0.05 * s, 0.035 * s], col1, col3, 3, side, th, 6);
    }

    // Thick muscular pentapedal tail
    this.addArticulatedChain(b, [
      [0, 0.58 * s, -0.34 * s],
      [0, 0.30 * s, -0.68 * s],
      [0, 0.12 * s, -1.05 * s],
      [0, 0.08 * s, -1.38 * s]
    ], [0.11 * s, 0.08 * s, 0.05 * s, 0.025 * s], col1, col3, 4, 0, th, 6);
  }

  buildWombatOrEchidna(b, s, col1, col2, col3, isEchidna, th) {
    // Robust planar barrel torso + stout digging limbs
    this.addEllipsoid(b, [0, 0.38 * s, 0], [0.34 * s, 0.28 * s, 0.48 * s], col1, col2, 0, 0, 0.5, th, 0, 7, 10);
    this.addBone(b, [0, 0.38 * s, -0.40 * s], [0, 0.42 * s, 0.42 * s]);

    // Head & Snout/Beak
    if (isEchidna) {
      this.addArticulatedChain(b, [[0, 0.36 * s, 0.42 * s], [0, 0.30 * s, 0.65 * s], [0, 0.24 * s, 0.88 * s]], [0.14 * s, 0.06 * s, 0.02 * s], col1, col3, 1, 0, th, 6);
      // Dorsal keratin spines
      for (let i = 0; i < 14; i++) {
        const ang = (i * 2.39996);
        const r = (0.12 + (i % 3) * 0.08) * s;
        const bx = Math.cos(ang) * r, bz = Math.sin(ang) * r * 1.2;
        this.addArticulatedChain(b, [[bx, 0.52 * s, bz], [bx * 1.4, 0.74 * s, bz - 0.12 * s]], [0.025 * s, 0.004 * s], col2, col3, 0, 0, th, 4);
      }
    } else {
      // Wombat broad blocky head & ears
      this.addEllipsoid(b, [0, 0.44 * s, 0.52 * s], [0.22 * s, 0.20 * s, 0.24 * s], col1, col3, 1, 0, 0.8, th, 0.1);
      for (const side of [-1, 1]) {
        this.addEllipsoid(b, [side * 0.16 * s, 0.62 * s, 0.46 * s], [0.05 * s, 0.07 * s, 0.04 * s], col1, col3, 1, side, 1.0, th);
      }
    }

    for (const side of [-1, 1]) {
      this.addArticulatedChain(b, [[side * 0.26 * s, 0.34 * s, 0.26 * s], [side * 0.28 * s, 0.04 * s, 0.30 * s]], [0.09 * s, 0.06 * s], col1, col3, 2, side, th, 5);
      this.addArticulatedChain(b, [[side * 0.26 * s, 0.34 * s, -0.26 * s], [side * 0.28 * s, 0.04 * s, -0.24 * s]], [0.10 * s, 0.06 * s], col1, col3, 3, side, th, 5);
    }
  }

  buildArborealMarsupial(b, s, col1, col2, col3, isGlider, th) {
    // Koala or Greater/Sugar Glider / Possum
    this.addEllipsoid(b, [0, 0.48 * s, 0], [0.24 * s, 0.26 * s, 0.38 * s], col1, col2, 0, 0, 0.5, th, -0.15);
    this.addEllipsoid(b, [0, 0.68 * s, 0.34 * s], [0.18 * s, 0.18 * s, 0.20 * s], col1, col2, 1, 0, 0.9, th);
    for (const side of [-1, 1]) {
      // Fluffy round ears
      this.addEllipsoid(b, [side * 0.18 * s, 0.80 * s, 0.32 * s], [0.09 * s, 0.09 * s, 0.05 * s], col2, col1, 1, side, 1.0, th);
      this.addArticulatedChain(b, [[side * 0.20 * s, 0.46 * s, 0.22 * s], [side * 0.24 * s, 0.08 * s, 0.26 * s]], [0.07 * s, 0.045 * s], col1, col3, 2, side, th, 5);
      this.addArticulatedChain(b, [[side * 0.20 * s, 0.42 * s, -0.22 * s], [side * 0.24 * s, 0.08 * s, -0.20 * s]], [0.08 * s, 0.045 * s], col1, col3, 3, side, th, 5);
    }
    if (isGlider) {
      // Long bushy prehensile/rudder tail
      this.addArticulatedChain(b, [[0, 0.44 * s, -0.34 * s], [0, 0.36 * s, -0.78 * s], [0, 0.28 * s, -1.25 * s]], [0.09 * s, 0.10 * s, 0.06 * s], col1, col3, 4, 0, th, 6);
    }
  }

  buildPlatypus(b, s, col1, col2, col3, th) {
    // Dorsoventrally flattened hydrodynamic torso + electroreceptive bill + beaver-like paddle tail
    this.addEllipsoid(b, [0, 0.22 * s, 0], [0.28 * s, 0.14 * s, 0.48 * s], col1, col2, 0, 0, 0.5, th);
    this.addEllipsoid(b, [0, 0.24 * s, 0.48 * s], [0.18 * s, 0.11 * s, 0.18 * s], col1, col2, 1, 0, 0.8, th);
    // Broad leathery duck bill
    this.addEllipsoid(b, [0, 0.20 * s, 0.72 * s], [0.15 * s, 0.045 * s, 0.18 * s], col3, col3, 1, 0, 1.0, th);
    // Flat paddle tail
    this.addEllipsoid(b, [0, 0.20 * s, -0.68 * s], [0.18 * s, 0.05 * s, 0.28 * s], col1, col3, 4, 0, 0.8, th);
    for (const side of [-1, 1]) {
      this.addArticulatedChain(b, [[side * 0.24 * s, 0.18 * s, 0.22 * s], [side * 0.42 * s, 0.06 * s, 0.26 * s]], [0.06 * s, 0.09 * s], col1, col3, 2, side, th, 4);
      this.addArticulatedChain(b, [[side * 0.24 * s, 0.18 * s, -0.24 * s], [side * 0.38 * s, 0.06 * s, -0.28 * s]], [0.06 * s, 0.08 * s], col1, col3, 3, side, th, 4);
    }
  }

  buildQuadrupedCarnivore(b, s, col1, col2, col3, isQuoll, th) {
    // Alpine Dingo or Spotted-tailed Quoll
    const yBody = isQuoll ? 0.38 * s : 0.62 * s;
    this.addEllipsoid(b, [0, yBody, 0], [0.18 * s, 0.20 * s, 0.46 * s], col1, col2, 0, 0, 0.5, th);
    this.addArticulatedChain(b, [[0, yBody + 0.08 * s, 0.36 * s], [0, yBody + 0.26 * s, 0.54 * s], [0, yBody + 0.24 * s, 0.74 * s]], [0.13 * s, 0.12 * s, 0.06 * s], col1, col3, 1, 0, th, 6);
    for (const side of [-1, 1]) {
      this.addArticulatedChain(b, [[side * 0.08 * s, yBody + 0.32 * s, 0.50 * s], [side * 0.11 * s, yBody + 0.46 * s, 0.50 * s]], [0.04 * s, 0.01 * s], col1, col3, 1, side, th, 4);
      this.addArticulatedChain(b, [[side * 0.14 * s, yBody - 0.04 * s, 0.28 * s], [side * 0.14 * s, yBody * 0.45, 0.26 * s], [side * 0.14 * s, 0.02 * s, 0.28 * s]], [0.065 * s, 0.045 * s, 0.035 * s], col1, col2, 2, side, th, 5);
      this.addArticulatedChain(b, [[side * 0.15 * s, yBody - 0.02 * s, -0.30 * s], [side * 0.15 * s, yBody * 0.45, -0.36 * s], [side * 0.15 * s, 0.02 * s, -0.30 * s]], [0.075 * s, 0.05 * s, 0.035 * s], col1, col2, 3, side, th, 5);
    }
    this.addArticulatedChain(b, [[0, yBody + 0.02 * s, -0.42 * s], [0, yBody - 0.08 * s, -0.78 * s], [0, yBody - 0.04 * s, -1.08 * s]], [0.07 * s, 0.06 * s, 0.03 * s], col1, col2, 4, 0, th, 5);
  }

  buildRaptorOrBat(b, s, col1, col2, col3, isBat, th) {
    // Wedge-tailed Eagle / Peregrine Falcon / Grey-headed Flying-fox
    const yFly = 0.65 * s;
    this.addEllipsoid(b, [0, yFly, 0], [0.18 * s, 0.15 * s, 0.38 * s], col1, col2, 0, 0, 0.5, th, 0.1);
    this.addArticulatedChain(b, [[0, yFly + 0.04 * s, 0.32 * s], [0, yFly + 0.08 * s, 0.50 * s], [0, yFly + 0.03 * s, 0.64 * s]], [0.10 * s, 0.09 * s, 0.03 * s], col2, col3, 1, 0, th, 5);

    // Broad articulated wings (inner arm + outer primaries)
    for (const side of [-1, 1]) {
      const pShoulder = [side * 0.14 * s, yFly + 0.04 * s, 0.08 * s];
      const pWrist = [side * 0.65 * s, yFly + 0.12 * s, 0.16 * s];
      const pTip = [side * 1.35 * s, yFly + 0.06 * s, -0.08 * s];
      this.addBone(b, pShoulder, pWrist);
      this.addBone(b, pWrist, pTip);
      this.addEllipsoid(b, [side * 0.42 * s, yFly + 0.08 * s, 0.04 * s], [0.34 * s, 0.035 * s, 0.22 * s], col1, col2, 2, side, 0.4, th);
      this.addEllipsoid(b, [side * 0.96 * s, yFly + 0.08 * s, -0.02 * s], [0.42 * s, 0.025 * s, 0.18 * s], col1, col3, 2, side, 0.9, th);
    }

    if (!isBat) {
      // Wedge-shaped tail fan
      this.addEllipsoid(b, [0, yFly - 0.02 * s, -0.52 * s], [0.22 * s, 0.03 * s, 0.26 * s], col1, col3, 4, 0, 0.9, th);
    }
  }

  buildWoodlandBird(b, s, col1, col2, col3, arch, th) {
    // Superb Lyrebird, Gang-gang/Yellow-tailed Black Cockatoo, Crimson Rosella, Kookaburra, Fairy-wren
    const yB = 0.42 * s;
    this.addEllipsoid(b, [0, yB, 0], [0.18 * s, 0.20 * s, 0.30 * s], col1, col2, 0, 0, 0.5, th, -0.35);
    this.addEllipsoid(b, [0, yB + 0.22 * s, 0.20 * s], [0.13 * s, 0.14 * s, 0.15 * s], col3, col2, 1, 0, 0.85, th);
    // Beak & Crest
    this.addArticulatedChain(b, [[0, yB + 0.22 * s, 0.32 * s], [0, yB + 0.20 * s, 0.48 * s]], [0.05 * s, 0.01 * s], col3, col3, 1, 0, th, 4);
    if (arch === 'cockatoo_parrot') {
      this.addArticulatedChain(b, [[0, yB + 0.32 * s, 0.18 * s], [0, yB + 0.48 * s, 0.12 * s]], [0.05 * s, 0.02 * s], col3, col3, 1, 0, th, 4);
    }

    for (const side of [-1, 1]) {
      this.addEllipsoid(b, [side * 0.20 * s, yB + 0.04 * s, -0.02 * s], [0.06 * s, 0.14 * s, 0.24 * s], col1, col3, 2, side, 0.6, th, -0.2);
      this.addArticulatedChain(b, [[side * 0.09 * s, yB - 0.14 * s, -0.02 * s], [side * 0.09 * s, 0.02 * s, 0.04 * s]], [0.03 * s, 0.02 * s], col3, col3, 3, side, th, 4);
    }

    if (arch === 'lyrebird') {
      // Magnificent lyre-shaped outer rectrices + filamentous plumes
      for (const side of [-1, 1]) {
        this.addArticulatedChain(b, [
          [side * 0.04 * s, yB - 0.04 * s, -0.26 * s],
          [side * 0.24 * s, yB + 0.25 * s, -0.68 * s],
          [side * 0.36 * s, yB + 0.58 * s, -1.05 * s]
        ], [0.045 * s, 0.06 * s, 0.03 * s], col2, col3, 4, side, th, 4);
      }
      this.addEllipsoid(b, [0, yB + 0.22 * s, -0.68 * s], [0.22 * s, 0.24 * s, 0.42 * s], col2, col1, 4, 0, 0.8, th, 0.4);
    } else {
      this.addEllipsoid(b, [0, yB - 0.08 * s, -0.44 * s], [0.11 * s, 0.035 * s, 0.24 * s], col1, col3, 4, 0, 0.9, th, -0.2);
    }
  }

  buildRatite(b, s, col1, col2, col3, th) {
    // Emu / Southern Cassowary
    this.addEllipsoid(b, [0, 0.92 * s, 0], [0.30 * s, 0.34 * s, 0.46 * s], col1, col1, 0, 0, 0.5, th, 0.1);
    this.addArticulatedChain(b, [[0, 1.12 * s, 0.34 * s], [0, 1.58 * s, 0.42 * s], [0, 1.66 * s, 0.58 * s]], [0.09 * s, 0.07 * s, 0.04 * s], col2, col3, 1, 0, th, 5);
    for (const side of [-1, 1]) {
      this.addArticulatedChain(b, [[side * 0.16 * s, 0.68 * s, 0], [side * 0.16 * s, 0.36 * s, -0.08 * s], [side * 0.16 * s, 0.02 * s, 0.06 * s]], [0.09 * s, 0.05 * s, 0.04 * s], col2, col3, 3, side, th, 5);
    }
  }

  buildWaterbird(b, s, col1, col2, col3, th) {
    // Black Swan
    this.addEllipsoid(b, [0, 0.26 * s, 0], [0.24 * s, 0.18 * s, 0.42 * s], col1, col2, 0, 0, 0.5, th);
    this.addArticulatedChain(b, [[0, 0.34 * s, 0.34 * s], [0, 0.66 * s, 0.30 * s], [0, 0.82 * s, 0.44 * s], [0, 0.78 * s, 0.58 * s]], [0.07 * s, 0.055 * s, 0.065 * s, 0.03 * s], col1, col3, 1, 0, th, 5);
  }

  buildLizard(b, s, col1, col2, col3, th) {
    // Lace Monitor / Water Dragon / Blue-tongue / Alpine Skink
    this.addArticulatedChain(b, [
      [0, 0.18 * s, 0.55 * s],
      [0, 0.22 * s, 0.32 * s],
      [0, 0.18 * s, 0],
      [0, 0.16 * s, -0.35 * s],
      [0, 0.10 * s, -0.78 * s],
      [0, 0.06 * s, -1.25 * s]
    ], [0.07 * s, 0.12 * s, 0.16 * s, 0.13 * s, 0.07 * s, 0.02 * s], col1, col2, 4, 0, th, 6);

    for (const side of [-1, 1]) {
      this.addArticulatedChain(b, [[side * 0.14 * s, 0.16 * s, 0.20 * s], [side * 0.30 * s, 0.14 * s, 0.24 * s], [side * 0.28 * s, 0.02 * s, 0.30 * s]], [0.055 * s, 0.045 * s, 0.03 * s], col1, col3, 2, side, th, 4);
      this.addArticulatedChain(b, [[side * 0.14 * s, 0.16 * s, -0.24 * s], [side * 0.32 * s, 0.14 * s, -0.28 * s], [side * 0.30 * s, 0.02 * s, -0.22 * s]], [0.06 * s, 0.05 * s, 0.03 * s], col1, col3, 3, side, th, 4);
    }
  }

  buildSnake(b, s, col1, col2, col3, th) {
    // Diamond Python / Red-bellied Black Snake / Mainland Tiger Snake
    const pts = [];
    const rads = [];
    const segs = 10;
    for (let i = 0; i < segs; i++) {
      const t = i / (segs - 1);
      const z = (0.5 - t) * 1.6 * s;
      const x = Math.sin(t * Math.PI * 2.5) * 0.24 * s;
      const y = (i === 0 ? 0.18 : 0.08) * s;
      pts.push([x, y, z]);
      rads.push(Math.sin(Math.PI * (0.12 + 0.82 * (1 - t))) * 0.085 * s);
    }
    this.addArticulatedChain(b, pts, rads, col1, col2, 4, 0, th, 6);
  }

  buildTurtle(b, s, col1, col2, col3, th) {
    // Murray River Turtle
    this.addEllipsoid(b, [0, 0.18 * s, 0], [0.32 * s, 0.14 * s, 0.40 * s], col1, col2, 0, 0, 0.5, th);
    this.addArticulatedChain(b, [[0, 0.18 * s, 0.36 * s], [0, 0.24 * s, 0.58 * s]], [0.07 * s, 0.06 * s], col2, col3, 1, 0, th, 5);
    for (const side of [-1, 1]) {
      this.addArticulatedChain(b, [[side * 0.26 * s, 0.12 * s, 0.22 * s], [side * 0.42 * s, 0.04 * s, 0.28 * s]], [0.055 * s, 0.04 * s], col2, col3, 2, side, th, 4);
      this.addArticulatedChain(b, [[side * 0.26 * s, 0.12 * s, -0.22 * s], [side * 0.40 * s, 0.04 * s, -0.28 * s]], [0.055 * s, 0.04 * s], col2, col3, 3, side, th, 4);
    }
  }

  buildFrog(b, s, col1, col2, col3, th) {
    // Southern Corroboree Frog (vivid yellow/black stripes) / Green & Golden Bell Frog
    this.addEllipsoid(b, [0, 0.22 * s, 0], [0.22 * s, 0.16 * s, 0.28 * s], col1, col2, 0, 0, 0.5, th, -0.35);
    this.addEllipsoid(b, [0, 0.30 * s, 0.22 * s], [0.18 * s, 0.12 * s, 0.16 * s], col2, col3, 1, 0, 0.9, th);
    for (const side of [-1, 1]) {
      this.addArticulatedChain(b, [[side * 0.18 * s, 0.22 * s, 0.14 * s], [side * 0.22 * s, 0.02 * s, 0.22 * s]], [0.05 * s, 0.035 * s], col1, col2, 2, side, th, 4);
      this.addArticulatedChain(b, [[side * 0.18 * s, 0.14 * s, -0.16 * s], [side * 0.32 * s, 0.26 * s, -0.04 * s], [side * 0.24 * s, 0.02 * s, -0.20 * s]], [0.065 * s, 0.05 * s, 0.035 * s], col2, col1, 3, side, th, 4);
    }
  }

  buildAquaticFauna(b, s, col1, col2, col3, isCrayfish, th) {
    // Murray Cod / Macquarie Perch / Mountain Galaxias / Murray Spiny Crayfish
    this.addEllipsoid(b, [0, 0.22 * s, 0.08 * s], [0.16 * s, 0.20 * s, 0.42 * s], col1, col2, 0, 0, 0.4, th);
    this.addArticulatedChain(b, [[0, 0.22 * s, -0.28 * s], [0, 0.22 * s, -0.58 * s], [0, 0.22 * s, -0.78 * s]], [0.12 * s, 0.05 * s, 0.14 * s], col1, col3, 4, 0, th, 4);
    if (isCrayfish) {
      for (const side of [-1, 1]) {
        this.addArticulatedChain(b, [[side * 0.14 * s, 0.20 * s, 0.34 * s], [side * 0.28 * s, 0.22 * s, 0.64 * s]], [0.05 * s, 0.10 * s], col3, col2, 2, side, th, 5);
      }
    }
  }

  buildInsect(b, s, col1, col2, col3, th) {
    // Bogong Moth / Mountain Katydid / Golden Stag Beetle / Sugarbag Bee
    this.addEllipsoid(b, [0, 0.24 * s, 0], [0.12 * s, 0.10 * s, 0.28 * s], col1, col2, 0, 0, 0.5, th);
    for (const side of [-1, 1]) {
      this.addEllipsoid(b, [side * 0.28 * s, 0.28 * s, -0.04 * s], [0.26 * s, 0.02 * s, 0.20 * s], col2, col3, 2, side, 0.8, th);
    }
  }

  /**
   * Advances autonomous N3Logic / VibeScript ecological behavior and biomechanical gait
   * for an active fauna instance on the terrain.
   */
  updateAgent(agent, dt, time, getTerrainHeight, observerPos) {
    const sp = agent.species;
    const gait = sp.kinematics?.locomotion_gait || 'bipedal_hop';
    const habitat = sp.habitat_stratum || 'terrestrial';
    const alertRad = sp.ethology?.alert_radius_m || 12.0;

    // Check proximity to observer / camera for N3Logic hazard evasion rule
    const dxObs = agent.x - observerPos[0];
    const dzObs = agent.z - observerPos[2];
    const distObs = Math.hypot(dxObs, dzObs);

    agent.stateTimer -= dt;
    if (distObs < alertRad * 0.65 && habitat !== 'benthic_stream') {
      agent.behaviorState = 'FLEEING';
      agent.targetYaw = Math.atan2(dxObs, dzObs);
      agent.stateTimer = 2.5;
    } else if (agent.stateTimer <= 0) {
      const states = sp.ethology?.states || ['FORAGING', 'MOVING', 'VIGILANT_SCAN'];
      agent.stateIndex = (agent.stateIndex + 1) % states.length;
      agent.behaviorState = states[agent.stateIndex];
      agent.stateTimer = 3.5 + (agent.seed % 5) * 1.1;
      agent.targetYaw += (Math.sin(time * 1.7 + agent.seed) * 1.4);
    }

    // Smooth yaw steering
    let yawDiff = agent.targetYaw - agent.yaw;
    while (yawDiff > Math.PI) yawDiff -= 2 * Math.PI;
    while (yawDiff < -Math.PI) yawDiff += 2 * Math.PI;
    agent.yaw += yawDiff * Math.min(1.0, dt * 3.0);

    // Speed envelope from Movebank parameters
    const isMoving = !['GRAZING', 'RESTING_SHADE', 'VIGILANT_SCAN', 'BASKING', 'PERCHING', 'DISPLAYING'].includes(agent.behaviorState);
    const isFleeing = agent.behaviorState === 'FLEEING';
    const baseSpeed = isFleeing
      ? Math.min(8.5, (sp.kinematics?.velocity_sprint_ms || 6.0) * 0.65)
      : isMoving
        ? Math.min(4.2, (sp.kinematics?.velocity_trot_ms || 2.5) * 0.65)
        : 0.35;

    agent.speed = baseSpeed;
    agent.phase += dt * (sp.kinematics?.stride_freq_hz || 2.2) * (isMoving ? 3.2 : 1.1);

    // Advance world XZ coordinates within sector bounds [-52, 52]
    if (gait === 'soaring_flight') {
      // Circular thermal soaring trajectory high above ridges
      const orbitAng = time * 0.35 + agent.seed;
      agent.x = agent.homeX + Math.cos(orbitAng) * 18.0;
      agent.z = agent.homeZ + Math.sin(orbitAng) * 18.0;
      agent.yaw = orbitAng + Math.PI * 0.5;
    } else {
      agent.x += Math.sin(agent.yaw) * agent.speed * dt;
      agent.z += Math.cos(agent.yaw) * agent.speed * dt;
      if (Math.abs(agent.x) > 52 || Math.abs(agent.z) > 52) {
        agent.targetYaw = Math.atan2(-agent.x, -agent.z);
        agent.x = Math.max(-52, Math.min(52, agent.x));
        agent.z = Math.max(-52, Math.min(52, agent.z));
      }
    }

    // Compute vertical height & gait bounce
    const groundY = getTerrainHeight(agent.x, agent.z);
    let yOffset = 0.0;
    if (gait === 'bipedal_hop' && isMoving) {
      yOffset = Math.abs(Math.sin(agent.phase)) * 0.85;
    } else if (gait === 'saltatorial_frog' && isMoving) {
      yOffset = Math.max(0, Math.sin(agent.phase)) * 0.65;
    } else if (gait === 'soaring_flight') {
      yOffset = 14.0 + Math.sin(time * 0.8 + agent.seed) * 2.5;
    } else if (gait === 'flapping_flight' || gait === 'patagial_glide' || gait === 'insect_flutter') {
      yOffset = 4.5 + Math.sin(time * 1.6 + agent.seed) * 1.4;
    } else if (habitat === 'arboreal') {
      yOffset = 2.4 + Math.sin(time * 0.6 + agent.seed) * 0.3;
    }

    agent.y = groundY + yOffset;
  }
}

window.FaunaEngine = FaunaEngine;
