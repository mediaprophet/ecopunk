/**
 * EcoPunk — East Coast Australia 3D Ecological Continuum, Informatics & Multi-View Application
 */

document.addEventListener('DOMContentLoaded', async () => {
  const canvas = document.getElementById('viewport-canvas');
  const audioCanvas = document.getElementById('audio-canvas');

  window.sonicToponymEngine.init(audioCanvas);
  const renderer = new window.TerrainRenderer(canvas);
  window.ecopunkRenderer = renderer;

  // Load Master Catalog (from pre-bundled window.ECOPUNK_MASTER_DATA or fetch JSONs)
  let masterData = window.ECOPUNK_MASTER_DATA || null;
  if (!masterData) {
    try {
      const [secRes, flRes, faRes] = await Promise.all([
        fetch('../data/geospatial/sectors_terrain.json'),
        fetch('../data/catalog/flora_100.json'),
        fetch('../data/catalog/fauna_64.json')
      ]);
      const secJson = await secRes.json();
      const flJson = await flRes.json();
      const faJson = await faRes.json();
      masterData = {
        sectors: secJson.sectors || [],
        flora: flJson.species || [],
        fauna: faJson.species || [],
        quinCount: 7634
      };
    } catch (e) {
      console.error('Failed to load catalog JSONs:', e);
      masterData = { sectors: [], flora: [], fauna: [], quinCount: 0 };
    }
  }

  const sectors = masterData.sectors || [];
  const floraList = masterData.flora || [];
  const faunaList = masterData.fauna || [];

  // Update Header Telemetry Counts
  document.getElementById('flora-count-badge').textContent = `${floraList.length} Taxa`;
  document.getElementById('fauna-count-badge').textContent = `${faunaList.length} Active`;
  document.getElementById('quin-count-badge').textContent = `${(masterData.quinCount || 7634).toLocaleString()} .q42`;

  let currentSector = sectors[0] || { id: 'talbingo_pines', name: 'Talbingo Gorge', slga_strata: [], sonic_toponyms: [] };
  let selectedType = 'flora';
  let selectedSpecies = floraList[0] || null;

  // Initialize 3D Scene with first sector + all 105 Flora + 64 Fauna
  renderer.loadSectorAndEcosystem(currentSector, floraList, faunaList);

  // Load binary .10d / 16KB LUT substrates asynchronously over HTTP and refresh terrain
  renderer.loadBinarySubstrates().then(() => {
    renderer.buildTerrainMesh();
  });

  // ============================================================================
  // UI RENDERERS: FLORA LIST (105), FAUNA LIST (64), STRATA (6), INSPECTOR
  // ============================================================================

  const floraListEl = document.getElementById('flora-list');
  const faunaListEl = document.getElementById('fauna-list');
  const floraSearchEl = document.getElementById('flora-search');
  const floraGuildEl = document.getElementById('flora-guild-filter');
  const faunaSearchEl = document.getElementById('fauna-search');
  const faunaClassEl = document.getElementById('fauna-class-filter');

  function rgbCss(arr) {
    if (!arr || arr.length < 3) return 'rgb(120, 160, 120)';
    return `rgb(${Math.round(arr[0] * 255)}, ${Math.round(arr[1] * 255)}, ${Math.round(arr[2] * 255)})`;
  }

  function renderFloraCatalog() {
    const q = (floraSearchEl.value || '').toLowerCase().trim();
    const guild = floraGuildEl.value;
    floraListEl.innerHTML = '';

    const filtered = floraList.filter(sp => {
      if (guild !== 'all') {
        const gf = sp.morphology?.growth_form || '';
        if (guild === 'tree_fern' && !['tree_fern', 'grass_tree', 'palm_rosette', 'cycad_rosette'].includes(gf)) return false;
        else if (guild === 'tussock_grass' && !['tussock_grass', 'reed_sedge', 'alpine_cushion', 'wildflower_herb', 'mangrove_prop'].includes(gf)) return false;
        else if (!['tree_fern', 'tussock_grass'].includes(guild) && gf !== guild) return false;
      }
      if (!q) return true;
      return (
        sp.common_name.toLowerCase().includes(q) ||
        sp.scientific_name.toLowerCase().includes(q) ||
        (sp.indigenous_name || '').toLowerCase().includes(q) ||
        (sp.family || '').toLowerCase().includes(q)
      );
    });

    filtered.forEach(sp => {
      const row = document.createElement('div');
      const isSelected = selectedType === 'flora' && selectedSpecies?.id === sp.id;
      row.className = `species-row ${isSelected ? 'active' : ''}`;
      const elev = sp.elevation_range_m || [0, 1500];
      row.innerHTML = `
        <div class="sp-row-top">
          <span class="sp-common">${sp.common_name}</span>
          <span class="sp-badge">${sp.traits?.typical_height_m || 8}m · ${sp.family}</span>
        </div>
        <div class="sp-sci">${sp.scientific_name}</div>
        <div class="sp-meta">
          <span>${(sp.indigenous_name || sp.custodial_country || '').slice(0, 28)}</span>
          <span>${elev[0]}–${elev[1]}m</span>
        </div>
      `;
      row.addEventListener('click', () => {
        selectedType = 'flora';
        selectedSpecies = sp;
        renderer.setLoupeSpecimen('flora', sp);
        renderFloraCatalog();
        renderInspector();
      });
      floraListEl.appendChild(row);
    });
  }

  function renderFaunaCatalog() {
    const q = (faunaSearchEl.value || '').toLowerCase().trim();
    const cls = faunaClassEl.value;
    faunaListEl.innerHTML = '';

    const filtered = faunaList.filter(sp => {
      if (cls !== 'all') {
        if (cls === 'Aquatic' && !['Actinopterygii', 'Malacostraca'].includes(sp.class_name)) return false;
        else if (cls !== 'Aquatic' && sp.class_name !== cls) return false;
      }
      if (!q) return true;
      return (
        sp.common_name.toLowerCase().includes(q) ||
        sp.scientific_name.toLowerCase().includes(q) ||
        (sp.indigenous_name || '').toLowerCase().includes(q) ||
        (sp.family || '').toLowerCase().includes(q)
      );
    });

    filtered.forEach(sp => {
      const agentIdx = faunaList.findIndex(f => f.id === sp.id);
      const liveAgent = renderer.faunaAgents[agentIdx];
      const liveState = liveAgent ? liveAgent.behaviorState : (sp.ethology?.states?.[0] || 'ACTIVE');
      const isSelected = selectedType === 'fauna' && selectedSpecies?.id === sp.id;

      const row = document.createElement('div');
      row.className = `species-row ${isSelected ? 'active' : ''}`;
      row.innerHTML = `
        <div class="sp-row-top">
          <span class="sp-common">${sp.common_name}</span>
          <span class="sp-badge" style="color: var(--accent-amber); background: rgba(245, 158, 11, 0.15);">${liveState}</span>
        </div>
        <div class="sp-sci">${sp.scientific_name}</div>
        <div class="sp-meta">
          <span>${sp.kinematics?.locomotion_gait || 'bipedal'} · ${sp.biophysics?.adult_mass_kg || 1}kg</span>
          <span>${sp.biophysics?.bmr_watts || 12}W BMR</span>
        </div>
      `;
      row.addEventListener('click', () => {
        selectedType = 'fauna';
        selectedSpecies = sp;
        renderer.focusFaunaAgent(agentIdx >= 0 ? agentIdx : 0);
        renderFaunaCatalog();
        renderInspector();
      });
      faunaListEl.appendChild(row);
    });
  }

  function renderInspector() {
    if (!selectedSpecies) return;
    const sp = selectedSpecies;

    document.getElementById('inspector-type-badge').textContent =
      selectedType === 'flora' ? `FLORA · ${(sp.morphology?.growth_form || 'BOTANICAL').toUpperCase()}` : `FAUNA · ${(sp.class_name || 'ANIMALIA').toUpperCase()}`;

    document.getElementById('insp-common-name').textContent = sp.common_name;
    document.getElementById('insp-sci-name').textContent = sp.scientific_name;
    document.getElementById('insp-indigenous').textContent =
      `🪃 ${sp.indigenous_name || 'Traditional Custodial Taxon'} · ${sp.custodial_country || currentSector.indigenous_country}`;

    // Color swatches
    const swatchEl = document.getElementById('insp-swatches');
    swatchEl.innerHTML = '';
    const colors = selectedType === 'flora'
      ? [sp.morphology?.bark_rgb, sp.morphology?.foliage_rgb, sp.morphology?.flower_rgb]
      : [sp.anatomy_3d?.primary_rgb, sp.anatomy_3d?.secondary_rgb, sp.anatomy_3d?.accent_rgb];
    colors.forEach(c => {
      const d = document.createElement('span');
      d.className = 'swatch-dot';
      d.style.background = rgbCss(c);
      swatchEl.appendChild(d);
    });

    const gridEl = document.getElementById('insp-metrics-grid');
    const footerEl = document.getElementById('insp-extra-details');

    if (selectedType === 'flora') {
      const tr = sp.traits || {};
      const ae = sp.aeroelastic || {};
      const jh = sp.morphology?.jakob_hanika_leaf || [-0.2, 0.18, -0.3];
      gridEl.innerHTML = `
        <div class="metric-cell"><span class="m-label">Max / Typ Height</span><span class="m-val">${tr.max_height_m || 20}m / ${tr.typical_height_m || 12}m</span></div>
        <div class="metric-cell"><span class="m-label">Wood Density</span><span class="m-val">${tr.wood_density_kg_m3 || 720} kg/m³</span></div>
        <div class="metric-cell"><span class="m-label">LMA / SLA</span><span class="m-val">${tr.leaf_mass_per_area_g_m2 || 160} g/m² · ${tr.specific_leaf_area_m2_kg || 6.2}</span></div>
        <div class="metric-cell"><span class="m-label">Xylem P50 / Root</span><span class="m-val">${tr.xylem_p50_mpa || -4.2} MPa · ${tr.root_depth_m || 3.5}m</span></div>
        <div class="metric-cell"><span class="m-label">Fire Strategy</span><span class="m-val">${(tr.fire_response || 'resprouter').replace(/_/g, ' ')}</span></div>
        <div class="metric-cell"><span class="m-label">Sway / Flutter</span><span class="m-val">${ae.natural_sway_freq_hz || 0.9}Hz / ${ae.leaf_flutter_freq_hz || 4.8}Hz</span></div>
      `;
      footerEl.innerHTML = `
        <div><strong>auNSL / NCBI:</strong> NCBI:${sp.ncbi_id || '116962'} · GBIF:${sp.gbif_id || '3176787'} · Mycorrhiza: ${tr.mycorrhizal_type || 'ECM'}</div>
        <div><strong>Jakob-Hanika Leaf:</strong> [${jh.map(n => Number(n).toFixed(3)).join(', ')}] · <strong>Trophic:</strong> ${(sp.trophic_links || []).slice(0, 3).join(', ')}</div>
      `;
    } else {
      const bio = sp.biophysics || {};
      const kin = sp.kinematics || {};
      const eth = sp.ethology || {};
      const ac = sp.bioacoustics || {};
      gridEl.innerHTML = `
        <div class="metric-cell"><span class="m-label">Adult Mass / Len</span><span class="m-val">${bio.adult_mass_kg || 12} kg · ${bio.body_length_m || 0.9}m</span></div>
        <div class="metric-cell"><span class="m-label">Metabolic BMR</span><span class="m-val">${bio.bmr_watts || 35} W (${bio.body_temp_c || 36.5}°C)</span></div>
        <div class="metric-cell"><span class="m-label">Locomotion Gait</span><span class="m-val">${(kin.locomotion_gait || 'bipedal_hop').replace(/_/g, ' ')}</span></div>
        <div class="metric-cell"><span class="m-label">Walk / Sprint</span><span class="m-val">${kin.velocity_walk_ms || 1.4} / ${kin.velocity_sprint_ms || 12.0} m/s</span></div>
        <div class="metric-cell"><span class="m-label">Activity / Guild</span><span class="m-val">${eth.activity_cycle || 'crepuscular'} · ${eth.diet_guild || 'grazer'}</span></div>
        <div class="metric-cell"><span class="m-label">Bioacoustic Call</span><span class="m-val">${ac.call_fundamental_hz || 280} Hz (${ac.spl_db || 78} dB)</span></div>
      `;
      footerEl.innerHTML = `
        <div><strong>N3Logic Rule:</strong> ${eth.n3_rule_summary || 'Crepuscular foraging & roadside hazard flight'}</div>
        <div><strong>Trophic Foraging:</strong> ${(eth.forages_on || []).slice(0, 3).join(', ')} · <strong>IUCN:</strong> ${sp.iucn_status || 'Least Concern'}</div>
      `;
    }
  }

  function renderSectorStrataAndToponyms() {
    const sec = currentSector;
    document.getElementById('sector-subtitle').textContent =
      `${sec.region_zone || sec.name} · ${sec.indigenous_country || 'East Coast Country'} · Elev ${sec.elev_min_m}–${sec.elev_max_m}m AHD`;
    document.getElementById('geol-unit-banner').textContent =
      `${sec.geology?.unit_name || 'GSNSW Stratigraphic Unit'} (${sec.geology?.age || 'Palaeozoic'})`;

    document.getElementById('country-name').textContent = sec.indigenous_country || 'Ngarigo & Walgalu Country';
    document.getElementById('country-details').textContent =
      `${sec.geology?.lithology || ''} · Language: ${sec.language_group || 'Walgalu / Ngarigo'}.`;

    // Render 6 CSIRO SLGA Soil Horizons
    const strataEl = document.getElementById('strata-stack-list');
    strataEl.innerHTML = '';
    const strata = sec.slga_strata || [];
    strata.forEach((st, idx) => {
      const bar = document.createElement('div');
      bar.className = `stratum-bar stratum-${idx}`;
      bar.innerHTML = `
        <span>L${idx} (${st.horizon}): Clay ${st.clay_pct}% · c=${st.cohesion_kpa}kPa</span>
        <span>Sw ${st.water_saturation_pct}%</span>
      `;
      strataEl.appendChild(bar);
    });

    // Render Sonic Toponyms
    const topEl = document.getElementById('toponym-list');
    topEl.innerHTML = '';
    (sec.sonic_toponyms || []).forEach((tp, idx) => {
      const item = document.createElement('div');
      item.className = 'toponym-item';
      item.innerHTML = `
        <div class="toponym-name">
          <span>${tp.name}</span>
          <span class="toponym-lang">${tp.lang}</span>
        </div>
        <div class="card-desc">${tp.meaning}</div>
        <div class="sp-meta">
          <span>📍 [${tp.x.toFixed(1)}, ${tp.z.toFixed(1)}] · ${tp.freq} Hz</span>
          <span>${tp.stratum}</span>
        </div>
      `;
      item.addEventListener('click', () => {
        document.querySelectorAll('.toponym-item').forEach(el => el.classList.remove('active'));
        item.classList.add('active');
        const wy = renderer.sampleElevation(tp.x, tp.z);
        renderer.triggerSonicRipple(tp.x, wy, tp.z);
        window.sonicToponymEngine.playToponymAcoustics(
          { id: `tp_${idx}`, x: tp.x, y: wy, z: tp.z, freq: tp.freq, waveType: tp.waveType, baseSpl: tp.baseSpl },
          { x: renderer.camera.eye[0], y: renderer.camera.eye[1], z: renderer.camera.eye[2] },
          renderer.camera.velocity
        );
        renderer.camera.target = [tp.x, wy + 2.0, tp.z];
      });
      topEl.appendChild(item);
    });
  }

  // Initial UI render
  renderFloraCatalog();
  renderFaunaCatalog();
  renderInspector();
  renderSectorStrataAndToponyms();

  // ============================================================================
  // EVENT BINDINGS: SECTORS, VIEWS, SHADERS, TABS, SLIDERS
  // ============================================================================

  floraSearchEl.addEventListener('input', renderFloraCatalog);
  floraGuildEl.addEventListener('change', renderFloraCatalog);
  faunaSearchEl.addEventListener('input', renderFaunaCatalog);
  faunaClassEl.addEventListener('change', renderFaunaCatalog);

  // Sector Switcher
  document.getElementById('sector-select').addEventListener('change', (e) => {
    const secId = e.target.value;
    const found = sectors.find(s => s.id === secId);
    if (found) {
      currentSector = found;
      renderer.loadSectorAndEcosystem(currentSector, floraList, faunaList);
      renderSectorStrataAndToponyms();
      renderFloraCatalog();
      renderFaunaCatalog();
    }
  });

  // 6 View Mode Buttons
  const viewBtns = document.querySelectorAll('#view-modes-bar .mode-btn');
  viewBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      viewBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      const vMode = btn.dataset.view;
      renderer.setViewMode(vMode);
      const cutSlider = document.getElementById('slider-cutaway');
      const cutVal = document.getElementById('val-cutaway');
      if (vMode === 'subsurface_cutaway') {
        cutSlider.value = '0';
        cutVal.textContent = 'Z = 0m';
      } else {
        cutSlider.value = '60';
        cutVal.textContent = 'Uncut';
      }
    });
  });

  // 6 Observer Shader Mode Buttons
  const shaderBtns = document.querySelectorAll('#shader-modes-bar .mode-btn');
  shaderBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      shaderBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      renderer.setObserverMode(parseInt(btn.dataset.mode, 10));
    });
  });

  // Sidebar Tabs
  const tabBtns = document.querySelectorAll('.tab-btn');
  tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      tabBtns.forEach(b => b.classList.remove('active'));
      document.querySelectorAll('.tab-pane').forEach(p => p.classList.remove('active'));
      btn.classList.add('active');
      const tabName = btn.dataset.tab;
      document.getElementById(`tab-${tabName}`).classList.add('active');
    });
  });

  // Inspector Action Buttons
  document.getElementById('btn-open-loupe').addEventListener('click', () => {
    viewBtns.forEach(b => b.classList.toggle('active', b.dataset.view === 'specimen_loupe'));
    renderer.setViewMode('specimen_loupe');
  });

  document.getElementById('btn-track-entity').addEventListener('click', () => {
    if (selectedType === 'fauna') {
      viewBtns.forEach(b => b.classList.toggle('active', b.dataset.view === 'fauna_tracker'));
      renderer.setViewMode('fauna_tracker');
    } else {
      viewBtns.forEach(b => b.classList.toggle('active', b.dataset.view === 'aerial_orbit'));
      renderer.setViewMode('aerial_orbit');
      const inst = renderer.floraInstances.find(f => f.species.id === selectedSpecies?.id) || renderer.floraInstances[0];
      if (inst) {
        renderer.camera.target = [inst.x, inst.y + 3.0, inst.z];
        renderer.camera.dist = 24.0;
        renderer.triggerSonicRipple(inst.x, inst.y, inst.z, [0.1, 0.95, 0.55]);
      }
    }
  });

  const btnXray = document.getElementById('btn-toggle-xray');
  btnXray.addEventListener('click', () => {
    renderer.showWireframeOverlay = !renderer.showWireframeOverlay;
    btnXray.classList.toggle('active', renderer.showWireframeOverlay);
  });

  document.getElementById('btn-play-call').addEventListener('click', () => {
    if (!selectedSpecies) return;
    const freq = selectedType === 'fauna'
      ? (selectedSpecies.bioacoustics?.call_fundamental_hz || 320)
      : (selectedSpecies.acoustics?.wind_hiss_freq_hz || 440);
    window.sonicToponymEngine.playToponymAcoustics(
      { id: selectedSpecies.id, x: 0, y: 2, z: 0, freq, waveType: 'triangle', baseSpl: 82 },
      { x: 0, y: 2, z: 4 },
      [0, 0, 0]
    );
    renderer.triggerSonicRipple(renderer.camera.target[0], renderer.camera.target[1], renderer.camera.target[2]);
  });

  // Specimen Allometric Scale Slider
  const heightSlider = document.getElementById('specimen-height');
  const heightValEl = document.getElementById('specimen-height-val');
  heightSlider.addEventListener('input', (e) => {
    const mult = parseFloat(e.target.value);
    renderer.specimenGrowthScale = mult;
    const baseH = selectedType === 'flora'
      ? (selectedSpecies?.traits?.typical_height_m || 12.0)
      : (selectedSpecies?.biophysics?.body_length_m || 1.2);
    heightValEl.textContent = `${(baseH * mult).toFixed(1)} m (${mult.toFixed(2)}x)`;
  });

  // Microclimate & Space Weather Sliders
  const sliderTime = document.getElementById('slider-time');
  const sliderWind = document.getElementById('slider-wind');
  const sliderCutaway = document.getElementById('slider-cutaway');
  const sliderKp = document.getElementById('slider-kp');

  function updateEnvReadout() {
    const t = parseFloat(sliderTime.value);
    const hrs = Math.floor(t);
    const mins = Math.round((t - hrs) * 60).toString().padStart(2, '0');
    const timeStr = `${hrs.toString().padStart(2, '0')}:${mins}`;
    document.getElementById('val-time').textContent = timeStr;
    renderer.timeOfDayHours = t;

    const w = parseFloat(sliderWind.value);
    document.getElementById('val-wind').textContent = `${w.toFixed(1)} m/s`;
    renderer.windSpeedMs = w;

    const cut = parseFloat(sliderCutaway.value);
    document.getElementById('val-cutaway').textContent = cut >= 58 ? 'Uncut' : `Z=${cut}m`;
    renderer.clipPlaneZ = cut >= 58 ? 999.0 : cut;

    const kp = parseFloat(sliderKp.value);
    document.getElementById('val-kp').textContent = `Kp ${kp.toFixed(1)}`;
    renderer.kpIndex = kp;

    document.getElementById('weather-readout').textContent = `${timeStr} · ${w.toFixed(1)} m/s`;
  }

  sliderTime.addEventListener('input', updateEnvReadout);
  sliderWind.addEventListener('input', updateEnvReadout);
  sliderCutaway.addEventListener('input', updateEnvReadout);
  sliderKp.addEventListener('input', updateEnvReadout);

  // Floating Action Toolbar
  const btnExcavate = document.getElementById('btn-excavate');
  const excStatusEl = document.getElementById('exc-status');
  btnExcavate.addEventListener('click', () => {
    const active = !btnExcavate.classList.contains('active');
    btnExcavate.classList.toggle('active', active);
    document.body.classList.toggle('excavate-active', active);
    renderer.setExcavationMode(active);
    excStatusEl.textContent = active ? 'CLICK TERRAIN TO EXCAVATE' : 'GSNSW Verified';
    excStatusEl.style.color = active ? 'var(--accent-magenta)' : 'var(--accent-emerald)';
  });

  window.onExcavationDone = (x, z, depth) => {
    excStatusEl.textContent = `Cut @[${x.toFixed(0)},${z.toFixed(0)}] ΔV=${(depth * 14.2).toFixed(0)}m³`;
  };

  document.getElementById('btn-next-fauna').addEventListener('click', () => {
    if (!faunaList.length) return;
    const nextIdx = (renderer.trackedFaunaIndex + 1) % faunaList.length;
    selectedType = 'fauna';
    selectedSpecies = faunaList[nextIdx];
    renderer.focusFaunaAgent(nextIdx);
    viewBtns.forEach(b => b.classList.toggle('active', b.dataset.view === 'fauna_tracker'));
    renderer.setViewMode('fauna_tracker');
    renderFaunaCatalog();
    renderInspector();
  });

  document.getElementById('btn-reset-view').addEventListener('click', () => {
    viewBtns.forEach(b => b.classList.toggle('active', b.dataset.view === 'aerial_orbit'));
    renderer.setViewMode('aerial_orbit');
  });

  const btnAudioToggle = document.getElementById('btn-audio-toggle');
  btnAudioToggle.addEventListener('click', () => {
    const isMuted = window.sonicToponymEngine.toggleMute();
    btnAudioToggle.classList.toggle('active', !isMuted);
  });

  // Diachronic Timeline
  const timelineSlider = document.getElementById('timeline-slider');
  const eraYearBadge = document.getElementById('era-year');
  const isoglossNameEl = document.getElementById('isogloss-name');
  timelineSlider.addEventListener('input', () => {
    const yr = parseInt(timelineSlider.value, 10);
    eraYearBadge.textContent = `${yr} CE`;
    if (yr < 1850) {
      isoglossNameEl.textContent = 'Pre-Colonial Kinship Songline Lect';
      renderer.activeIsogloss = { center_x: -4, center_z: 2, radius: 18 };
    } else if (yr < 1960) {
      isoglossNameEl.textContent = 'Pastoral Cadastre & Snowy Hydro Survey';
      renderer.activeIsogloss = { center_x: 6, center_z: -5, radius: 22 };
    } else {
      isoglossNameEl.textContent = 'Dual-Register CARE & Qualia Living Commons';
      renderer.activeIsogloss = { center_x: 0, center_z: 0, radius: 28 };
    }
  });
  renderer.activeIsogloss = { center_x: 0, center_z: 0, radius: 28 };

  function applyScenario(opts = {}) {
    if (opts.sector) {
      const sec = sectors.find(s => s.id === opts.sector);
      if (sec) {
        currentSector = sec;
        document.getElementById('sector-select').value = sec.id;
        renderer.loadSectorAndEcosystem(currentSector, floraList, faunaList);
        renderSectorStrataAndToponyms();
      }
    }
    if (opts.flora) {
      const fl = floraList.find(f => f.id === opts.flora);
      if (fl) {
        selectedType = 'flora';
        selectedSpecies = fl;
        renderer.setLoupeSpecimen('flora', fl);
      }
    }
    if (opts.fauna) {
      const faIdx = faunaList.findIndex(f => f.id === opts.fauna);
      if (faIdx >= 0) {
        selectedType = 'fauna';
        selectedSpecies = faunaList[faIdx];
        renderer.focusFaunaAgent(faIdx);
      }
    }
    if (opts.view) {
      const vMode = opts.view;
      viewBtns.forEach(b => b.classList.toggle('active', b.dataset.view === vMode));
      renderer.setViewMode(vMode);
    }
    if (opts.shader !== undefined && opts.shader !== null) {
      const sMode = parseInt(opts.shader, 10);
      shaderBtns.forEach(b => b.classList.toggle('active', parseInt(b.dataset.mode, 10) === sMode));
      renderer.setObserverMode(sMode);
    }
    if (opts.tab) {
      const tName = opts.tab;
      tabBtns.forEach(b => b.classList.toggle('active', b.dataset.tab === tName));
      document.querySelectorAll('.tab-pane').forEach(p => p.classList.toggle('active', p.id === `tab-${tName}`));
    }
    if (opts.xray !== undefined) {
      renderer.showWireframeOverlay = Boolean(opts.xray);
      btnXray.classList.toggle('active', renderer.showWireframeOverlay);
    }
    renderFloraCatalog();
    renderFaunaCatalog();
    renderInspector();
    renderer.render();
  }

  // Support URL query parameter deep-linking & programmatic test hooks
  const params = new URLSearchParams(window.location.search);
  applyScenario({
    sector: params.get('sector'),
    flora: params.get('flora'),
    fauna: params.get('fauna'),
    view: params.get('view'),
    shader: params.get('shader'),
    tab: params.get('tab'),
    xray: params.get('xray') === '1' ? true : undefined
  });

  if (renderer.webgpuInitPromise) {
    await renderer.webgpuInitPromise;
  }
  renderer.render();

  window.ecopunkApp = {
    configureScenario: applyScenario,
    getStats: () => ({
      sectorCount: sectors.length,
      floraCount: floraList.length,
      faunaCount: faunaList.length,
      quinCount: masterData.quinCount || 7634,
      activeFloraInstances: renderer.floraInstances.length,
      activeFaunaAgents: renderer.faunaAgents.length,
      currentSector: currentSector.id,
      viewMode: renderer.viewMode,
      observerMode: renderer.observerMode,
      usingWebGPU: Boolean(renderer.webgpuEngine && renderer.webgpuEngine.ready),
      webgpuInitError: renderer.webgpuEngine ? renderer.webgpuEngine.lastInitError : 'no engine',
      wgslForgeSchemaVersion: window.ECOPUNK_WGSL_FORGE?.forge_schema_version || 0,
      nagaValidated: Boolean(window.ECOPUNK_WGSL_FORGE?.unified_pipeline_report?.naga_validated),
      computeDispatchCount: renderer.webgpuEngine ? renderer.webgpuEngine.computeDispatchCount : 0
    })
  };

  // Main Animation & Telemetry Loop
  let frameCount = 0;
  let lastFpsTime = performance.now();
  const fpsCounterEl = document.getElementById('fps-counter');
  const ethologyTextEl = document.getElementById('ethology-status-text');

  function loop(now) {
    renderer.render();
    frameCount++;

    if (now - lastFpsTime >= 1000) {
      const fps = Math.round((frameCount * 1000) / (now - lastFpsTime));
      fpsCounterEl.textContent = `${fps} FPS`;
      frameCount = 0;
      lastFpsTime = now;

      // Update live Fauna Ethology Ticker in footer
      if (renderer.faunaAgents.length) {
        const ag = renderer.faunaAgents[renderer.trackedFaunaIndex % renderer.faunaAgents.length];
        ethologyTextEl.textContent =
          `[${ag.species.common_name}] State: ${ag.behaviorState} · Speed: ${ag.speed.toFixed(1)} m/s · Pos: [${ag.x.toFixed(0)}, ${ag.y.toFixed(1)}, ${ag.z.toFixed(0)}]`;
      }
    }

    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);
});
