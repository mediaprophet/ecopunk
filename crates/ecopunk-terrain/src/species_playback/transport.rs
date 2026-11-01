//! Ephemeral editorial cursor over an admitted native clip. This owns no world,
//! biological facts, actor geometry, audio queue or second animation engine.
use super::time::{seconds_to_tick, Tick, TICKS_PER_SECOND};
use crate::animal_motion::MotionClipDescriptor;

const MAX_SESSION_TICKS: i64 = 86_400 * TICKS_PER_SECOND;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaybackRate { pub numerator: u32, pub denominator: u32 }
impl Default for PlaybackRate {
    fn default() -> Self { Self { numerator: 1, denominator: 1 } }
}
impl PlaybackRate {
    pub fn validate(self) -> Result<(), &'static str> {
        if self.numerator == 0 || self.denominator == 0 || self.numerator > 1024
            || self.denominator > 1024 || self.numerator > 16 * self.denominator
            || self.denominator > 16 * self.numerator {
            Err("Playback rate is outside 1/16 through 16 with bounded rational terms")
        } else { Ok(()) }
    }
}

pub struct PlaybackTimeline {
    clip: String,
    duration: Tick,
    duration_s: f64,
    authored_loop: bool,
    looping: bool,
    playing: bool,
    position: Tick,
    timeline: Tick,
    rate: PlaybackRate,
    remainder: u32,
    epoch: u64,
    sequence: u64,
    dirty: bool,
}
#[derive(Clone, Copy)]
pub(super) struct PosePlan {
    pub timeline: Tick,
    pub position: Tick,
    pub sequence: u64,
    pub required: bool,
    remainder: u32,
    pub(super) playing: bool,
}
impl PlaybackTimeline {
    pub fn new(clip: MotionClipDescriptor<'_>) -> Result<Self, &'static str> {
        Ok(Self { clip: clip.name.into(), duration: seconds_to_tick(clip.duration_s)?,
            duration_s: clip.duration_s, authored_loop: clip.looping, looping: clip.looping,
            playing: false, position: Tick::ZERO, timeline: Tick::ZERO,
            rate: PlaybackRate::default(), remainder: 0, epoch: 1, sequence: 0, dirty: true })
    }
    pub fn clip(&self) -> &str { &self.clip }
    pub fn position(&self) -> Tick { self.position }
    pub fn timeline(&self) -> Tick { self.timeline }
    pub fn duration(&self) -> Tick { self.duration }
    pub fn epoch(&self) -> u64 { self.epoch }
    pub fn playing(&self) -> bool { self.playing }
    pub fn looping(&self) -> bool { self.looping }
    pub fn rate(&self) -> PlaybackRate { self.rate }
    pub fn sequence(&self) -> u64 { self.sequence }
    fn next_epoch(&self) -> Result<u64, &'static str> {
        self.epoch.checked_add(1).ok_or("Transport epoch exhausted")
    }
    pub fn play(&mut self) -> Result<(), &'static str> {
        if !self.looping && self.position == self.duration {
            return Err("Clip is at its end; seek explicitly before replay");
        }
        if !self.playing {
            self.epoch = self.next_epoch()?;
            self.playing = true;
            self.dirty = true;
        }
        Ok(())
    }
    pub fn pause(&mut self) -> Result<(), &'static str> {
        if self.playing {
            self.epoch = self.next_epoch()?;
            self.playing = false;
            self.dirty = true;
        }
        Ok(())
    }
    pub fn seek(&mut self, tick: Tick) -> Result<(), &'static str> {
        if tick.0 < 0 || tick > self.duration { return Err("Seek is outside the authored clip"); }
        let epoch = self.next_epoch()?;
        self.position = tick; self.timeline = tick; self.remainder = 0;
        self.epoch = epoch; self.dirty = true;
        Ok(())
    }
    pub fn set_rate(&mut self, rate: PlaybackRate) -> Result<(), &'static str> {
        rate.validate()?;
        if self.rate != rate {
            self.epoch = self.next_epoch()?;
            self.rate = rate; self.remainder = 0;
            self.dirty = true;
        }
        Ok(())
    }
    pub fn set_looping(&mut self, looping: bool) -> Result<(), &'static str> {
        if looping && !self.authored_loop {
            return Err("Source clip is not authored as loop-compatible");
        }
        if self.looping != looping {
            self.epoch = self.next_epoch()?;
            self.looping = looping; self.remainder = 0;
            self.dirty = true;
        }
        Ok(())
    }
    pub fn select(&mut self, clip: MotionClipDescriptor<'_>) -> Result<(), &'static str> {
        let epoch = self.next_epoch()?;
        let mut next = Self::new(clip)?;
        next.epoch = epoch;
        // Sequence remains monotone across seeks and clip changes; Core also
        // fences the changed epoch so delayed older frames cannot publish.
        next.sequence = self.sequence;
        next.rate = self.rate;
        *self = next;
        Ok(())
    }
    pub(super) fn plan(&self, elapsed: Tick) -> Result<PosePlan, &'static str> {
        if !(0..=TICKS_PER_SECOND).contains(&elapsed.0) {
            return Err("Live advance requires 0 through 1 second of admitted ticks");
        }
        let scaled = if self.playing {
            i128::from(elapsed.0) * i128::from(self.rate.numerator) + i128::from(self.remainder)
        } else { i128::from(self.remainder) };
        let step = i64::try_from(scaled / i128::from(self.rate.denominator))
            .map_err(|_| "Playback step overflow")?;
        let mut remainder = (scaled % i128::from(self.rate.denominator)) as u32;
        let mut timeline = self.timeline.checked_add(Tick(step)).ok_or("Timeline overflow")?;
        if timeline.0 > MAX_SESSION_TICKS { return Err("Playback session exceeds 24-hour clock profile"); }
        let mut position = self.position.checked_add(Tick(step)).ok_or("Playhead overflow")?;
        let mut playing = self.playing;
        if self.looping {
            // Preserve the explicitly sought endpoint until time actually moves.
            if step > 0 { position.0 %= self.duration.0; }
        } else if position >= self.duration {
            // Local clip position clamps; the editorial cursor may already be
            // several cycles ahead after looping was disabled. Consume only
            // the remaining clip interval, never jump the editorial clock back.
            let remaining = self.duration.checked_sub(self.position).ok_or("Clip interval overflow")?;
            timeline = self.timeline.checked_add(remaining).ok_or("Timeline overflow")?;
            position = self.duration; playing = false; remainder = 0;
        }
        let required = self.dirty || step > 0;
        let sequence = if required { self.sequence.checked_add(1).ok_or("Frame sequence exhausted")? }
            else { self.sequence };
        Ok(PosePlan { timeline, position, sequence, required, remainder, playing })
    }
    pub(super) fn sample_seconds(&self, plan: PosePlan) -> f64 {
        if plan.position == self.duration { self.duration_s }
        else { plan.position.0 as f64 / TICKS_PER_SECOND as f64 }
    }
    pub(super) fn commit(&mut self, plan: PosePlan) {
        self.timeline = plan.timeline; self.position = plan.position;
        self.sequence = plan.sequence; self.remainder = plan.remainder;
        self.playing = plan.playing; self.dirty = false;
    }
}
