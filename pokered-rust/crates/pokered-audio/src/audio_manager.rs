use crate::apu::Apu;
use crate::music_data::{self, MusicId};
use crate::sequencer::Sequencer;
use crate::sfx_data::{self, SfxId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeState {
    None,
    FadingOut,
}

pub struct AudioManager {
    pub sequencer: Sequencer,
    pub apu: Apu,

    master_volume_left: u8,
    master_volume_right: u8,

    pub(crate) fade_state: FadeState,
    pub(crate) fade_counter: u8,
    pub(crate) fade_counter_reload: u8,
    pub(crate) fade_queued_music: Option<MusicId>,

    no_audio_fade_out: bool,

    last_music_id: Option<MusicId>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            sequencer: Sequencer::new(),
            apu: Apu::new(),
            master_volume_left: 7,
            master_volume_right: 7,
            fade_state: FadeState::None,
            fade_counter: 0,
            fade_counter_reload: 0,
            fade_queued_music: None,
            no_audio_fade_out: false,
            last_music_id: None,
        }
    }

    pub fn master_volume_left(&self) -> u8 {
        self.master_volume_left
    }

    pub fn master_volume_right(&self) -> u8 {
        self.master_volume_right
    }

    pub fn set_master_volume(&mut self, left: u8, right: u8) {
        self.master_volume_left = left.min(7);
        self.master_volume_right = right.min(7);
        self.apply_master_volume();
    }

    pub fn fade_state(&self) -> FadeState {
        self.fade_state
    }

    pub fn last_music_id(&self) -> Option<MusicId> {
        self.last_music_id
    }

    pub fn set_no_audio_fade_out(&mut self, val: bool) {
        self.no_audio_fade_out = val;
    }

    pub fn no_audio_fade_out(&self) -> bool {
        self.no_audio_fade_out
    }

    pub fn play_music(&mut self, id: MusicId) {
        self.fade_state = FadeState::None;
        self.fade_queued_music = None;
        self.last_music_id = Some(id);

        let track = music_data::get_music_track(id);
        let mut channel_data = Vec::new();
        for ch_opt in &track.channels {
            if let Some(data) = ch_opt {
                channel_data.push(data.to_vec());
            }
        }
        self.sequencer
            .play_music(id as u8, &channel_data, track.tempo);
        self.master_volume_left = 7;
        self.master_volume_right = 7;
        self.apply_master_volume();
    }

    pub fn play_music_with_fade(&mut self, id: MusicId, fade_speed: u8) {
        if self.last_music_id == Some(id) {
            return;
        }

        if !self.sequencer.music_playing {
            self.play_music(id);
            return;
        }

        self.fade_state = FadeState::FadingOut;
        self.fade_counter = fade_speed;
        self.fade_counter_reload = fade_speed;
        self.fade_queued_music = Some(id);
    }

    pub fn play_sfx(&mut self, id: SfxId) {
        let track = sfx_data::get_sfx_track(id);
        let mut channel_data = Vec::new();
        let mut start_channel = 0usize;
        let mut found_first = false;

        for (hw_idx, ch_opt) in track.channels.iter().enumerate() {
            if let Some(data) = ch_opt {
                if !found_first {
                    start_channel = hw_idx;
                    found_first = true;
                }
                channel_data.push(data.to_vec());
            }
        }

        if !channel_data.is_empty() {
            self.sequencer
                .play_sfx(id as u8, &channel_data, start_channel, 0x0100);
        }
    }

    pub fn stop_music(&mut self) {
        self.sequencer.stop_music();
        self.last_music_id = None;
        self.fade_state = FadeState::None;
        self.fade_queued_music = None;
    }

    pub fn stop_sfx(&mut self) {
        self.sequencer.stop_sfx();
    }

    pub fn stop_all(&mut self) {
        self.sequencer.stop_all();
        self.last_music_id = None;
        self.fade_state = FadeState::None;
        self.fade_queued_music = None;
    }

    /// Call once per VBlank (~60 Hz). Ticks sequencer, processes fade, applies to APU.
    pub fn update_frame(&mut self) {
        self.process_fade();
        self.sequencer.update_frame(&mut self.apu);
        self.apply_master_volume();
    }

    fn process_fade(&mut self) {
        if self.fade_state != FadeState::FadingOut {
            if !self.no_audio_fade_out {
                self.apply_master_volume();
            }
            return;
        }

        if self.fade_counter > 0 {
            self.fade_counter -= 1;
            return;
        }

        self.fade_counter = self.fade_counter_reload;

        if self.master_volume_left == 0 && self.master_volume_right == 0 {
            self.fade_complete();
            return;
        }

        self.master_volume_left = self.master_volume_left.saturating_sub(1);
        self.master_volume_right = self.master_volume_right.saturating_sub(1);
        self.apply_master_volume();
    }

    fn fade_complete(&mut self) {
        self.fade_state = FadeState::None;

        self.sequencer.stop_all();

        if let Some(next_id) = self.fade_queued_music.take() {
            self.play_music(next_id);
        }
    }

    fn apply_master_volume(&mut self) {
        let nr50 = (self.master_volume_left << 4) | self.master_volume_right;
        self.apu.nr50 = nr50;
    }

    pub fn is_fading(&self) -> bool {
        self.fade_state == FadeState::FadingOut
    }

    pub fn is_music_playing(&self) -> bool {
        self.sequencer.music_playing
    }

    pub fn is_sfx_playing(&self) -> bool {
        self.sequencer.sfx_playing
    }

    pub fn nr50(&self) -> u8 {
        self.apu.nr50
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}
