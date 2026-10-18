/**
 * Ecopunk 3D Holographic Earth-Terrain & Observer Shader Engine
 *
 * Implements 5 Normative Observer Projection Kernels:
 * 0x00 PBR, 0x01 Toon, 0x02 Watercolor, 0x03 HUD, 0x04 Thermal
 * Reference: qualia-core-db::wgsl_forge::observer_kernels
 */

class TerrainRenderer {
  constructor(canvas) {
    this.canvas = canvas;
    this.gl = canvas.getContext('webgl', { antialias: true, alpha: false }) ||
              canvas.getContext('experimental-webgl');
    
    if (!this.gl) {
      console.error('WebGL not supported');
      return;
    }

    this.observerMode = 0; // 0=PBR, 1=Toon, 2=Watercolor, 3=HUD, 4=Thermal
    this.gridSize = 64; // 64x64 vertices (matches Hilbert tile chunks)
    this.camera = {
      rotX: 45 * Math.PI / 180,
      rotY: 35 * Math.PI / 180,
      dist: 55.0,
      target: [0, 0, 0],
      velocity: [0, 0, 0],
      lastPos: [0, 0, 0]
    };
    this.sunAngle = 0.8;
    this.time = 0;
    this.isDragging = false;
    this.lastMouse = { x: 0, y: 0 };
    this.excavationMode = false;
    this.excavations = []; // {x, z, radius, depth}

    this.isoglosses = [];
    this.toponyms = [];
    this.selectedToponym = null;

    this.initShaders();
    this.buildTerrainMesh();
    this.setupInteractions();
    this.resize();
  }

