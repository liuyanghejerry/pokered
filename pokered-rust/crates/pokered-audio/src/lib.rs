//! Game Boy APU (Audio Processing Unit) emulation for pokered.
//!
//! Models the 4-channel Game Boy sound hardware:
//! - Channel 1: Pulse wave with frequency sweep
//! - Channel 2: Pulse wave (no sweep)
//! - Channel 3: Programmable wave
//! - Channel 4: Noise (LFSR-based)
//!
//! The audio engine (sequencer) that reads music/SFX commands is in a separate
//! module; this crate provides the low-level sample generation.

pub mod apu;
pub mod channel;
pub mod commands;
pub mod effects;
pub mod music_data;
pub mod sequencer;
pub mod sfx_data;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod sequencer_tests;

#[cfg(test)]
mod music_data_tests;

#[cfg(test)]
mod sfx_data_tests;

// ── Constants ────────────────────────────────────────────────────────────

/// Game Boy CPU clock frequency in Hz (≈4.194304 MHz).
pub const CPU_CLOCK_HZ: u32 = 4_194_304;

/// The APU's frame sequencer runs at 512 Hz (CPU_CLOCK / 8192).
pub const FRAME_SEQUENCER_HZ: u32 = 512;

/// Number of CPU cycles per frame sequencer tick.
pub const CYCLES_PER_FRAME_SEQ_TICK: u32 = CPU_CLOCK_HZ / FRAME_SEQUENCER_HZ; // 8192

/// Standard output sample rate (can be resampled from GB rate).
pub const SAMPLE_RATE: u32 = 44_100;

/// Number of music channels in the pokered engine.
pub const NUM_MUSIC_CHANNELS: usize = 4;

/// Number of SFX channels (mirrors of the 4 music channels).
pub const NUM_SFX_CHANNELS: usize = 4;

/// Total logical channels (4 music + 4 SFX).
pub const NUM_CHANNELS: usize = NUM_MUSIC_CHANNELS + NUM_SFX_CHANNELS;

/// Number of notes in one octave.
pub const NUM_NOTES: usize = 12;

// ── Duty Cycle ───────────────────────────────────────────────────────────

/// Pulse wave duty cycle patterns.
/// Each pattern is 8 steps; 1 = high, 0 = low.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DutyCycle {
    /// 12.5% — waveform: 00000001
    Duty12 = 0,
    /// 25% — waveform: 10000001
    Duty25 = 1,
    /// 50% — waveform: 10000111
    Duty50 = 2,
    /// 75% — waveform: 01111110
    Duty75 = 3,
}

impl DutyCycle {
    /// The 8-step waveform pattern for this duty cycle.
    /// Returns the output level (0 or 1) for each of the 8 positions.
    pub const fn pattern(self) -> [u8; 8] {
        match self {
            DutyCycle::Duty12 => [0, 0, 0, 0, 0, 0, 0, 1],
            DutyCycle::Duty25 => [1, 0, 0, 0, 0, 0, 0, 1],
            DutyCycle::Duty50 => [1, 0, 0, 0, 0, 1, 1, 1],
            DutyCycle::Duty75 => [0, 1, 1, 1, 1, 1, 1, 0],
        }
    }

    /// Convert from a 2-bit value.
    pub const fn from_u8(val: u8) -> Self {
        match val & 0x03 {
            0 => DutyCycle::Duty12,
            1 => DutyCycle::Duty25,
            2 => DutyCycle::Duty50,
            _ => DutyCycle::Duty75,
        }
    }
}

impl Default for DutyCycle {
    fn default() -> Self {
        DutyCycle::Duty50
    }
}

// ── Note Pitch ───────────────────────────────────────────────────────────

/// Musical note names (within an octave).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NoteName {
    C = 0,
    Cs = 1, // C#
    D = 2,
    Ds = 3, // D#
    E = 4,
    F = 5,
    Fs = 6, // F#
    G = 7,
    Gs = 8, // G#
    A = 9,
    As = 10, // A#
    B = 11,
}

/// Frequency register values for base octave notes.
/// These are the raw 11-bit values written to NR13/NR14 (or NR23/NR24, NR33/NR34).
/// From audio/notes.asm — stored as big-endian words in the original.
/// The GB frequency formula: freq_hz = 131072 / (2048 - freq_reg)
pub const NOTE_FREQUENCIES: [u16; NUM_NOTES] = [
    0xF82C, // C
    0xF89D, // C#
    0xF907, // D
    0xF96B, // D#
    0xF9CA, // E
    0xFA23, // F
    0xFA77, // F#
    0xFAC7, // G
    0xFB12, // G#
    0xFB58, // A
    0xFB9B, // A#
    0xFBDA, // B
];

/// Extract the 11-bit frequency register value from a note table entry.
/// The original stores as big-endian 16-bit; the low 11 bits are the frequency.
pub const fn note_freq_reg(note: usize) -> u16 {
    NOTE_FREQUENCIES[note] & 0x07FF
}

// ── Wave Samples ─────────────────────────────────────────────────────────

