//! Admission over the existing hypermedia timebase. Its compatibility floor/
//! saturation rules are never interpreted as exact production timestamps.
pub use timeline_editing::{FrameRate, Tick, TICKS_PER_SECOND};

pub fn seconds_to_tick(seconds: f64) -> Result<Tick, &'static str> {
    if !seconds.is_finite() || !(0.05..=30.).contains(&seconds) {
        return Err("Clip duration is outside the admitted motion profile");
    }
    // Authored motion currently supplies Float64 seconds. Record its nearest
    // representable timeline tick; do not call it an exact original decimal.
    Ok(Tick((seconds * TICKS_PER_SECOND as f64).round() as i64))
}

/// Sample positions include zero; clip-duration admission above remains stricter.
pub fn sample_seconds_to_tick(seconds: f64) -> Result<Tick, &'static str> {
    if !seconds.is_finite() || !(0.0..=30.0).contains(&seconds) {
        return Err("Sample position is outside the admitted motion profile");
    }
    Ok(Tick((seconds * TICKS_PER_SECOND as f64).round() as i64))
}

pub fn frame_tick(frame: i64, rate: FrameRate) -> Result<Tick, &'static str> {
    if frame < 0 || rate.num <= 0 || rate.den <= 0
        || rate.num > 1_000_000 || rate.den > 1_000_000 {
        return Err("Invalid rational frame clock");
    }
    let numerator = i128::from(frame).checked_mul(i128::from(rate.den))
        .and_then(|n| n.checked_mul(i128::from(TICKS_PER_SECOND)))
        .ok_or("Frame clock overflow")?;
    if numerator % i128::from(rate.num) != 0 {
        return Err("Frame boundary is not exact in the existing media timebase");
    }
    let exact = i64::try_from(numerator / i128::from(rate.num))
        .map_err(|_| "Frame clock overflow")?;
    let tick = rate.tick_of(frame);
    if tick.0 != exact { return Err("Frame clock saturated or rounded"); }
    Ok(tick)
}

pub fn sample_tick(sample: u64, rate: u32) -> Result<Tick, &'static str> {
    if rate == 0 { return Err("Audio sample rate is zero"); }
    let n = i128::from(sample).checked_mul(i128::from(TICKS_PER_SECOND))
        .ok_or("Sample clock overflow")?;
    if n % i128::from(rate) != 0 { return Err("Sample boundary is not exact in the media timebase"); }
    let exact = i64::try_from(n / i128::from(rate)).map_err(|_| "Sample clock overflow")?;
    let input = i64::try_from(sample).map_err(|_| "Sample clock overflow")?;
    let tick = Tick::from_units(input, i64::from(rate));
    if tick.0 != exact { return Err("Sample clock saturated or rounded"); }
    Ok(tick)
}
