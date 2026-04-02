//! Music/SFX sequencer — reads command streams and drives APU channels.
//!
//! Models the pokered audio engine (audio/engine_1.asm). The engine has
//! 8 logical channels: 4 music (CHAN1-4) and 4 SFX (CHAN5-8). SFX channels
//! override the corresponding music channel on the same hardware output.
//!
//! Each logical channel has its own command pointer, note delay, volume,
//! vibrato state, pitch slide state, etc.

use crate::apu::Apu;
use crate::commands::{self, Command};
use crate::{HwChannel, NUM_CHANNELS, NUM_MUSIC_CHANNELS, NUM_NOTES, WAVE_INSTRUMENTS};

// ── Channel index constants ──────────────────────────────────────────────

/// Music channel indices (0-3).
pub const CHAN1: usize = 0;
pub const CHAN2: usize = 1;
pub const CHAN3: usize = 2;
pub const CHAN4: usize = 3;

/// SFX channel indices (4-7) — mirror HW channels 0-3.
pub const CHAN5: usize = 4;
pub const CHAN6: usize = 5;
pub const CHAN7: usize = 6;
pub const CHAN8: usize = 7;

/// Map logical channel index (0-7) to hardware channel (0-3).
pub const fn hw_channel_for(ch: usize) -> usize {
    ch & 3
}

/// Is this a SFX channel? (index 4-7)
pub const fn is_sfx_channel(ch: usize) -> bool {
    ch >= NUM_MUSIC_CHANNELS
}

// ── Bitflags for channel state ───────────────────────────────────────────

bitflags::bitflags! {
    /// Per-channel flags (matches wChannelFlags1 in the original engine).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ChannelFlags1: u8 {
        /// Bit 0: perfect pitch — add 1 to frequency register.
        const PERFECT_PITCH     = 0x01;
        /// Bit 1: channel is in a sound_call subroutine.
        const SOUND_CALL_ACTIVE = 0x02;
        /// Bit 2: this is a noise or SFX channel.
        const NOISE_OR_SFX      = 0x04;
        /// Bit 3: vibrato direction (0=up, 1=down).
        const VIBRATO_DOWN      = 0x08;
        /// Bit 4: pitch slide is active.
        const PITCH_SLIDE_ON    = 0x10;
        /// Bit 5: pitch slide direction (0=increasing, 1=decreasing).
        const PITCH_SLIDE_DEC   = 0x20;
        /// Bit 6: rotate duty cycle pattern each note.
        const ROTATE_DUTY       = 0x40;
    }
}

bitflags::bitflags! {
    /// Per-channel flags2 (matches wChannelFlags2 in the original engine).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ChannelFlags2: u8 {
        /// Bit 0: execute_music — SFX channel interprets data like music
        ///        (enables vibrato/pitch slide processing on SFX).
        const EXECUTE_MUSIC = 0x01;
    }
}

impl Default for ChannelFlags1 {
    fn default() -> Self {
        Self::empty()
    }
}

impl Default for ChannelFlags2 {
    fn default() -> Self {
        Self::empty()
    }
}

// ── Vibrato State ────────────────────────────────────────────────────────

/// Per-channel vibrato parameters and running state.
#[derive(Debug, Clone, Default)]
pub struct VibratoState {
    /// Delay before vibrato starts (reload value in frames).
    pub delay_reload: u8,
    /// Current delay countdown.
    pub delay_counter: u8,
    /// Vibrato extent (upper nibble = up amount, lower nibble = down amount).
    pub extent: u8,
    /// Vibrato rate (upper nibble = reload, lower nibble = counter).
    pub rate: u8,
}

impl VibratoState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Upper extent (pitch increase amount).
    pub fn extent_up(&self) -> u8 {
        (self.extent >> 4) & 0x0F
    }

    /// Lower extent (pitch decrease amount).
    pub fn extent_down(&self) -> u8 {
        self.extent & 0x0F
    }

    /// Rate reload value.
    pub fn rate_reload(&self) -> u8 {
        (self.rate >> 4) & 0x0F
    }

    /// Rate counter value.
    pub fn rate_counter(&self) -> u8 {
        self.rate & 0x0F
    }

    /// Set the rate counter (low nibble).
    pub fn set_rate_counter(&mut self, val: u8) {
        self.rate = (self.rate & 0xF0) | (val & 0x0F);
    }
}

// ── Pitch Slide State ────────────────────────────────────────────────────