/// Number of wave instruments available.
pub const NUM_WAVE_INSTRUMENTS: usize = 6; // wave0..wave5 (wave5 reused for 6,7,8)

/// Wave RAM is 16 bytes = 32 4-bit samples.
pub const WAVE_RAM_SIZE: usize = 16;
pub const WAVE_SAMPLES_PER_INSTRUMENT: usize = 32;

/// Wave instrument data from audio/wave_samples.asm.
/// Each instrument is 32 nibbles (stored as 16 bytes, 2 nibbles per byte).
/// These represent the waveform loaded into FF30-FF3F.
pub const WAVE_INSTRUMENTS: [[u8; WAVE_RAM_SIZE]; NUM_WAVE_INSTRUMENTS] = [
    // wave0: sawtooth-like
    pack_wave([
        0, 2, 4, 6, 8, 10, 12, 14, 15, 15, 15, 14, 14, 13, 13, 12, 12, 11, 10, 9, 8, 7, 6, 5, 4, 4,
        3, 3, 2, 2, 1, 1,
    ]),
    // wave1: slightly different sawtooth
    pack_wave([
        0, 2, 4, 6, 8, 10, 12, 14, 14, 15, 15, 15, 15, 14, 14, 14, 13, 13, 12, 11, 10, 9, 8, 7, 6,
        5, 4, 3, 2, 2, 1, 1,
    ]),
    // wave2: triangle-like
    pack_wave([
        1, 3, 6, 9, 11, 13, 14, 14, 14, 14, 15, 15, 15, 15, 14, 13, 13, 14, 15, 15, 15, 15, 14, 14,
        14, 14, 13, 11, 9, 6, 3, 1,
    ]),
    // wave3: modified sawtooth
    pack_wave([
        0, 2, 4, 6, 8, 10, 12, 13, 14, 15, 15, 14, 13, 14, 15, 15, 14, 14, 13, 12, 11, 10, 9, 8, 7,
        6, 5, 4, 3, 2, 1, 0,
    ]),
    // wave4: complex wave
    pack_wave([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 13, 14, 14, 15, 7, 7, 15, 14, 14, 13, 12, 10, 8, 7, 6,
        5, 4, 3, 2, 1, 0,
    ]),
    // wave5: used in Lavender Town / Pokemon Tower themes (actual data is from sfx stream)
    // The base definition; actual data varies by audio engine context.
    pack_wave([
        2, 1, 14, 2, 3, 3, 2, 8, 14, 1, 2, 2, 15, 15, 14, 10, 1, 0, 1, 4, 13, 12, 1, 0, 14, 3, 4,
        1, 5, 1, 7, 3,
    ]),
];

/// Pack 32 nibbles into 16 bytes (2 nibbles per byte, high nibble first).
const fn pack_wave(nibbles: [u8; 32]) -> [u8; 16] {
    let mut result = [0u8; 16];
    let mut i = 0;
    while i < 16 {
        result[i] = (nibbles[i * 2] << 4) | (nibbles[i * 2 + 1] & 0x0F);
        i += 1;
    }
    result
}

/// Unpack a 16-byte wave RAM into 32 nibble samples (0-15).
pub const fn unpack_wave(packed: &[u8; 16]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut i = 0;
    while i < 16 {
        result[i * 2] = packed[i] >> 4;
        result[i * 2 + 1] = packed[i] & 0x0F;
        i += 1;
    }
    result
}

// ── Hardware Channel Mapping ─────────────────────────────────────────────

/// Hardware channel index (0-3 for the 4 physical GB sound channels).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HwChannel {
    Pulse1 = 0,
    Pulse2 = 1,
    Wave = 2,
    Noise = 3,
}

impl HwChannel {
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(HwChannel::Pulse1),
            1 => Some(HwChannel::Pulse2),
            2 => Some(HwChannel::Wave),
            3 => Some(HwChannel::Noise),
            _ => None,
        }
    }

    /// The NR5x enable/disable bitmask for this channel (for rAUDTERM / NR51).
    /// Bit layout: bit 7-4 = left output, bit 3-0 = right output.
    pub const fn enable_mask(self) -> u8 {
        match self {
            HwChannel::Pulse1 => 0x11, // bit 0 + bit 4
            HwChannel::Pulse2 => 0x22, // bit 1 + bit 5
            HwChannel::Wave => 0x44,   // bit 2 + bit 6
            HwChannel::Noise => 0x88,  // bit 3 + bit 7
        }
    }

    /// The disable mask (complement of enable_mask).
    pub const fn disable_mask(self) -> u8 {
        !self.enable_mask()
    }
}

// ── Volume Envelope Direction ────────────────────────────────────────────

/// Direction of volume envelope change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnvelopeDirection {
    Decrease,
    Increase,
}

impl Default for EnvelopeDirection {
    fn default() -> Self {
        EnvelopeDirection::Decrease
    }
}

// ── Sweep Direction ──────────────────────────────────────────────────────

/// Direction of frequency sweep (Channel 1 only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SweepDirection {
    Increase,
    Decrease,
}

impl Default for SweepDirection {
    fn default() -> Self {
        SweepDirection::Increase
    }
}