  initShaders() {
    const gl = this.gl;

    const vsSource = `
      attribute vec3 aPosition;
      attribute vec3 aNormal;
      attribute vec2 aTexCoord;
      attribute float aElevation;

      uniform mat4 uProjection;
      uniform mat4 uView;
      uniform mat4 uModel;

      varying vec3 vNormal;
      varying vec3 vWorldPos;
      varying vec2 vTexCoord;
      varying float vElevation;

      void main() {
        vec4 worldPos = uModel * vec4(aPosition, 1.0);
        vWorldPos = worldPos.xyz;
        vNormal = mat3(uModel) * aNormal;
        vTexCoord = aTexCoord;
        vElevation = aElevation;
        gl_Position = uProjection * uView * worldPos;
      }
    `;

    const fsSource = `
      precision mediump float;

      varying vec3 vNormal;
      varying vec3 vWorldPos;
      varying vec2 vTexCoord;
      varying float vElevation;

      uniform vec3 uCameraPos;
      uniform vec3 uSunDir;
      uniform int uObserverMode;
      uniform float uTime;
      uniform vec4 uIsoglossParam; // x, z, radius, active

      // Stratigraphic colors for Australian Terrain
      vec3 getTerrainColor(float h, vec3 n) {
        // Red Ochre, Weathered Sandstone, Spinifex scrub, Gorge Rock
        vec3 ochre = vec3(0.72, 0.28, 0.16);
        vec3 sand = vec3(0.85, 0.65, 0.42);
        vec3 scrub = vec3(0.42, 0.52, 0.32);
        vec3 darkRock = vec3(0.35, 0.20, 0.15);
        vec3 ghostGumWhite = vec3(0.92, 0.94, 0.96);

        float slope = 1.0 - max(dot(n, vec3(0.0, 1.0, 0.0)), 0.0);
        vec3 base = mix(sand, ochre, smoothstep(-2.0, 4.0, h));
        base = mix(base, darkRock, smoothstep(0.4, 0.8, slope));
        base = mix(base, scrub, smoothstep(-1.0, 1.5, h) * (1.0 - slope) * 0.5);
        if (h > 6.5) base = mix(base, ghostGumWhite, smoothstep(6.5, 9.0, h) * 0.7);
        return base;
      }

      void main() {
        vec3 n = normalize(vNormal);
        vec3 viewDir = normalize(uCameraPos - vWorldPos);
        vec3 lightDir = normalize(uSunDir);
        float NdotL = max(dot(n, lightDir), 0.0);
        vec3 baseCol = getTerrainColor(vElevation, n);

        vec3 finalCol = baseCol;

        // Mode 0: Spectral PBR
        if (uObserverMode == 0) {
          vec3 halfDir = normalize(lightDir + viewDir);
          float spec = pow(max(dot(n, halfDir), 0.0), 32.0) * 0.4;
          vec3 ambient = vec3(0.12, 0.14, 0.22) * baseCol;
          vec3 sunLight = vec3(1.0, 0.95, 0.85) * (NdotL + spec);
          finalCol = ambient + baseCol * sunLight;

          // Subtle elevation contour lines
          float contour = fract(vElevation * 0.5);
          if (contour < 0.04) finalCol *= 0.85;
        }
        // Mode 1: Cel-Shaded Toon
        else if (uObserverMode == 1) {
          float cel = 0.3;
          if (NdotL > 0.6) cel = 1.0;
          else if (NdotL > 0.25) cel = 0.65;

          // Edge inking via view falloff
          float edge = max(dot(n, viewDir), 0.0);
          float ink = smoothstep(0.18, 0.25, edge);

          finalCol = baseCol * cel * (0.2 + 0.8 * ink);
        }
        // Mode 2: Storybook Watercolor
        else if (uObserverMode == 2) {
          float paperNoise = sin(vWorldPos.x * 3.0) * cos(vWorldPos.z * 3.0) * 0.08;
          float bleed = smoothstep(0.1, 0.9, NdotL + paperNoise);
          vec3 wash = mix(baseCol * 0.7, baseCol * 1.25, bleed);
          
          // Edge granulation
          float edgeDarkening = 1.0 - 0.25 * pow(1.0 - max(dot(n, viewDir), 0.0), 2.0);
          finalCol = wash * edgeDarkening;
        }
        // Mode 3: Tactical / GPR HUD
        else if (uObserverMode == 3) {
          float grid = step(0.92, fract(vWorldPos.x * 0.5)) + step(0.92, fract(vWorldPos.z * 0.5));
          float scanline = sin(vWorldPos.y * 12.0 + uTime * 4.0) * 0.5 + 0.5;
          vec3 cyanWire = vec3(0.0, 0.95, 1.0);
          vec3 darkBacking = vec3(0.03, 0.08, 0.14);
          
          finalCol = mix(darkBacking, cyanWire, clamp(grid + scanline * 0.3, 0.0, 1.0));
          if (vElevation > 4.0) finalCol += vec3(0.5, 0.0, 0.8) * 0.4;
        }
        // Mode 4: Planck Thermal Infrared
        else if (uObserverMode == 4) {
          // Heat mapping based on elevation & sun exposure
          float heat = clamp((NdotL * 0.6) + (vElevation + 5.0) / 15.0, 0.0, 1.0);
          
          vec3 coldBlue = vec3(0.0, 0.1, 0.6);
          vec3 midPurple = vec3(0.6, 0.0, 0.8);
          vec3 warmOrange = vec3(1.0, 0.5, 0.0);
          vec3 hotWhite = vec3(1.0, 1.0, 0.9);

          if (heat < 0.33) finalCol = mix(coldBlue, midPurple, heat * 3.0);
          else if (heat < 0.66) finalCol = mix(midPurple, warmOrange, (heat - 0.33) * 3.0);
          else finalCol = mix(warmOrange, hotWhite, (heat - 0.66) * 3.0);
        }

        // Draw Diachronic Isogloss geographic boundary ring if active
        if (uIsoglossParam.w > 0.5) {
          float distToCenter = length(vWorldPos.xz - uIsoglossParam.xz);
          float ringWidth = 0.4;
          float ringDist = abs(distToCenter - uIsoglossParam.y);
          if (ringDist < ringWidth) {
            float glow = 1.0 - (ringDist / ringWidth);
            finalCol = mix(finalCol, vec3(0.0, 1.0, 0.8), glow * 0.85);
          }
        }

        gl_FragColor = vec4(finalCol, 1.0);
      }
    `;

    const vs = this.compileShader(gl.VERTEX_SHADER, vsSource);
    const fs = this.compileShader(gl.FRAGMENT_SHADER, fsSource);

    this.program = gl.createProgram();
    gl.attachShader(this.program, vs);
    gl.attachShader(this.program, fs);
    gl.linkProgram(this.program);

    if (!gl.getProgramParameter(this.program, gl.LINK_STATUS)) {
      console.error('Shader link error:', gl.getProgramInfoLog(this.program));
      return;
    }

    this.attribs = {
      position: gl.getAttribLocation(this.program, 'aPosition'),
      normal: gl.getAttribLocation(this.program, 'aNormal'),
      texCoord: gl.getAttribLocation(this.program, 'aTexCoord'),
      elevation: gl.getAttribLocation(this.program, 'aElevation')
    };

    this.uniforms = {
      projection: gl.getUniformLocation(this.program, 'uProjection'),
      view: gl.getUniformLocation(this.program, 'uView'),
      model: gl.getUniformLocation(this.program, 'uModel'),
      cameraPos: gl.getUniformLocation(this.program, 'uCameraPos'),
      sunDir: gl.getUniformLocation(this.program, 'uSunDir'),
      observerMode: gl.getUniformLocation(this.program, 'uObserverMode'),
      time: gl.getUniformLocation(this.program, 'uTime'),
      isoglossParam: gl.getUniformLocation(this.program, 'uIsoglossParam')
    };
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

  buildTerrainMesh() {
    const gl = this.gl;
    const N = this.gridSize;
    const size = 40.0;
    const half = size / 2.0;

    const positions = [];
    const normals = [];
    const texCoords = [];
    const elevations = [];
    const indices = [];

    // Height calculation with Australian MacDonnell Ranges / Red Centre contours
    const getHeight = (x, z) => {
      let h = Math.sin(x * 0.15) * Math.cos(z * 0.15) * 3.5;
      h += Math.sin(x * 0.35 + 1.2) * Math.cos(z * 0.3) * 1.8;
      h += Math.sin(x * 0.8 - z * 0.6) * 0.8;
      // Ridge feature (Gorge)
      const ridge = Math.exp(-Math.pow((x * 0.6 - z * 0.2) / 3.0, 2.0)) * 4.5;
      h += ridge;

      // Apply excavated cavities if any
      for (const exc of this.excavations) {
        const d = Math.sqrt((x - exc.x) * (x - exc.x) + (z - exc.z) * (z - exc.z));
        if (d < exc.radius) {
          const depth = (1.0 - Math.pow(d / exc.radius, 2.0)) * exc.depth;
          h -= depth;
        }
      }
      return h;
    };

    this.getHeightAt = getHeight;

    // Generate vertices
    for (let j = 0; j <= N; j++) {
      const z = (j / N) * size - half;
      for (let i = 0; i <= N; i++) {
        const x = (i / N) * size - half;
        const y = getHeight(x, z);

        positions.push(x, y, z);
        elevations.push(y);
        texCoords.push(i / N, j / N);

        // Approximate normal
        const eps = 0.1;
        const hL = getHeight(x - eps, z);
        const hR = getHeight(x + eps, z);
        const hD = getHeight(x, z - eps);
        const hU = getHeight(x, z + eps);
        const norm = [- (hR - hL) / (2 * eps), 1.0, - (hU - hD) / (2 * eps)];
        const len = Math.sqrt(norm[0]*norm[0] + norm[1]*norm[1] + norm[2]*norm[2]);
        normals.push(norm[0]/len, norm[1]/len, norm[2]/len);
      }
    }

    // Generate triangle indices
    for (let j = 0; j < N; j++) {
      for (let i = 0; i < N; i++) {
        const row1 = j * (N + 1);
        const row2 = (j + 1) * (N + 1);

        indices.push(row1 + i, row2 + i, row1 + i + 1);
        indices.push(row1 + i + 1, row2 + i, row2 + i + 1);
      }
    }

    this.indexCount = indices.length;

    // Buffers
    this.posBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, this.posBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

    this.normBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, this.normBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(normals), gl.STATIC_DRAW);

    this.elevBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, this.elevBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(elevations), gl.STATIC_DRAW);