/// Per-channel pitch slide (portamento) parameters and running state.
#[derive(Debug, Clone, Default)]
pub struct PitchSlideState {
    /// Target frequency (11-bit).
    pub target_freq: u16,
    /// Current frequency (full 16-bit for fractional precision).
    pub current_freq: u16,
    /// Frequency step per tick (how much to add/subtract each frame).
    pub freq_step: u16,
    /// Fractional accumulator for sub-frame precision.
    pub freq_frac: u8,
    /// Length modifier for slide duration.
    pub length_modifier: u8,
}

impl PitchSlideState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// ── Channel State ────────────────────────────────────────────────────────

/// Full state for one logical audio channel (music or SFX).
///
/// Models the wChannel* variables from the original engine (one set per
/// channel, stored in the channel struct array).
#[derive(Debug, Clone)]
pub struct ChannelState {
    // ── Command stream ──
    /// The sound data (byte stream) for this channel.
    pub data: Vec<u8>,
    /// Current read position in `data`.
    pub ptr: usize,
    /// Saved position for sound_call return.
    pub return_ptr: usize,

    // ── Timing ──
    /// Frames remaining before the next command is read.
    pub delay_counter: u8,
    /// Fractional delay accumulator (sub-frame precision for tempo).
    pub delay_frac: u8,
    /// Note speed (from note_type command).
    pub note_speed: u8,
    /// Note length (from the current note/rest command).
    pub note_length: u8,

    // ── Pitch ──
    /// Current octave (0-7).
    pub octave: u8,
    /// Current note frequency (11-bit register value for HW).
    pub frequency: u16,
    /// Saved frequency low byte (for vibrato base).
    pub freq_lo_saved: u8,

    // ── Volume / Envelope ──
    /// Volume and fade packed as in note_type: high nibble = volume, low nibble = fade.
    pub volume_envelope: u8,

    // ── Duty cycle ──
    /// Current duty cycle (0-3).
    pub duty_cycle: u8,
    /// Packed duty cycle rotation pattern (4 x 2-bit, rotated left each note).
    pub duty_cycle_pattern: u8,

    // ── Effects ──
    pub vibrato: VibratoState,
    pub pitch_slide: PitchSlideState,

    // ── Flags ──
    pub flags1: ChannelFlags1,
    pub flags2: ChannelFlags2,

    // ── Loop ──
    /// Loop counter for sound_loop command.
    pub loop_counter: u8,

    // ── Identity ──
    /// The sound ID currently playing on this channel.
    pub sound_id: u8,
    /// Whether this channel is currently active.
    pub active: bool,
    /// Wave instrument index (for ch3/ch7).
    pub wave_instrument: u8,

    // ── Stereo ──
    /// Panning enable mask for this channel (bits in NR51 format).
    pub stereo_panning: u8,

    /// Set when a new note starts; cleared after APU trigger write.
    pub trigger: bool,

    /// Vibrato-modified frequency low byte for APU write (None = no vibrato active).
    pub vibrato_freq_lo: Option<u8>,

    /// Pitch sweep value for NR10 (channel 1 only). Set by PitchSweep command.
    /// Written to 0xFF10 when a note triggers on HW channel 0.
    pub pitch_sweep_value: u8,
}

impl ChannelState {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            ptr: 0,
            return_ptr: 0,
            delay_counter: 0,
            delay_frac: 0,
            note_speed: 1,
            note_length: 0,
            octave: 4,
            frequency: 0,
            freq_lo_saved: 0,
            volume_envelope: 0xF0,
            duty_cycle: 0,
            duty_cycle_pattern: 0,
            vibrato: VibratoState::default(),
            pitch_slide: PitchSlideState::default(),
            flags1: ChannelFlags1::default(),
            flags2: ChannelFlags2::default(),
            loop_counter: 0,
            sound_id: 0,
            active: false,
            wave_instrument: 0,
            stereo_panning: 0xFF,
            trigger: false,
            vibrato_freq_lo: None,
            pitch_sweep_value: 0,
        }
    }

    /// Reset channel to inactive state.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Read the next byte from the command stream, advancing the pointer.
    /// Returns None if at end of data.
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.ptr < self.data.len() {
            let b = self.data[self.ptr];
            self.ptr += 1;
            Some(b)
        } else {
            None
        }
    }

    /// Peek at the next byte without advancing.
    pub fn peek_byte(&self) -> Option<u8> {
        if self.ptr < self.data.len() {
            Some(self.data[self.ptr])
        } else {
            None
        }
    }

    /// Read a 16-bit little-endian value (low byte first, as in Z80 convention).
    pub fn read_u16_le(&mut self) -> Option<u16> {
        let lo = self.read_byte()? as u16;
        let hi = self.read_byte()? as u16;
        Some((hi << 8) | lo)
    }
}

