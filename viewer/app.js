/**
 * Ecopunk 3D Holographic Terrain & Spatio-Temporal Linguistics Application
 */

document.addEventListener('DOMContentLoaded', () => {
  const canvas = document.getElementById('viewport-canvas');
  const audioCanvas = document.getElementById('audio-canvas');
  
  // Initialize Audio & 3D Terrain Engine
  window.sonicToponymEngine.init(audioCanvas);
  const renderer = new window.TerrainRenderer(canvas);

  // Toponyms Dataset (Australian Bioregional, Oral & Cadastral)
  const toponyms = [
    {
      id: 'toponym-1',
      name: 'Mparntwe (Sacred Waterhole)',
      lang: 'Central Arrernte',
      eraStart: 1800,
      eraEnd: 2030,
      x: -5.0,
      y: 2.2,
      z: 3.5,
      freq: 196.0, // G3
      waveType: 'sine',
      baseSpl: 88,
      meaning: 'Ancestral Caterpillar (Yeperenye) sacred perennial spring',
      stratum: 'Layer 1: Kaolinite & Silt (Sw = 78%)'
    },
    {
      id: 'toponym-2',
      name: 'Corymbia Crest (Ghost Gum Ridge)',
      lang: 'Botanical / Arrernte',
      eraStart: 1840,
      eraEnd: 2030,
      x: 3.2,
      y: 6.8,
      z: -4.1,
      freq: 293.66, // D4
      waveType: 'triangle',
      baseSpl: 82,
      meaning: 'Specimen Loupe: Bark reflectance 74.2% (Jakob-Hanika), Height 14.5m',
      stratum: 'Layer 4: Weathered Clasts (c = 34 kPa)'
    },
    {
      id: 'toponym-3',
      name: '1860 Crown Survey Baseline',
      lang: 'Victorian Cadastre',
      eraStart: 1860,
      eraEnd: 1925,
      x: 10.5,
      y: 1.5,
      z: 8.0,
      freq: 440.0, // A4
      waveType: 'sawtooth',
      baseSpl: 78,
      meaning: 'Surveyor Gunter Chain: 28.5°C thermal expansion correction +0.18%',
      stratum: 'Layer 2: Alluvial Gravel (Permeability 4.2e-11 m²)'
    },
    {
      id: 'toponym-4',
      name: 'Larapinta Gorge Pass',
      lang: 'Western Desert Orogeny',
      eraStart: 1800,
      eraEnd: 2030,
      x: -8.5,
      y: 4.8,
      z: -7.5,
      freq: 146.83, // D3
      waveType: 'sine',
      baseSpl: 90,
      meaning: 'Hydrological Exner sediment drainage & Biot poroelastic basalt',
      stratum: 'Layer 5: Fractured Basalt (Vp = 2840 m/s)'
    }
  ];

  // Diachronic Isogloss Varieties
  const isoglossVarieties = [
    {
      eraStart: 1800,
      eraEnd: 1880,
      name: 'Pre-Colonial Kinship Songline Lect',
      center_x: -3.0,
      center_z: 1.0,
      radius: 12.0,
      lectDetails: 'Continuous acoustic prosody; seasonal ceremony; oral custodianship'
    },
    {
      eraStart: 1880,
      eraEnd: 1950,
      name: 'Colonial Cadastral & Mining Boundary Shift',
      center_x: 4.0,
      center_z: 3.0,
      radius: 9.5,
      lectDetails: 'Gunter surveyor chains; mineral staking; anglicized toponym drift'
    },
    {
      eraStart: 1950,
      eraEnd: 2030,
      name: 'Contemporary Biocultural Dual-Register',
      center_x: 0.0,
      center_z: 0.0,
      radius: 16.0,
      lectDetails: 'Statutory Indigenous Land Use Agreement (ILUA) & Geospatial WFS'
    }
  ];

  // UI Elements
  const timelineSlider = document.getElementById('timeline-slider');
  const eraYearBadge = document.getElementById('era-year');
  const isoglossNameEl = document.getElementById('isogloss-name');
  const isoglossDetailsEl = document.getElementById('isogloss-details');
  const toponymListEl = document.getElementById('toponym-list');
  const fpsCounterEl = document.getElementById('fps-counter');
  const vramCounterEl = document.getElementById('vram-counter');
  const excStatusEl = document.getElementById('exc-status');
  const specimenHeightSlider = document.getElementById('specimen-height');
  const specimenHeightVal = document.getElementById('specimen-height-val');

  // Render Toponyms in Sidebar
  function renderToponymList() {
    toponymListEl.innerHTML = '';
    const currentYear = parseFloat(timelineSlider.value);

    toponyms.forEach(tp => {
      const activeInEra = currentYear >= tp.eraStart && currentYear <= tp.eraEnd;
      const item = document.createElement('div');
      item.className = `toponym-item ${activeInEra ? '' : 'toponym-inactive'}`;
      item.dataset.id = tp.id;
      item.innerHTML = `
        <div class="toponym-name">
          <span>${tp.name}</span>
          <span class="toponym-lang">${tp.lang}</span>
        </div>
        <div class="card-desc">${tp.meaning}</div>
        <div style="font-family: var(--font-mono); font-size: 0.65rem; color: var(--accent-cyan); margin-top: 4px;">
          📍 [${tp.x.toFixed(1)}, ${tp.y.toFixed(1)}, ${tp.z.toFixed(1)}] · Stratum: ${tp.stratum.split(':')[0]}
        </div>
      `;

      item.addEventListener('click', () => {
        document.querySelectorAll('.toponym-item').forEach(el => el.classList.remove('active'));
        item.classList.add('active');
        
        // Trigger spatial audio
        window.sonicToponymEngine.playToponymAcoustics(
          tp,
          { x: renderer.camera.target[0], y: renderer.camera.target[1], z: renderer.camera.target[2] },
          renderer.camera.velocity
        );

        // Center camera towards toponym
        renderer.camera.target = [tp.x, tp.y, tp.z];
      });

      toponymListEl.appendChild(item);
    });
  }

  // Update Timeline & Isogloss State
  function updateTimeline() {
    const year = parseFloat(timelineSlider.value);
    eraYearBadge.textContent = `${year} CE`;

    const activeIso = isoglossVarieties.find(iv => year >= iv.eraStart && year <= iv.eraEnd) || isoglossVarieties[0];
    isoglossNameEl.textContent = activeIso.name;
    isoglossDetailsEl.textContent = activeIso.lectDetails;

    renderer.activeIsogloss = activeIso;
    renderToponymList();
  }

  timelineSlider.addEventListener('input', updateTimeline);
  updateTimeline();

  // Observer Shader Mode Selection
  const modeBtns = document.querySelectorAll('.mode-btn');
  modeBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      modeBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      const mode = parseInt(btn.dataset.mode, 10);
      renderer.setObserverMode(mode);
    });
  });

  // Action Buttons
  const btnExcavate = document.getElementById('btn-excavate');
  btnExcavate.addEventListener('click', () => {
    const active = !btnExcavate.classList.contains('active');
    btnExcavate.classList.toggle('active', active);
    document.body.classList.toggle('excavate-active', active);
    renderer.setExcavationMode(active);
    excStatusEl.textContent = active ? 'EXCAVATE MODE: Click on 3D terrain to carve CSG cavity' : 'Nominal Stratigraphic Stack (Uncut)';
    excStatusEl.style.color = active ? 'var(--accent-magenta)' : 'var(--text-dim)';
  });

  window.onExcavationDone = (x, z, depth) => {
    excStatusEl.textContent = `CSG Excavation @ [${x.toFixed(1)}, ${z.toFixed(1)}]: Removed ΔV = ${(depth * 12.5).toFixed(1)} m³ · Cartoon Re-inflation queued`;
    excStatusEl.style.color = 'var(--accent-cyan)';
    // Flash strata column
    const stack = document.querySelector('.strata-stack');
    stack.style.filter = 'drop-shadow(0 0 10px #00f0ff)';
    setTimeout(() => { stack.style.filter = 'none'; }, 800);
  };

  const btnResetView = document.getElementById('btn-reset-view');
  btnResetView.addEventListener('click', () => {
    renderer.camera.rotX = 45 * Math.PI / 180;
    renderer.camera.rotY = 35 * Math.PI / 180;
    renderer.camera.dist = 55.0;
    renderer.camera.target = [0, 0, 0];
  });

  const btnAudioToggle = document.getElementById('btn-audio-toggle');
  btnAudioToggle.addEventListener('click', () => {
    const isMuted = window.sonicToponymEngine.toggleMute();
    btnAudioToggle.classList.toggle('active', !isMuted);
    btnAudioToggle.title = isMuted ? 'Unmute Spatial Audio' : 'Mute Spatial Audio';
  });

  // Specimen Loupe Slider
  specimenHeightSlider.addEventListener('input', (e) => {
    const val = parseFloat(e.target.value);
    specimenHeightVal.textContent = `${val.toFixed(1)} m`;
    // Scale Ghost Gum reflectance metrics
    const refl = (74.2 - (val - 0.5) * 0.15).toFixed(1);
    document.getElementById('specimen-refl').textContent = `${refl}%`;
  });

  // Telemetry Loop
  let frameCount = 0;
  let lastTime = performance.now();

  function loop(currentTime) {
    renderer.render();

    frameCount++;
    if (currentTime - lastTime >= 1000) {
      const fps = Math.round((frameCount * 1000) / (currentTime - lastTime));
      fpsCounterEl.textContent = `${fps} FPS`;
      frameCount = 0;
      lastTime = currentTime;

      // Simulated VRAM governor updates
      const baseVram = 42 + Math.sin(currentTime * 0.001) * 3.5;
      vramCounterEl.textContent = `${baseVram.toFixed(1)} / 256 MiB`;
    }

    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);
});