    this.texBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, this.texBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(texCoords), gl.STATIC_DRAW);

    this.indexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW);
  }

  setObserverMode(mode) {
    this.observerMode = mode;
  }

  setExcavationMode(active) {
    this.excavationMode = active;
  }

  digExcavation(worldX, worldZ) {
    this.excavations.push({
      x: worldX,
      z: worldZ,
      radius: 4.0,
      depth: 3.5
    });
    this.buildTerrainMesh(); // Rebuild mesh with CSG hole
  }

  setupInteractions() {
    const canvas = this.canvas;

    canvas.addEventListener('mousedown', (e) => {
      this.isDragging = true;
      this.lastMouse = { x: e.clientX, y: e.clientY };

      if (this.excavationMode) {
        // Project click ray onto terrain surface
        const rect = canvas.getBoundingClientRect();
        const ndcX = ((e.clientX - rect.left) / canvas.width) * 2 - 1;
        const ndcY = -(((e.clientY - rect.top) / canvas.height) * 2 - 1);
        // Simple planar pick approximation
        const pickX = ndcX * 15.0;
        const pickZ = -ndcY * 15.0;
        this.digExcavation(pickX, pickZ);
        if (window.onExcavationDone) {
          window.onExcavationDone(pickX, pickZ, 3.5);
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

      if (e.buttons === 1) { // Left click: orbit
        this.camera.rotY += dx * 0.007;
        this.camera.rotX = Math.max(0.1, Math.min(Math.PI / 2.2, this.camera.rotX + dy * 0.007));
      } else if (e.buttons === 2) { // Right click: pan
        this.camera.target[0] -= dx * 0.03;
        this.camera.target[2] -= dy * 0.03;
      }
    });

    canvas.addEventListener('wheel', (e) => {
      e.preventDefault();
      this.camera.dist = Math.max(15.0, Math.min(100.0, this.camera.dist + e.deltaY * 0.05));
    }, { passive: false });

    canvas.addEventListener('contextmenu', (e) => e.preventDefault());
  }

  resize() {
    const w = this.canvas.clientWidth;
    const h = this.canvas.clientHeight;
    if (this.canvas.width !== w || this.canvas.height !== h) {
      this.canvas.width = w;
      this.canvas.height = h;
      this.gl.viewport(0, 0, w, h);
    }
  }

  render() {
    this.resize();
    const gl = this.gl;
    this.time += 0.016;

    gl.clearColor(0.03, 0.04, 0.06, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    gl.enable(gl.DEPTH_TEST);

    gl.useProgram(this.program);

    // Compute Camera Eye Position
    const eyeX = this.camera.target[0] + this.camera.dist * Math.cos(this.camera.rotX) * Math.sin(this.camera.rotY);
    const eyeY = this.camera.target[1] + this.camera.dist * Math.sin(this.camera.rotX);
    const eyeZ = this.camera.target[2] + this.camera.dist * Math.cos(this.camera.rotX) * Math.cos(this.camera.rotY);
    const cameraPos = [eyeX, eyeY, eyeZ];

    // Compute camera velocity for Doppler
    this.camera.velocity = [
      eyeX - this.camera.lastPos[0],
      eyeY - this.camera.lastPos[1],
      eyeZ - this.camera.lastPos[2]
    ];
    this.camera.lastPos = [...cameraPos];

    // Matrices
    const aspect = this.canvas.width / this.canvas.height;
    const projMat = this.createPerspective(45 * Math.PI / 180, aspect, 0.5, 250.0);
    const viewMat = this.createLookAt(cameraPos, this.camera.target, [0, 1, 0]);
    const modelMat = this.createIdentity();

    // Uniforms
    gl.uniformMatrix4fv(this.uniforms.projection, false, projMat);
    gl.uniformMatrix4fv(this.uniforms.view, false, viewMat);
    gl.uniformMatrix4fv(this.uniforms.model, false, modelMat);
    gl.uniform3fv(this.uniforms.cameraPos, cameraPos);

    // Sun directional vector (revolving slowly)
    const sunDir = [Math.cos(this.time * 0.1), 0.75, Math.sin(this.time * 0.1)];
    gl.uniform3fv(this.uniforms.sunDir, sunDir);

    gl.uniform1i(this.uniforms.observerMode, this.observerMode);
    gl.uniform1f(this.uniforms.time, this.time);

    // Active Isogloss param [center_x, center_z, radius, active]
    if (this.activeIsogloss) {
      gl.uniform4f(
        this.uniforms.isoglossParam,
        this.activeIsogloss.center_x,
        this.activeIsogloss.center_z,
        this.activeIsogloss.radius,
        1.0
      );
    } else {
      gl.uniform4f(this.uniforms.isoglossParam, 0.0, 0.0, 0.0, 0.0);
    }

    // Attributes
    gl.bindBuffer(gl.ARRAY_BUFFER, this.posBuffer);
    gl.enableVertexAttribArray(this.attribs.position);
    gl.vertexAttribPointer(this.attribs.position, 3, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ARRAY_BUFFER, this.normBuffer);
    gl.enableVertexAttribArray(this.attribs.normal);
    gl.vertexAttribPointer(this.attribs.normal, 3, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ARRAY_BUFFER, this.elevBuffer);
    gl.enableVertexAttribArray(this.attribs.elevation);
    gl.vertexAttribPointer(this.attribs.elevation, 1, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ARRAY_BUFFER, this.texBuffer);
    gl.enableVertexAttribArray(this.attribs.texCoord);
    gl.vertexAttribPointer(this.attribs.texCoord, 2, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);
    gl.drawElements(gl.TRIANGLES, this.indexCount, gl.UNSIGNED_SHORT, 0);
  }

  // Linear algebra helpers for zero-dependency matrix generation
  createIdentity() {
    return new Float32Array([
      1,0,0,0,
      0,1,0,0,
      0,0,1,0,
      0,0,0,1
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
    let len = 1 / Math.sqrt(z0 * z0 + z1 * z1 + z2 * z2);
    const zx = z0 * len, zy = z1 * len, zz = z2 * len;

    const x0 = up[1] * zz - up[2] * zy;
    const x1 = up[2] * zx - up[0] * zz;
    const x2 = up[0] * zy - up[1] * zx;
    len = 1 / Math.sqrt(x0 * x0 + x1 * x1 + x2 * x2);
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