impl Default for ChannelState {
    fn default() -> Self {
        Self::new()
    }
}

// ── Sequencer ────────────────────────────────────────────────────────────

/// The music/SFX sequencer.
///
/// Drives 8 logical channels (4 music + 4 SFX), reads command streams,
/// calculates note timing, applies effects, and writes to the APU.
///
/// Call `update_frame()` once per VBlank (~60 Hz).
#[derive(Debug, Clone)]
pub struct Sequencer {
    /// 8 logical channels: [0-3] = music, [4-7] = SFX.
    pub channels: [ChannelState; NUM_CHANNELS],

    /// Music tempo (16-bit, big-endian format: high byte = integer, low byte = fraction).
    pub music_tempo: u16,
    /// SFX tempo.
    pub sfx_tempo: u16,

    /// Global stereo panning (NR51 value).
    pub stereo_panning: u8,

    /// Whether music is currently playing.
    pub music_playing: bool,
    /// Whether SFX is currently playing.
    pub sfx_playing: bool,

    /// The music sound ID currently playing.
    pub current_music_id: u8,
    /// The SFX sound ID currently playing.
    pub current_sfx_id: u8,

    /// Frequency modifier for cries (added to base frequency).
    pub frequency_modifier: i16,
    /// Tempo modifier for cries (added to base tempo).
    pub tempo_modifier: i16,

