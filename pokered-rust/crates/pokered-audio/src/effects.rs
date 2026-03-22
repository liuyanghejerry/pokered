//! Audio effect processors: vibrato, pitch slide, duty cycle rotation.
//!
//! These are applied per-frame to active channels, modifying frequency or
//! duty cycle based on the channel's current effect state.

use crate::sequencer::{ChannelFlags1, ChannelState};

// ── Vibrato ──────────────────────────────────────────────────────────────

/// Apply vibrato to a channel's frequency.
///
/// Replicates `Audio1_ApplyVibratoToFrequency` from engine_1.asm.
/// Returns the modified frequency (11-bit) to write to hardware.
///
/// The vibrato alternates between adding and subtracting from the base
/// frequency. The extent byte is split: upper nibble = upward amount,
/// lower nibble = downward amount.
pub fn apply_vibrato(channel: &mut ChannelState) -> Option<u16> {
    // If vibrato extent is zero, no vibrato
    if channel.vibrato.extent == 0 {
        return None;
    }

    // Delay phase: count down before vibrato starts
    if channel.vibrato.delay_counter > 0 {
        channel.vibrato.delay_counter -= 1;
        return None;
    }

    // Rate counter: tick down, only apply vibrato when it reaches zero
    let rate_counter = channel.vibrato.rate_counter();
    if rate_counter > 0 {
        channel.vibrato.set_rate_counter(rate_counter - 1);
        return None;
    }

    // Reload rate counter
    channel
        .vibrato
        .set_rate_counter(channel.vibrato.rate_reload());

    // Determine the vibrato amount based on direction
    let amount = if channel.flags1.contains(ChannelFlags1::VIBRATO_DOWN) {
        // Going down — use lower nibble (extent_down)
        channel.vibrato.extent_down()
    } else {
        // Going up — use upper nibble (extent_up)
        channel.vibrato.extent_up()
    };

    // Toggle direction for next tick
    channel.flags1.toggle(ChannelFlags1::VIBRATO_DOWN);

    // Apply to the saved frequency low byte
    let base_lo = channel.freq_lo_saved;
    let freq_hi = (channel.frequency >> 8) & 0x07;

    let new_lo;
    let new_hi;

    if channel.flags1.contains(ChannelFlags1::VIBRATO_DOWN) {
        // After toggling, if now DOWN, we just applied UP
        // Add amount to base frequency
        let sum = base_lo as u16 + amount as u16;
        new_lo = (sum & 0xFF) as u8;
        new_hi = freq_hi + (sum >> 8);
    } else {
        // After toggling, if now UP, we just applied DOWN
        // Subtract amount from base frequency
        if base_lo >= amount {
            new_lo = base_lo - amount;
            new_hi = freq_hi;
        } else {
            new_lo = base_lo.wrapping_sub(amount);
            new_hi = freq_hi.wrapping_sub(1);
        }
    };

    let result = ((new_hi & 0x07) << 8) | new_lo as u16;
    Some(result)
}

// ── Pitch Slide ──────────────────────────────────────────────────────────

/// Apply pitch slide to a channel.
///
/// Replicates `Audio1_ApplyPitchSlide` from engine_1.asm.
/// Returns `Some(new_freq)` if the frequency changed, or `None` if slide
/// has reached target and should be deactivated.
pub fn apply_pitch_slide(channel: &mut ChannelState) -> Option<u16> {
    if !channel.flags1.contains(ChannelFlags1::PITCH_SLIDE_ON) {
        return None;
    }

    let step = channel.pitch_slide.freq_step;
    let current = channel.frequency;
    let target = channel.pitch_slide.target_freq;

    let new_freq;

    if channel.flags1.contains(ChannelFlags1::PITCH_SLIDE_DEC) {
        // Sliding down (decreasing frequency register = higher pitch in GB terms)
        if current <= target {
            // Reached target
            channel.flags1.remove(ChannelFlags1::PITCH_SLIDE_ON);
            return None;
        }
        new_freq = current.saturating_sub(step);
        if new_freq <= target {
            channel.flags1.remove(ChannelFlags1::PITCH_SLIDE_ON);
            return Some(target);
        }
    } else {
        // Sliding up (increasing frequency register)
        if current >= target {
            channel.flags1.remove(ChannelFlags1::PITCH_SLIDE_ON);
            return None;
        }
        new_freq = current.saturating_add(step);
        if new_freq >= target {
            channel.flags1.remove(ChannelFlags1::PITCH_SLIDE_ON);
            return Some(target);
        }
    }

    channel.frequency = new_freq;
    Some(new_freq)
}

// ── Duty Cycle Rotation ──────────────────────────────────────────────────

/// Rotate the duty cycle pattern by 2 bits (one position in the 4-slot cycle).
///
/// The pattern byte holds 4 duty cycles (2 bits each):
/// bits 7-6 = slot 0, bits 5-4 = slot 1, bits 3-2 = slot 2, bits 1-0 = slot 3.
///
/// Each note, the byte is rotated left by 2 bits and the new current duty
/// is taken from bits 7-6.
///
/// Returns the new duty cycle value (0-3).
pub fn rotate_duty_cycle(channel: &mut ChannelState) -> u8 {
    // Rotate left by 2 bits (the low 2 bits wrap to the top)
    let pattern = channel.duty_cycle_pattern;
    let rotated = (pattern << 2) | (pattern >> 6);
    channel.duty_cycle_pattern = rotated;

    // The current duty is the top 2 bits
    (rotated >> 6) & 0x03
}
