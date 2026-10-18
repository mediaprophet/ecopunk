/**
 * Ecopunk Sonic Toponym Streamer & Spatial Audio Synthesizer
 *
 * Implements inverse-square SPL attenuation (L = L0 - 20*log10(d)),
 * Doppler pitch shifts (f' = f * (1 + vr/c)), and real-time spectrum visualization.
 * Reference: ecopunk-terrain::geospatial::isogloss::SonicToponymStreamer
 */

class SonicToponymEngine {
  constructor() {
    this.ctx = null;
    this.analyser = null;
    this.activeSources = new Map();
    this.isMuted = false;
    this.masterGain = null;
    this.audioCanvas = null;
    this.canvasCtx = null;
    this.initialized = false;
  }

  init(canvasElement) {
    if (this.initialized) return;
    this.audioCanvas = canvasElement;
    if (this.audioCanvas) {
      this.canvasCtx = this.audioCanvas.getContext('2d');
    }
  }

  ensureContext() {
    if (!this.ctx) {
      const AudioContext = window.AudioContext || window.webkitAudioContext;
      this.ctx = new AudioContext();

      this.analyser = this.ctx.createAnalyser();
      this.analyser.fftSize = 64;

      this.masterGain = this.ctx.createGain();
      this.masterGain.gain.setValueAtTime(0.4, this.ctx.currentTime);

      this.masterGain.connect(this.analyser);
      this.analyser.connect(this.ctx.destination);

      this.initialized = true;
      this.startVisualizer();
    }
    if (this.ctx.state === 'suspended') {
      this.ctx.resume();
    }
  }

  playToponymAcoustics(toponym, cameraPos, cameraVelocity = [0, 0, 0]) {
    this.ensureContext();
    if (this.isMuted) return;

    // Stop existing sound for this toponym if any
    this.stopToponym(toponym.id);

    const now = this.ctx.currentTime;

    // Calculate 3D distance to camera
    const dx = cameraPos.x - toponym.x;
    const dy = cameraPos.y - toponym.y;
    const dz = cameraPos.z - toponym.z;
    const dist = Math.max(Math.sqrt(dx * dx + dy * dy + dz * dz), 1.0);

    // Inverse-square SPL attenuation: L = L0 - 20 * log10(d)
    const baseSpl = toponym.baseSpl || 85.0; // dB
    const currentSpl = Math.max(baseSpl - 20 * Math.log10(dist), 0);
    const linearGain = Math.min(Math.pow(10, (currentSpl - 90) / 20), 1.0);

    // Doppler shift: f' = f * (1 + vr / 343)
    const SPEED_OF_SOUND = 343.0;
    const dirX = dx / dist;
    const dirY = dy / dist;
    const dirZ = dz / dist;
    const vRadial = cameraVelocity[0] * dirX + cameraVelocity[1] * dirY + cameraVelocity[2] * dirZ;
    const dopplerFactor = Math.max(0.7, Math.min(1.4, 1.0 + (vRadial / SPEED_OF_SOUND)));

    // Create multi-harmonic drone & wave-field synthesis
    const fundamental = (toponym.freq || 220) * dopplerFactor;

    // Oscillator 1: Fundamental
    const osc1 = this.ctx.createOscillator();
    osc1.type = toponym.waveType || 'sine';
    osc1.frequency.setValueAtTime(fundamental, now);

    // Oscillator 2: Fifth harmonic
    const osc2 = this.ctx.createOscillator();
    osc2.type = 'triangle';
    osc2.frequency.setValueAtTime(fundamental * 1.5, now);

    // Oscillator 3: Sub-octave drone
    const osc3 = this.ctx.createOscillator();
    osc3.type = 'sine';
    osc3.frequency.setValueAtTime(fundamental * 0.5, now);

    // Lowpass resonance filter
    const filter = this.ctx.createBiquadFilter();
    filter.type = 'lowpass';
    filter.frequency.setValueAtTime(Math.min(fundamental * 4, 3000), now);
    filter.Q.setValueAtTime(3.0, now);

    // Local gain envelope
    const gainNode = this.ctx.createGain();
    gainNode.gain.setValueAtTime(0.001, now);
    gainNode.gain.exponentialRampToValueAtTime(linearGain * 0.5, now + 0.3);
    gainNode.gain.exponentialRampToValueAtTime(linearGain * 0.25, now + 2.0);
    gainNode.gain.exponentialRampToValueAtTime(0.0001, now + 4.5);

    // Routing
    osc1.connect(filter);
    osc2.connect(filter);
    osc3.connect(filter);
    filter.connect(gainNode);
    gainNode.connect(this.masterGain);

    osc1.start(now);
    osc2.start(now);
    osc3.start(now);

    osc1.stop(now + 4.6);
    osc2.stop(now + 4.6);
    osc3.stop(now + 4.6);

    this.activeSources.set(toponym.id, { osc1, osc2, osc3, gainNode });
  }

  stopToponym(id) {
    if (this.activeSources.has(id)) {
      try {
        const { osc1, osc2, osc3, gainNode } = this.activeSources.get(id);
        const now = this.ctx.currentTime;
        gainNode.gain.exponentialRampToValueAtTime(0.0001, now + 0.1);
        setTimeout(() => {
          try {
            osc1.stop();
            osc2.stop();
            osc3.stop();
          } catch(e) {}
        }, 120);
      } catch (e) {}
      this.activeSources.delete(id);
    }
  }

  toggleMute() {
    this.isMuted = !this.isMuted;
    if (this.masterGain && this.ctx) {
      this.masterGain.gain.setValueAtTime(this.isMuted ? 0 : 0.4, this.ctx.currentTime);
    }
    return this.isMuted;
  }

  startVisualizer() {
    if (!this.canvasCtx || !this.analyser) return;

    const bufferLength = this.analyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);

    const render = () => {
      requestAnimationFrame(render);
      if (!this.audioCanvas || !this.canvasCtx) return;

      const width = this.audioCanvas.width;
      const height = this.audioCanvas.height;

      this.analyser.getByteFrequencyData(dataArray);

      this.canvasCtx.fillStyle = 'rgba(7, 9, 14, 0.4)';
      this.canvasCtx.fillRect(0, 0, width, height);

      const barWidth = (width / bufferLength) * 1.5;
      let x = 0;

      for (let i = 0; i < bufferLength; i++) {
        const barHeight = (dataArray[i] / 255) * height * 0.9;

        // Gradient from cyan to magenta
        const ratio = i / bufferLength;
        const r = Math.floor(0 + ratio * 255);
        const g = Math.floor(240 * (1 - ratio));
        const b = 255;

        this.canvasCtx.fillStyle = `rgb(${r},${g},${b})`;
        this.canvasCtx.fillRect(x, height - barHeight, barWidth - 1, barHeight);

        x += barWidth;
      }
    };
    render();
  }
}

window.sonicToponymEngine = new SonicToponymEngine();
