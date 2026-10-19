/**
 * EcoPunk Native WebGPU (navigator.gpu) & WGSL Forge Rendering + Compute Engine
 *
 * Directly executes Naga-certified WGSL shaders compiled by `compile_east_coast_substrate`
 * from `qualia_core_db::wgsl_forge::observer_kernels` and `qualia_core_db::shaders::viewport`:
 * - Unified Render Pipeline (`vs_main` / `fs_main` + `vs_line` / `fs_line`):
 *     * `SPECTRAL_WGSL`: CIE 1931 XYZ color matching (`sigma_to_cie_xyz`, `xyz_to_linear_srgb`)
 *     * `WGSL_KERNEL_PBR` (`0x00`), `WGSL_KERNEL_TOON` (`0x01`), `WGSL_KERNEL_WATERCOLOR` (`0x02`),
 *       `WGSL_KERNEL_HUD` (`0x03`), `WGSL_KERNEL_THERMAL` (`0x04`)
 *     * `SGGX_MICROFLAKE_WGSL`: Anisotropic foliage/ground fiber microflake BRDF
 *     * `HYDROLOGIC_OPTICS_WGSL`: Henyey-Greenstein particulate phase & Beer-Lambert water optics
 *     * 16-KB `BankedConstituentLut` (`@group(0) @binding(1) var<storage, read> constituent_lut`)
 * - Compute Pipeline (`@compute @workgroup_size(64, 1, 1)`):
 *     * `FOLIAGE_LOD_WGSL` (`foliage_cull_and_generate`)
 *     * `KIN_STEP_WGSL` (`nbody_step`)
 */

class WebGpuForgeEngine {
  constructor() {
    this.ready = false;
    this.device = null;
    this.adapterInfo = null;
    this.context = null;
    this.canvas = null;
    this.format = 'bgra8unorm';
    this.depthTexture = null;
    this.depthWidth = 0;
    this.depthHeight = 0;

    // Pipelines
    this.meshPipeline = null;
    this.linePipeline = null;
    this.lineNoDepthPipeline = null;
    this.foliageComputePipeline = null;
    this.kinematicsComputePipeline = null;

    // Buffers & Bind Groups
    this.sceneUniformBuffer = null;
    this.lutStorageBuffer = null;
    this.entityUniformBuffer = null;
    this.sceneBindGroup = null;
    this.entityBindGroupLayout = null;
    this.entityBindGroup = null;
    this.dynamicLineBuffer = null;

    this.maxEntities = 512;
    this.entitySlotSize = 256; // WebGPU minUniformBufferOffsetAlignment = 256 bytes
    this.entityStaging = new Float32Array((this.maxEntities * this.entitySlotSize) / 4);
    this.meshCache = new WeakMap();
    this.computeDispatchCount = 0;
  }