    /// Fade counter for music fade out (0 = no fade).
    pub fade_counter: u8,
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            channels: std::array::from_fn(|_| ChannelState::new()),
            music_tempo: 0x0100, // default: 1.0 (256)
            sfx_tempo: 0x0100,
            stereo_panning: 0xFF,
            music_playing: false,
            sfx_playing: false,
            current_music_id: 0,
            current_sfx_id: 0,
            frequency_modifier: 0,
            tempo_modifier: 0,
            fade_counter: 0,
        }
    }

    /// Start playing music on channels 0-3.
    ///
    /// `channel_data` is a slice of up to 4 byte streams, one per channel.
    /// `sound_id` is the music ID.
    /// `tempo` is the initial tempo.
    pub fn play_music(&mut self, sound_id: u8, channel_data: &[Vec<u8>], tempo: u16) {
        // Stop any existing music
        self.stop_music();

        self.current_music_id = sound_id;
        self.music_tempo = tempo;
        self.music_playing = true;

        for (i, data) in channel_data.iter().enumerate() {
            if i >= NUM_MUSIC_CHANNELS {
                break;
            }
            let ch = &mut self.channels[i];
            ch.reset();
            ch.data = data.clone();
            ch.active = true;
            ch.sound_id = sound_id;
            ch.note_speed = 1;
            ch.octave = 4;
            ch.volume_envelope = 0xF0;
            ch.duty_cycle = 0;
            ch.stereo_panning = 0xFF;

            // Channel 3/4 get special flags
            if i == CHAN3 {
                ch.wave_instrument = 0;
            }
            if i == CHAN4 {
                ch.flags1.insert(ChannelFlags1::NOISE_OR_SFX);
            }
        }
    }

    /// Start playing a SFX on channels 4-7.
    ///
    /// `channel_data` maps SFX channels to their byte streams.
    /// `start_channel` is the first channel (0-3) the SFX uses.
    pub fn play_sfx(
        &mut self,
        sound_id: u8,
        channel_data: &[Vec<u8>],
        start_channel: usize,
        tempo: u16,
    ) {
        self.current_sfx_id = sound_id;
        self.sfx_tempo = tempo;
        self.sfx_playing = true;

        for (i, data) in channel_data.iter().enumerate() {
            let ch_idx = NUM_MUSIC_CHANNELS + start_channel + i;
            if ch_idx >= NUM_CHANNELS {
                break;
            }
            let ch = &mut self.channels[ch_idx];
            ch.reset();
            ch.data = data.clone();
            ch.active = true;
            ch.sound_id = sound_id;
            ch.note_speed = 1;
            ch.octave = 4;
            ch.volume_envelope = 0xF0;
            ch.duty_cycle = 0;
            ch.stereo_panning = 0xFF;
            ch.flags1.insert(ChannelFlags1::NOISE_OR_SFX);

            if hw_channel_for(ch_idx) == 2 {
                ch.wave_instrument = 0;
            }
        }
    }

    /// Stop all music channels.
    pub fn stop_music(&mut self) {
        for i in 0..NUM_MUSIC_CHANNELS {
            self.channels[i].reset();
        }
        self.music_playing = false;
        self.current_music_id = 0;
    }

    /// Stop all SFX channels.
    pub fn stop_sfx(&mut self) {
        for i in NUM_MUSIC_CHANNELS..NUM_CHANNELS {
            self.channels[i].reset();
        }
        self.sfx_playing = false;
        self.current_sfx_id = 0;
    }

    /// Stop everything.
    pub fn stop_all(&mut self) {
        self.stop_music();
        self.stop_sfx();
    }

    /// Update one frame (~60 Hz). Call this once per VBlank.
    ///
    /// For each active channel:
    /// 1. Decrement delay counter
    /// 2. If delay expired, execute commands until next note/rest
    /// 3. Apply per-frame effects (vibrato, pitch slide, duty rotation)
    /// 4. Write results to APU
    pub fn update_frame(&mut self, apu: &mut Apu) {
        for ch_idx in 0..NUM_CHANNELS {
            if !self.channels[ch_idx].active {
                continue;
            }

            self.update_channel(ch_idx);
        }

        // Apply channel states to APU
        self.apply_to_apu(apu);
    }

    /// Update a single channel for one frame.
    fn update_channel(&mut self, ch_idx: usize) {
        let ch = &mut self.channels[ch_idx];

        // Decrement delay
        if ch.delay_counter > 1 {
            ch.delay_counter -= 1;
            // Apply effects while waiting
            self.apply_effects(ch_idx);
            return;
        }

        // Delay expired (or first frame) — execute commands
        self.execute_commands(ch_idx);
    }

    /// Execute commands from the stream until a note, rest, or end is encountered.
    fn execute_commands(&mut self, ch_idx: usize) {
        // Safety: limit iterations to prevent infinite loops on bad data
        let mut max_commands = 256;

        loop {
            if max_commands == 0 {
                self.channels[ch_idx].active = false;
                return;
            }
            max_commands -= 1;

            let ch = &self.channels[ch_idx];
            if !ch.active || ch.ptr >= ch.data.len() {
                self.channels[ch_idx].active = false;
                return;
            }

            let pos = ch.ptr;
            let is_noise = hw_channel_for(ch_idx) == 3;
            let is_sfx = is_sfx_channel(ch_idx);
            let exec_music = ch.flags2.contains(ChannelFlags2::EXECUTE_MUSIC);
            let data_clone = ch.data.clone(); // Clone to avoid borrow conflict
            let (cmd, new_pos) =
                commands::decode_command(&data_clone, pos, is_noise, is_sfx, exec_music);
            self.channels[ch_idx].ptr = new_pos;

            match cmd {
                Command::Note { pitch, length } => {
                    self.handle_note(ch_idx, pitch, length);
                    return; // Note starts playing — wait for delay
                }
                Command::DrumNote { length, instrument } => {
                    self.handle_drum_note(ch_idx, length, instrument);
                    return;
                }
                Command::Rest { length } => {
                    self.handle_rest(ch_idx, length);
                    return;
                }
                Command::NoteType { speed, param } => {
                    self.handle_note_type(ch_idx, speed, param);
                }
                Command::Octave(oct) => {
                    self.channels[ch_idx].octave = oct;
                }
                Command::TogglePerfectPitch => {
                    self.channels[ch_idx]
                        .flags1
                        .toggle(ChannelFlags1::PERFECT_PITCH);
                }
                Command::Vibrato { delay, depth_rate } => {
                    self.handle_vibrato(ch_idx, delay, depth_rate);
                }
                Command::PitchSlide {
                    length_modifier,
                    octave_pitch,
                } => {
                    self.handle_pitch_slide(ch_idx, length_modifier, octave_pitch);
                }
                Command::DutyCycle(duty) => {
                    self.channels[ch_idx].duty_cycle = duty & 0x03;
                }
                Command::Tempo(tempo) => {
                    if is_sfx_channel(ch_idx) {
                        self.sfx_tempo = tempo;
                    } else {
                        self.music_tempo = tempo;
                    }
                }
                Command::StereoPanning(pan) => {
                    self.channels[ch_idx].stereo_panning = pan;
                }
                Command::UnknownEF(_) => {
                    // Mostly unused — ignore
                }
                Command::Volume(_vol) => {
                    // Write directly to NR50 — handled when applying to APU
                    // Store in channel for now; apply_to_apu will use it
                    // Actually, volume is a global command. We store the most recent.
                    self.channels[ch_idx].volume_envelope = self.channels[ch_idx].volume_envelope;
                    // no-op for now
                    // The actual NR50 write happens in apply_to_apu via a stored value
                    // For simplicity, we treat this as a sequencer-level volume
                    // that gets applied during apply_to_apu.
                    // TODO: Store global volume separately
                }
                Command::ExecuteMusic => {
                    self.channels[ch_idx]
                        .flags2
                        .toggle(ChannelFlags2::EXECUTE_MUSIC);
                }
                Command::DutyCyclePattern(pattern) => {
                    self.channels[ch_idx].duty_cycle_pattern = pattern;
                    self.channels[ch_idx]
                        .flags1
                        .insert(ChannelFlags1::ROTATE_DUTY);
                }
                Command::SoundCall { offset } => {
                    let ch = &mut self.channels[ch_idx];
                    ch.return_ptr = ch.ptr;
                    ch.ptr = offset as usize;
                    ch.flags1.insert(ChannelFlags1::SOUND_CALL_ACTIVE);
                }
                Command::SoundLoop { count, offset } => {
                    self.handle_sound_loop(ch_idx, count, offset);
                }
                Command::SoundRet => {
                    let ch = &mut self.channels[ch_idx];
                    if ch.flags1.contains(ChannelFlags1::SOUND_CALL_ACTIVE) {
                        ch.ptr = ch.return_ptr;
                        ch.flags1.remove(ChannelFlags1::SOUND_CALL_ACTIVE);
                    } else {
                        // End of channel data
                        ch.active = false;
                        self.check_sfx_end(ch_idx);
                        return;
                    }
                }
                Command::PitchSweep { param } => {
                    self.handle_pitch_sweep_sfx(ch_idx, param);
                }
                Command::SfxSquareNote {
                    length,
                    volume_envelope,
                    frequency,
                } => {
                    self.handle_sfx_square_note(ch_idx, length, volume_envelope, frequency);
                    return; // Note starts playing — wait for delay
                }
                Command::SfxNoiseNote {
                    length,
                    volume_envelope,
                    noise_params,
                } => {
                    self.handle_sfx_noise_note(ch_idx, length, volume_envelope, noise_params);
                    return; // Note starts playing — wait for delay
                }
                Command::EndOfData => {
                    self.channels[ch_idx].active = false;
                    self.check_sfx_end(ch_idx);
                    return;
                }
            }
        }
    }

    // ── Command Handlers ─────────────────────────────────────────────────

    /// Handle a note command: set frequency, calculate delay, apply effects setup.
    fn handle_note(&mut self, ch_idx: usize, pitch: u8, length: u8) {
        let ch = &mut self.channels[ch_idx];

        // If duty cycle rotation is enabled, rotate and use new duty
        if ch.flags1.contains(ChannelFlags1::ROTATE_DUTY) {
            ch.duty_cycle = crate::effects::rotate_duty_cycle(ch);
        }

        // Calculate frequency for this note + octave
        let freq = commands::calculate_frequency(pitch, ch.octave);
        ch.frequency = freq;
        ch.freq_lo_saved = (freq & 0xFF) as u8;
        ch.note_length = length;

        // Apply perfect pitch
        if ch.flags1.contains(ChannelFlags1::PERFECT_PITCH) {
            ch.frequency = ch.frequency.wrapping_add(1) & 0x07FF;
        }

        // Reset vibrato delay for this note
        ch.vibrato.delay_counter = ch.vibrato.delay_reload;

        // Calculate note delay from length, speed, and tempo
        let tempo = if is_sfx_channel(ch_idx) {
            self.sfx_tempo
        } else {
            self.music_tempo
        };
        let (delay, new_frac) =
            commands::calculate_delay(length, ch.note_speed, tempo, ch.delay_frac);
        ch.delay_counter = delay;
        ch.delay_frac = new_frac;
        ch.trigger = true;
    }

    /// Handle a drum note: triggers a noise instrument.
    fn handle_drum_note(&mut self, ch_idx: usize, length: u8, _instrument: u8) {
        let ch = &mut self.channels[ch_idx];
        ch.note_length = length;

        // Calculate delay
        let tempo = if is_sfx_channel(ch_idx) {
            self.sfx_tempo
        } else {
            self.music_tempo
        };
        let (delay, new_frac) =
            commands::calculate_delay(length, ch.note_speed, tempo, ch.delay_frac);
        ch.delay_counter = delay;
        ch.delay_frac = new_frac;

        ch.flags1.insert(ChannelFlags1::NOISE_OR_SFX);
        ch.trigger = true;
    }

    /// Handle a rest command: silence the channel for the given duration.
    fn handle_rest(&mut self, ch_idx: usize, length: u8) {
        let ch = &mut self.channels[ch_idx];
        ch.note_length = length;

        let tempo = if is_sfx_channel(ch_idx) {
            self.sfx_tempo
        } else {
            self.music_tempo
        };
        let (delay, new_frac) =
            commands::calculate_delay(length, ch.note_speed, tempo, ch.delay_frac);
        ch.delay_counter = delay;
        ch.delay_frac = new_frac;

        // Set frequency to 0 to indicate silence
        ch.frequency = 0;
        ch.trigger = true;
    }

    /// Handle note_type command.
    fn handle_note_type(&mut self, ch_idx: usize, speed: u8, param: u8) {
        let ch = &mut self.channels[ch_idx];
        ch.note_speed = speed;

        let hw = hw_channel_for(ch_idx);
        if hw == 2 {
            // Wave channel: low nibble = wave instrument index,
            // bits 5-4 = volume code (already shifted for NR32)
            ch.wave_instrument = param & 0x0F;
            ch.volume_envelope = param; // Store raw for APU write
        } else {
            // Pulse/noise: high nibble = volume, low nibble = fade
            ch.volume_envelope = param;
        }
    }

    /// Handle vibrato command.
    fn handle_vibrato(&mut self, ch_idx: usize, delay: u8, depth_rate: u8) {
        let ch = &mut self.channels[ch_idx];
        ch.vibrato.delay_reload = delay;
        ch.vibrato.delay_counter = delay;
        ch.vibrato.extent = depth_rate >> 4;

        // Pack extent: upper nibble = ceil(extent/2), lower nibble = floor(extent/2)
        let raw_extent = depth_rate >> 4;
        let up = (raw_extent + 1) / 2;
        let down = raw_extent / 2;
        ch.vibrato.extent = (up << 4) | down;

        ch.vibrato.rate = depth_rate & 0x0F;
        // Set rate as reload|counter — reload in upper nibble, counter in lower
        let rate_val = depth_rate & 0x0F;
        ch.vibrato.rate = (rate_val << 4) | rate_val;

        // Clear vibrato direction
        ch.flags1.remove(ChannelFlags1::VIBRATO_DOWN);
    }

    /// Handle pitch slide command.
    fn handle_pitch_slide(&mut self, ch_idx: usize, length_modifier: u8, octave_pitch: u8) {
        let ch = &mut self.channels[ch_idx];

        let target_octave = (octave_pitch >> 4) & 0x0F;
        let target_pitch = octave_pitch & 0x0F;

        // Calculate target frequency
        let target_freq = if (target_pitch as usize) < NUM_NOTES {
            commands::calculate_frequency(target_pitch, target_octave)
        } else {
            0
        };

        ch.pitch_slide.target_freq = target_freq;
        ch.pitch_slide.length_modifier = length_modifier;
        ch.flags1.insert(ChannelFlags1::PITCH_SLIDE_ON);

        // Determine direction
        if target_freq < ch.frequency {
            ch.flags1.insert(ChannelFlags1::PITCH_SLIDE_DEC);
        } else {
            ch.flags1.remove(ChannelFlags1::PITCH_SLIDE_DEC);
        }

        // The next command should be a note — we need to read it to get the
        // starting frequency and calculate the step. But in our decode-execute loop,
        // the note will be processed naturally. We pre-calculate the step here
        // based on current info.

        // For now, calculate step as simple linear interpolation
        let current = ch.frequency;
        let diff = if target_freq > current {
            target_freq - current
        } else {
            current - target_freq
        };

        // Step per frame — the original divides by (delay - length_modifier)
        // Since we don't know the delay yet (it depends on the next note),
        // we store the modifier and calculate the step when the note plays.
        ch.pitch_slide.freq_step = if diff > 0 { diff.max(1) } else { 0 };
        ch.pitch_slide.current_freq = current;
    }

    /// Handle sound_loop command.
    fn handle_sound_loop(&mut self, ch_idx: usize, count: u8, offset: u16) {
        let ch = &mut self.channels[ch_idx];

        if count == 0 {
            // Infinite loop
            ch.ptr = offset as usize;
        } else {
            // Counted loop
            if ch.loop_counter == 0 {
                // First time: set counter
                ch.loop_counter = count;
            }
            ch.loop_counter -= 1;
            if ch.loop_counter > 0 {
                ch.ptr = offset as usize;
            }
            // else: counter exhausted, continue past the loop
        }
    }

    /// Handle SFX pitch sweep — writes param directly to NR10 (0xFF10).
    /// Replicates engine_1.asm lines 628-642.
    fn handle_pitch_sweep_sfx(&mut self, ch_idx: usize, param: u8) {
        self.channels[ch_idx].pitch_sweep_value = param;
    }

    /// Handle SFX square note — directly sets volume, frequency, and triggers.
    /// Replicates engine_1.asm lines 575-618 (square_note path).
    fn handle_sfx_square_note(
        &mut self,
        ch_idx: usize,
        length: u8,
        volume_envelope: u8,
        frequency: u16,
    ) {
        let ch = &mut self.channels[ch_idx];

        let tempo = self.sfx_tempo;
        let (delay, new_frac) =
            commands::calculate_delay(length + 1, ch.note_speed, tempo, ch.delay_frac);
        ch.delay_counter = delay;
        ch.delay_frac = new_frac;

        ch.volume_envelope = volume_envelope;
        ch.frequency = frequency;
        ch.trigger = true;
    }

    /// Handle SFX noise note — directly sets volume, noise params, and triggers.
    /// Replicates engine_1.asm lines 575-626 (noise_note path).
    fn handle_sfx_noise_note(
        &mut self,
        ch_idx: usize,
        length: u8,
        volume_envelope: u8,
        noise_params: u8,
    ) {
        let ch = &mut self.channels[ch_idx];

        let tempo = self.sfx_tempo;
        let (delay, new_frac) =
            commands::calculate_delay(length + 1, ch.note_speed, tempo, ch.delay_frac);
        ch.delay_counter = delay;
        ch.delay_frac = new_frac;

        ch.volume_envelope = volume_envelope;
        ch.frequency = noise_params as u16;
        ch.trigger = true;
    }

    /// Check if a SFX channel has ended and resume corresponding music channel.
    fn check_sfx_end(&mut self, ch_idx: usize) {
        if !is_sfx_channel(ch_idx) {
            return;
        }

        // Check if all SFX channels are done
        let any_sfx_active = (NUM_MUSIC_CHANNELS..NUM_CHANNELS).any(|i| self.channels[i].active);

        if !any_sfx_active {
            self.sfx_playing = false;
            self.current_sfx_id = 0;
        }
    }

    // ── Effects ──────────────────────────────────────────────────────────

    /// Apply per-frame effects to a channel (vibrato, pitch slide).
    fn apply_effects(&mut self, ch_idx: usize) {
        let is_music = !is_sfx_channel(ch_idx);
        let has_execute_music = self.channels[ch_idx]
            .flags2
            .contains(ChannelFlags2::EXECUTE_MUSIC);

        // Effects only apply to music channels, or SFX with execute_music flag
        if !is_music && !has_execute_music {
            return;
        }

        if let Some(vibrato_lo) = crate::effects::apply_vibrato(&mut self.channels[ch_idx]) {
            self.channels[ch_idx].vibrato_freq_lo = Some(vibrato_lo);
        } else {
            self.channels[ch_idx].vibrato_freq_lo = None;
        }

        // Apply pitch slide
        if let Some(new_freq) = crate::effects::apply_pitch_slide(&mut self.channels[ch_idx]) {
            self.channels[ch_idx].frequency = new_freq;
        }
    }

    // ── APU Interface ────────────────────────────────────────────────────

    /// Apply all channel states to the APU hardware registers.
    ///
    /// For each HW channel, determine which logical channel takes priority
    /// (SFX over music), and write its frequency/volume/duty to the APU.
    fn apply_to_apu(&mut self, apu: &mut Apu) {
        let mut panning = 0u8;

        for hw in 0..4usize {
            let sfx_idx = hw + NUM_MUSIC_CHANNELS;
            let music_idx = hw;

            let active_idx = if self.channels[sfx_idx].active {
                sfx_idx
            } else if self.channels[music_idx].active {
                music_idx
            } else {
                self.silence_hw_channel(apu, hw);
                continue;
            };

            let hw_mask = HwChannel::from_u8(hw as u8)
                .map(|h| h.enable_mask())
                .unwrap_or(0);
            panning |= self.channels[active_idx].stereo_panning & hw_mask;

            let ch = &mut self.channels[active_idx];
            match hw {
                0 => Self::apply_pulse_channel(apu, ch, true),
                1 => Self::apply_pulse_channel(apu, ch, false),
                2 => Self::apply_wave_channel(apu, ch),
                3 => Self::apply_noise_channel(apu, ch),
                _ => {}
            }
        }

        apu.nr51 = panning;
    }

    /// Write pulse channel state to APU.
    /// On new note (trigger=true): writes all registers and clears trigger.
    /// On sustain: only updates frequency low byte (for vibrato/pitch slide).
    fn apply_pulse_channel(apu: &mut Apu, ch: &mut ChannelState, is_ch1: bool) {
        let freq = ch.frequency;
        let nrx3 = ch.vibrato_freq_lo.unwrap_or((freq & 0xFF) as u8);

        if ch.trigger {
            let duty = ch.duty_cycle;
            let vol_env = ch.volume_envelope;

            let nrx1 = (duty & 0x03) << 6;
            let nrx2 = vol_env;
            let nrx4 = 0x80 | ((freq >> 8) & 0x07) as u8;

            if is_ch1 {
                apu.write_register(0xFF10, ch.pitch_sweep_value);
                apu.write_register(0xFF11, nrx1);
                apu.write_register(0xFF12, nrx2);
                apu.write_register(0xFF13, nrx3);
                apu.write_register(0xFF14, nrx4);
            } else {
                apu.write_register(0xFF16, nrx1);
                apu.write_register(0xFF17, nrx2);
                apu.write_register(0xFF18, nrx3);
                apu.write_register(0xFF19, nrx4);
            }

            ch.trigger = false;
        } else {
            if is_ch1 {
                apu.write_register(0xFF13, nrx3);
            } else {
                apu.write_register(0xFF18, nrx3);
            }
        }
    }

    /// Write wave channel state to APU.
    fn apply_wave_channel(apu: &mut Apu, ch: &mut ChannelState) {
        let freq = ch.frequency;

        if ch.trigger {
            let wave_idx = ch.wave_instrument as usize;

            if wave_idx < WAVE_INSTRUMENTS.len() {
                apu.write_register(0xFF1A, 0x00);
                let wave_data = &WAVE_INSTRUMENTS[wave_idx];
                for (i, &byte) in wave_data.iter().enumerate() {
                    apu.write_register(0xFF30 + i as u16, byte);
                }
                apu.write_register(0xFF1A, 0x80);
            }

            let volume_code = (ch.volume_envelope >> 4) & 0x03;
            apu.write_register(0xFF1C, volume_code << 5);
            apu.write_register(0xFF1D, (freq & 0xFF) as u8);
            apu.write_register(0xFF1E, 0x80 | ((freq >> 8) & 0x07) as u8);

            ch.trigger = false;
        } else {
            let nrx3 = ch.vibrato_freq_lo.unwrap_or((freq & 0xFF) as u8);
            apu.write_register(0xFF1D, nrx3);
        }
    }

    /// Write noise channel state to APU.
    fn apply_noise_channel(apu: &mut Apu, ch: &mut ChannelState) {
        if ch.trigger {
            apu.write_register(0xFF21, ch.volume_envelope);

            let freq = ch.frequency;
            let shift = ((freq >> 4) & 0x0F) as u8;
            let divisor = (freq & 0x07) as u8;
            apu.write_register(0xFF22, (shift << 4) | divisor);
            apu.write_register(0xFF23, 0x80);

            ch.trigger = false;
        }
    }

    /// Silence a hardware channel.
    fn silence_hw_channel(&self, apu: &mut Apu, hw: usize) {
        match hw {
            0 => {
                apu.write_register(0xFF12, 0x00); // volume 0
            }
            1 => {
                apu.write_register(0xFF17, 0x00);
            }
            2 => {
                apu.write_register(0xFF1A, 0x00); // DAC off
            }
            3 => {
                apu.write_register(0xFF21, 0x00);
            }
            _ => {}
        }
    }

    // ── Query Methods ────────────────────────────────────────────────────

    /// Check if any channel is actively playing.
    pub fn is_playing(&self) -> bool {
        self.channels.iter().any(|ch| ch.active)
    }

    /// Check if a specific music channel is active.
    pub fn is_music_channel_active(&self, ch: usize) -> bool {
        ch < NUM_MUSIC_CHANNELS && self.channels[ch].active
    }

    /// Check if a specific SFX channel is active.
    pub fn is_sfx_channel_active(&self, ch: usize) -> bool {
        let idx = ch + NUM_MUSIC_CHANNELS;
        idx < NUM_CHANNELS && self.channels[idx].active
    }

    /// Get the current frequency of a logical channel.
    pub fn channel_frequency(&self, ch: usize) -> u16 {
        if ch < NUM_CHANNELS {
            self.channels[ch].frequency
        } else {
            0
        }
    }

    /// Get the current octave of a logical channel.
    pub fn channel_octave(&self, ch: usize) -> u8 {
        if ch < NUM_CHANNELS {
            self.channels[ch].octave
        } else {
            0
        }
    }
}

impl Default for Sequencer {
    fn default() -> Self {
        Self::new()
    }
}