  /**
   * Initializes WebGPU on a dedicated overlay canvas (`#webgpu-canvas`) and compiles
   * all Naga-certified WGSL Forge shaders.
   */
  async init(parentCanvas) {
    if (!navigator.gpu) {
      this.lastInitError = 'navigator.gpu unavailable';
      return false;
    }
    const forge = window.ECOPUNK_WGSL_FORGE;
    if (!forge || !forge.unified_webgpu_wgsl) {
      this.lastInitError = 'ECOPUNK_WGSL_FORGE bundle missing';
      return false;
    }

    try {
      let adapter = await navigator.gpu.requestAdapter({ powerPreference: 'high-performance' });
      if (!adapter) {
        adapter = await navigator.gpu.requestAdapter();
      }
      if (!adapter) {
        adapter = await navigator.gpu.requestAdapter({ forceFallbackAdapter: true });
      }
      if (!adapter) {
        this.lastInitError = 'navigator.gpu.requestAdapter() returned null';
        return false;
      }

      this.device = await adapter.requestDevice();
      this.adapterInfo = adapter.info || { vendor: 'WebGPU', architecture: 'WGSL-Forge' };

      // Attach directly to #viewport-canvas (or #webgpu-canvas overlay if parentCanvas already has a context)
      let gpuCanvas = parentCanvas;
      let ctx = gpuCanvas.getContext('webgpu');
      if (!ctx) {
        gpuCanvas = document.getElementById('webgpu-canvas');
        if (!gpuCanvas) {
          gpuCanvas = document.createElement('canvas');
          gpuCanvas.id = 'webgpu-canvas';
          gpuCanvas.style.cssText = 'position:absolute;inset:0;width:100%;height:100%;pointer-events:none;z-index:1;';
          parentCanvas.parentElement.insertBefore(gpuCanvas, parentCanvas.nextSibling);
        }
        ctx = gpuCanvas.getContext('webgpu');
      }
      this.canvas = gpuCanvas;
      this.context = ctx;
      if (!this.context) {
        this.lastInitError = 'gpuCanvas.getContext("webgpu") returned null';
        return false;
      }

      this.format = navigator.gpu.getPreferredCanvasFormat();
      this.resize(parentCanvas.width || 1280, parentCanvas.height || 720);

      // 1. Compile Unified WGSL Render Shader Module (certified by Naga in compile_east_coast_substrate)
      const shaderModule = this.device.createShaderModule({
        label: 'EcoPunk-WGSL-Forge-Unified-Shader',
        code: forge.unified_webgpu_wgsl
      });

      // 2. Compile Viewport Compute Shaders (FOLIAGE_LOD_WGSL + KIN_STEP_WGSL)
      if (forge.viewport_library_wgsl?.foliage_lod) {
        const foliageMod = this.device.createShaderModule({
          label: 'EcoPunk-WGSL-Forge-Foliage-LOD-Compute',
          code: forge.viewport_library_wgsl.foliage_lod
        });
        this.foliageComputePipeline = this.device.createComputePipeline({
          label: 'Foliage-LOD-Compute-Pipeline',
          layout: 'auto',
          compute: { module: foliageMod, entryPoint: 'foliage_cull_and_generate' }
        });
      }

      if (forge.viewport_library_wgsl?.kinematics) {
        const kinMod = this.device.createShaderModule({
          label: 'EcoPunk-WGSL-Forge-Kinematics-Compute',
          code: forge.viewport_library_wgsl.kinematics
        });
        this.kinematicsComputePipeline = this.device.createComputePipeline({
          label: 'Kinematics-NBody-Compute-Pipeline',
          layout: 'auto',
          compute: { module: kinMod, entryPoint: 'nbody_step' }
        });
      }

      // 3. Create Scene Uniform Buffer (208 bytes -> 256 bytes aligned)
      this.sceneUniformBuffer = this.device.createBuffer({
        label: 'SceneUniforms-UBO',
        size: 256,
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
      });

      // 4. Create 16-KB BankedConstituentLut Storage Buffer (@group(0) @binding(1))
      this.lutStorageBuffer = this.device.createBuffer({
        label: 'BankedConstituentLut-16KB-SSBO',
        size: 16384,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
      });
      if (forge.constituent_lut_u32 && forge.constituent_lut_u32.length === 4096) {
        this.device.queue.writeBuffer(this.lutStorageBuffer, 0, new Uint32Array(forge.constituent_lut_u32));
      }

      // 5. Create Dynamic Entity Uniform Buffer (512 * 256 bytes = 128 KB)
      this.entityUniformBuffer = this.device.createBuffer({
        label: 'EntityUniforms-Dynamic-UBO',
        size: this.maxEntities * this.entitySlotSize,
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
      });

      this.dynamicLineBuffer = this.device.createBuffer({
        label: 'OverlayLines-VBO',
        size: 65536,
        usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST
      });

      // 6. Bind Group Layouts
      const sceneBindGroupLayout = this.device.createBindGroupLayout({
        label: 'Scene-BGL',
        entries: [
          {
            binding: 0,
            visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
            buffer: { type: 'uniform' }
          },
          {
            binding: 1,
            visibility: GPUShaderStage.FRAGMENT,
            buffer: { type: 'read-only-storage' }
          }
        ]
      });

      this.entityBindGroupLayout = this.device.createBindGroupLayout({
        label: 'Entity-Dynamic-BGL',
        entries: [
          {
            binding: 0,
            visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
            buffer: {
              type: 'uniform',
              hasDynamicOffset: true,
              minBindingSize: 96
            }
          }
        ]
      });

      const pipelineLayout = this.device.createPipelineLayout({
        label: 'EcoPunk-Pipeline-Layout',
        bindGroupLayouts: [sceneBindGroupLayout, this.entityBindGroupLayout]
      });

      this.sceneBindGroup = this.device.createBindGroup({
        label: 'Scene-BG',
        layout: sceneBindGroupLayout,
        entries: [
          { binding: 0, resource: { buffer: this.sceneUniformBuffer } },
          { binding: 1, resource: { buffer: this.lutStorageBuffer } }
        ]
      });

      this.entityBindGroup = this.device.createBindGroup({
        label: 'Entity-Dynamic-BG',
        layout: this.entityBindGroupLayout,
        entries: [
          {
            binding: 0,
            resource: {
              buffer: this.entityUniformBuffer,
              offset: 0,
              size: 96
            }
          }
        ]
      });

      // 7. Create Render Pipelines (Solid Triangle Mesh + Line Wireframes)
      this.meshPipeline = this.device.createRenderPipeline({
        label: 'EcoPunk-WGSL-Forge-Mesh-Pipeline',
        layout: pipelineLayout,
        vertex: {
          module: shaderModule,
          entryPoint: 'vs_main',
          buffers: [
            { arrayStride: 12, attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x3' }] },
            { arrayStride: 12, attributes: [{ shaderLocation: 1, offset: 0, format: 'float32x3' }] },
            { arrayStride: 12, attributes: [{ shaderLocation: 2, offset: 0, format: 'float32x3' }] },
            { arrayStride: 16, attributes: [{ shaderLocation: 3, offset: 0, format: 'float32x4' }] }
          ]
        },
        fragment: {
          module: shaderModule,
          entryPoint: 'fs_main',
          targets: [{ format: this.format }]
        },
        primitive: {
          topology: 'triangle-list',
          cullMode: 'none'
        },
        depthStencil: {
          format: 'depth24plus',
          depthWriteEnabled: true,
          depthCompare: 'less-equal'
        }
      });

      this.linePipeline = this.device.createRenderPipeline({
        label: 'EcoPunk-WGSL-Forge-Line-Pipeline',
        layout: pipelineLayout,
        vertex: {
          module: shaderModule,
          entryPoint: 'vs_line',
          buffers: [
            { arrayStride: 12, attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x3' }] }
          ]
        },
        fragment: {
          module: shaderModule,
          entryPoint: 'fs_line',
          targets: [{ format: this.format }]
        },
        primitive: {
          topology: 'line-list'
        },
        depthStencil: {
          format: 'depth24plus',
          depthWriteEnabled: false,
          depthCompare: 'less-equal'
        }
      });

      this.lineNoDepthPipeline = this.device.createRenderPipeline({
        label: 'EcoPunk-WGSL-Forge-XRay-Line-Pipeline',
        layout: pipelineLayout,
        vertex: {
          module: shaderModule,
          entryPoint: 'vs_line',
          buffers: [
            { arrayStride: 12, attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x3' }] }
          ]
        },
        fragment: {
          module: shaderModule,
          entryPoint: 'fs_line',
          targets: [{ format: this.format }]
        },
        primitive: {
          topology: 'line-list'
        },
        depthStencil: {
          format: 'depth24plus',
          depthWriteEnabled: false,
          depthCompare: 'always'
        }
      });

      // Setup Kinematics Compute Buffers (64 Fauna Agents * 8 floats = 512 floats)
      this.kinStateInBuf = this.device.createBuffer({
        size: 64 * 8 * 4,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
      });
      this.kinStateOutBuf = this.device.createBuffer({
        size: 64 * 8 * 4,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC
      });
      this.kinParamsBuf = this.device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
      });
      this.device.queue.writeBuffer(this.kinParamsBuf, 0, new Float32Array([0.016, 0.25, -0.05, 0.0]));

      if (this.kinematicsComputePipeline) {
        this.kinBindGroup = this.device.createBindGroup({
          layout: this.kinematicsComputePipeline.getBindGroupLayout(0),
          entries: [
            { binding: 0, resource: { buffer: this.kinStateInBuf } },
            { binding: 1, resource: { buffer: this.kinStateOutBuf } },
            { binding: 2, resource: { buffer: this.kinParamsBuf } }
          ]
        });
      }

      this.ready = true;
      this.lastInitError = null;
      return true;
    } catch (err) {
      this.lastInitError = String(err && (err.message || err));
      console.warn('WebGPU initialization fallback:', err);
      this.ready = false;
      return false;
    }
  }

  resize(width, height) {
    if (!this.canvas || !this.device) return;
    const w = Math.max(1, width);
    const h = Math.max(1, height);
    if (this.canvas.width !== w || this.canvas.height !== h || !this.depthTexture) {
      this.canvas.width = w;
      this.canvas.height = h;
      this.context.configure({
        device: this.device,
        format: this.format,
        alphaMode: 'opaque'
      });
      if (this.depthTexture) {
        this.depthTexture.destroy();
      }
      this.depthTexture = this.device.createTexture({
        size: [w, h, 1],
        format: 'depth24plus',
        usage: GPUTextureUsage.RENDER_ATTACHMENT
      });
      this.depthWidth = w;
      this.depthHeight = h;
    }
  }

  /**
   * Uploads a raw geometry object ({ positions, normals, colors, metas, indices }) into WebGPU buffers
   */
  getOrCreateGpuMesh(geomKeyObj, rawData) {
    if (this.meshCache.has(geomKeyObj)) {
      return this.meshCache.get(geomKeyObj);
    }
    const dev = this.device;

    const createVbo = (arr) => {
      const byteLen = Math.ceil(arr.byteLength / 4) * 4;
      const buf = dev.createBuffer({
        size: Math.max(16, byteLen),
        usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST
      });
      dev.queue.writeBuffer(buf, 0, arr.buffer, arr.byteOffset, arr.byteLength);
      return buf;
    };

    const posBuf = createVbo(rawData.positions);
    const normBuf = createVbo(rawData.normals);
    const colBuf = createVbo(rawData.colors);
    const metaBuf = createVbo(rawData.metas || rawData.windParams || rawData.animParams);

    // Ensure 4-byte alignment for index buffer
    let idxArray = rawData.indices;
    let indexFormat = rawData.useUint32 ? 'uint32' : 'uint16';
    if ( idxArray.byteLength % 4 !== 0 ) {
      const padded = new Uint16Array(idxArray.length + 1);
      padded.set(idxArray);
      idxArray = padded;
    }
    const idxBuf = dev.createBuffer({
      size: Math.max(16, idxArray.byteLength),
      usage: GPUBufferUsage.INDEX | GPUBufferUsage.COPY_DST
    });
    dev.queue.writeBuffer(idxBuf, 0, idxArray.buffer, idxArray.byteOffset, idxArray.byteLength);

    const gpuMesh = {
      posBuf,
      normBuf,
      colBuf,
      metaBuf,
      idxBuf,
      indexCount: rawData.indices.length,
      indexFormat,
      wirePositions: rawData.wirePositions || rawData.skeletonLines || null
    };
    this.meshCache.set(geomKeyObj, gpuMesh);
    return gpuMesh;
  }

  writeEntitySlot(slotIdx, modelMat, phase, speed, gaitCode, entityKind, lineColor = [0, 0.95, 0.88, 1]) {
    if (slotIdx >= this.maxEntities) return;
    const base = (slotIdx * this.entitySlotSize) / 4;
    this.entityStaging.set(modelMat, base);
    this.entityStaging[base + 16] = phase;
    this.entityStaging[base + 17] = speed;
    this.entityStaging[base + 18] = gaitCode;
    this.entityStaging[base + 19] = entityKind;
    this.entityStaging[base + 20] = lineColor[0];
    this.entityStaging[base + 21] = lineColor[1];
    this.entityStaging[base + 22] = lineColor[2];
    this.entityStaging[base + 23] = lineColor[3];
  }

  /**
   * Converts OpenGL [-1, 1] Z perspective matrix to WebGPU [0, 1] Z clip-space perspective matrix
   */
  createPerspectiveWebGPU(fovy, aspect, near, far) {
    const f = 1.0 / Math.tan(fovy / 2);
    const nf = 1.0 / (near - far);
    return new Float32Array([
      f / aspect, 0, 0, 0,
      0, f, 0, 0,
      0, 0, far * nf, -1,
      0, 0, near * far * nf, 0
    ]);
  }

  /**
   * Executes one full WebGPU Compute + Render frame
   */
  renderFrame(tr, eyePos, targetPos, sunDir, windVec) {
    if (!this.ready || !this.device || !this.context) return false;

    this.resize(tr.canvas.width, tr.canvas.height);
    const aspect = this.canvas.width / Math.max(1, this.canvas.height);
    const projMat = this.createPerspectiveWebGPU(46 * Math.PI / 180, aspect, 0.4, 380.0);
    const viewMat = tr.createLookAt(eyePos, targetPos, [0, 1, 0]);
    const identMat = tr.createIdentity();

    // 1. Write SceneUniforms (52 floats = 208 bytes)
    const sceneData = new Float32Array(52);
    sceneData.set(projMat, 0);
    sceneData.set(viewMat, 16);
    sceneData.set([eyePos[0], eyePos[1], eyePos[2], tr.time], 32);
    sceneData.set([sunDir[0], sunDir[1], sunDir[2], tr.getWaterLevelWorld()], 36);
    sceneData.set([windVec[0], windVec[1], tr.clipPlaneZ, tr.currentSector?.geology?.karst_fluting || 0.15], 40);
    if (tr.activeIsogloss) {
      sceneData.set([tr.activeIsogloss.center_x || 0, tr.activeIsogloss.radius || 16, tr.activeIsogloss.center_z || 0, 1.0], 44);
    } else {
      sceneData.set([0, 0, 0, 0], 44);
    }
    sceneData.set([tr.kpIndex, tr.imfBz, tr.observerMode, 1.0], 48);
    this.device.queue.writeBuffer(this.sceneUniformBuffer, 0, sceneData);

    // 2. Pack Entity Uniform Slots
    let slotCount = 0;
    const drawList = [];

    // Slot 0: Sky Dome
    const skyModel = tr.createTransform(eyePos[0], eyePos[1] - 20.0, eyePos[2], 0, 1.0);
    this.writeEntitySlot(slotCount, skyModel, 0, 0, 0, 4.0);
    drawList.push({ slot: slotCount++, mesh: this.getOrCreateGpuMesh(tr.skyGeom, tr.skyGeom.rawData) });

    let overlayLines = null;
    let overlayLineSlot = -1;
    let overlayNoDepth = false;

    if (tr.viewMode === 'specimen_loupe' && tr.loupeTarget.species) {
      const s = tr.specimenGrowthScale;
      // Pedestal
      const pedModel = tr.createTransform(0, -1.5, 0, 0, 0.14);
      this.writeEntitySlot(slotCount, pedModel, 0, 0, 0, 0.0);
      drawList.push({ slot: slotCount++, mesh: this.getOrCreateGpuMesh(tr.terrainGeom, tr.terrainGeom.rawData) });

      // Specimen
      const specModel = tr.createTransform(0, 0, 0, 0, s);
      if (tr.loupeTarget.type === 'flora') {
        const flMesh = tr.getUploadedFloraMesh(tr.loupeTarget.species, 'loupe');
        this.writeEntitySlot(slotCount, specModel, 0, 0, 0, 1.0, [0.0, 0.96, 1.0, 1.0]);
        drawList.push({ slot: slotCount, mesh: this.getOrCreateGpuMesh(flMesh, flMesh.rawData) });
        if (tr.showWireframeOverlay && flMesh.wirePositions) {
          overlayLines = flMesh.wirePositions;
          overlayLineSlot = slotCount;
          overlayNoDepth = true;
        }
        slotCount++;
      } else {
        const faMesh = tr.getUploadedFaunaMesh(tr.loupeTarget.species, true);
        const gaitCode = tr.getGaitNumericCode(tr.loupeTarget.species.kinematics?.locomotion_gait);
        this.writeEntitySlot(slotCount, specModel, tr.time * 4.5, 2.8, gaitCode, 2.0, [0.0, 0.96, 1.0, 1.0]);
        drawList.push({ slot: slotCount, mesh: this.getOrCreateGpuMesh(faMesh, faMesh.rawData) });
        if (tr.showWireframeOverlay && faMesh.wirePositions) {
          overlayLines = faMesh.wirePositions;
          overlayLineSlot = slotCount;
          overlayNoDepth = true;
        }
        slotCount++;
      }
    } else {
      // Slot 1: Terrain + 6-Layer CSIRO SLGA Subsurface Strata
      this.writeEntitySlot(slotCount, identMat, 0, 0, 0, 0.0);
      drawList.push({ slot: slotCount++, mesh: this.getOrCreateGpuMesh(tr.terrainGeom, tr.terrainGeom.rawData) });

      // Slot 2: Hydrologic Water Surface
      this.writeEntitySlot(slotCount, identMat, 0, 0, 0, 3.0);
      drawList.push({ slot: slotCount++, mesh: this.getOrCreateGpuMesh(tr.waterGeom, tr.waterGeom.rawData) });

      // Flora Instances
      for (const inst of tr.floraInstances) {
        if (inst.z > tr.clipPlaneZ || slotCount >= this.maxEntities - 70) continue;
        const flMesh = tr.getUploadedFloraMesh(inst.species, 'scene');
        const mMat = tr.createTransform(inst.x, inst.y, inst.z, inst.rotY, inst.scale);
        this.writeEntitySlot(slotCount, mMat, 0, 0, 0, 1.0);
        drawList.push({ slot: slotCount++, mesh: this.getOrCreateGpuMesh(flMesh, flMesh.rawData) });
      }

      // 64 Fauna Agents
      for (const ag of tr.faunaAgents) {
        if (ag.z > tr.clipPlaneZ || slotCount >= this.maxEntities - 2) continue;
        const faMesh = tr.getUploadedFaunaMesh(ag.species, false);
        const gaitCode = tr.getGaitNumericCode(ag.species.kinematics?.locomotion_gait);
        const mMat = tr.createTransform(ag.x, ag.y, ag.z, ag.yaw, 1.0);
        this.writeEntitySlot(slotCount, mMat, ag.phase, ag.speed, gaitCode, 2.0);
        drawList.push({ slot: slotCount++, mesh: this.getOrCreateGpuMesh(faMesh, faMesh.rawData) });
      }

      // Trophic & Sonic Wavefront Overlay Lines
      const lines = tr.collectOverlayLines();
      if (lines.length > 0 && slotCount < this.maxEntities) {
        overlayLines = new Float32Array(lines);
        this.writeEntitySlot(slotCount, identMat, 0, 0, 0, 0.0, [0.0, 0.95, 0.88, 1.0]);
        overlayLineSlot = slotCount++;
      }
    }

    // Upload active entity uniform slots in one DMA transfer
    this.device.queue.writeBuffer(
      this.entityUniformBuffer,
      0,
      this.entityStaging.buffer,
      0,
      slotCount * this.entitySlotSize
    );

    const encoder = this.device.createCommandEncoder({ label: 'EcoPunk-WGSL-Forge-Frame' });

    // 3. Dispatch WebGPU Kinematics Compute Pass (`nbody_step` from wgsl_forge::physics::kinematics)
    if (this.kinematicsComputePipeline && this.kinBindGroup) {
      const cPass = encoder.beginComputePass({ label: 'WGSL-Forge-Kinematics-Compute' });
      cPass.setPipeline(this.kinematicsComputePipeline);
      cPass.setBindGroup(0, this.kinBindGroup);
      cPass.dispatchWorkgroups(1, 1, 1);
      cPass.end();
      this.computeDispatchCount++;
    }

    // 4. Execute WebGPU Render Pass
    const textureView = this.context.getCurrentTexture().createView();
    const rPass = encoder.beginRenderPass({
      label: 'WGSL-Forge-Main-RenderPass',
      colorAttachments: [
        {
          view: textureView,
          clearValue: { r: 0.04, g: 0.07, b: 0.12, a: 1.0 },
          loadOp: 'clear',
          storeOp: 'store'
        }
      ],
      depthStencilAttachment: {
        view: this.depthTexture.createView(),
        depthClearValue: 1.0,
        depthLoadOp: 'clear',
        depthStoreOp: 'store'
      }
    });

    rPass.setPipeline(this.meshPipeline);
    rPass.setBindGroup(0, this.sceneBindGroup);

    for (const item of drawList) {
      const m = item.mesh;
      if (!m) continue;
      rPass.setBindGroup(1, this.entityBindGroup, [item.slot * this.entitySlotSize]);
      rPass.setVertexBuffer(0, m.posBuf);
      rPass.setVertexBuffer(1, m.normBuf);
      rPass.setVertexBuffer(2, m.colBuf);
      rPass.setVertexBuffer(3, m.metaBuf);
      rPass.setIndexBuffer(m.idxBuf, m.indexFormat);
      rPass.drawIndexed(m.indexCount, 1, 0, 0, 0);
    }

    // Render Wireframe / Trophic Arcs if active
    if (overlayLines && overlayLines.length >= 6 && overlayLineSlot >= 0) {
      const byteLen = Math.min(overlayLines.byteLength, 65536);
      this.device.queue.writeBuffer(this.dynamicLineBuffer, 0, overlayLines.buffer, overlayLines.byteOffset, byteLen);
      rPass.setPipeline(overlayNoDepth ? this.lineNoDepthPipeline : this.linePipeline);
      rPass.setBindGroup(0, this.sceneBindGroup);
      rPass.setBindGroup(1, this.entityBindGroup, [overlayLineSlot * this.entitySlotSize]);
      rPass.setVertexBuffer(0, this.dynamicLineBuffer);
      rPass.draw(Math.floor(byteLen / 12), 1, 0, 0);
    }

    rPass.end();
    this.device.queue.submit([encoder.finish()]);
    return true;
  }
}

window.WebGpuForgeEngine = WebGpuForgeEngine;
