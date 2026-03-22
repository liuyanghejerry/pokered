use crate::audio_manager::{AudioManager, FadeState};
use crate::music_data::MusicId;
use crate::sfx_data::SfxId;

#[test]
fn test_new_defaults() {
    let mgr = AudioManager::new();
    assert_eq!(mgr.master_volume_left(), 7);
    assert_eq!(mgr.master_volume_right(), 7);
    assert_eq!(mgr.fade_state(), FadeState::None);
    assert!(!mgr.is_fading());
    assert!(!mgr.is_music_playing());
    assert!(!mgr.is_sfx_playing());
    assert_eq!(mgr.last_music_id(), None);
    assert_eq!(mgr.nr50(), 0x77);
}

#[test]
fn test_set_master_volume() {
    let mut mgr = AudioManager::new();
    mgr.set_master_volume(5, 3);
    assert_eq!(mgr.master_volume_left(), 5);
    assert_eq!(mgr.master_volume_right(), 3);
    assert_eq!(mgr.nr50(), 0x53);
}

#[test]
fn test_set_master_volume_clamps_to_7() {
    let mut mgr = AudioManager::new();
    mgr.set_master_volume(15, 10);
    assert_eq!(mgr.master_volume_left(), 7);
    assert_eq!(mgr.master_volume_right(), 7);
}

#[test]
fn test_play_music() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    assert!(mgr.is_music_playing());
    assert_eq!(mgr.last_music_id(), Some(MusicId::PALLET_TOWN));
    assert_eq!(mgr.fade_state(), FadeState::None);
}

#[test]
fn test_stop_music() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    mgr.stop_music();
    assert!(!mgr.is_music_playing());
    assert_eq!(mgr.last_music_id(), None);
}

#[test]
fn test_stop_all() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    mgr.play_sfx(SfxId::PressAB);
    mgr.stop_all();
    assert!(!mgr.is_music_playing());
    assert!(!mgr.is_sfx_playing());
    assert_eq!(mgr.last_music_id(), None);
}

#[test]
fn test_play_sfx() {
    let mut mgr = AudioManager::new();
    mgr.play_sfx(SfxId::PressAB);
    assert!(mgr.is_sfx_playing());
}

#[test]
fn test_stop_sfx() {
    let mut mgr = AudioManager::new();
    mgr.play_sfx(SfxId::PressAB);
    mgr.stop_sfx();
    assert!(!mgr.is_sfx_playing());
}

#[test]
fn test_play_music_with_fade_no_current_music_plays_immediately() {
    let mut mgr = AudioManager::new();
    mgr.play_music_with_fade(MusicId::PALLET_TOWN, 10);
    assert!(mgr.is_music_playing());
    assert_eq!(mgr.last_music_id(), Some(MusicId::PALLET_TOWN));
    assert_eq!(mgr.fade_state(), FadeState::None);
}

#[test]
fn test_play_music_with_fade_same_id_noop() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    mgr.play_music_with_fade(MusicId::PALLET_TOWN, 10);
    assert_eq!(mgr.fade_state(), FadeState::None);
}

#[test]
fn test_play_music_with_fade_starts_fading() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    mgr.play_music_with_fade(MusicId::BIKE_RIDING, 2);
    assert!(mgr.is_fading());
    assert_eq!(mgr.fade_state(), FadeState::FadingOut);
}

#[test]
fn test_fade_decrements_volume_over_time() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    mgr.play_music_with_fade(MusicId::BIKE_RIDING, 0);
    assert_eq!(mgr.master_volume_left(), 7);

    mgr.update_frame();
    assert_eq!(mgr.master_volume_left(), 6);
    assert_eq!(mgr.master_volume_right(), 6);

    mgr.update_frame();
    assert_eq!(mgr.master_volume_left(), 5);
}

#[test]
fn test_fade_with_counter_delays_volume_decrement() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    mgr.play_music_with_fade(MusicId::BIKE_RIDING, 2);

    // First two frames: counter counts down (2->1, 1->0)
    mgr.update_frame();
    assert_eq!(mgr.master_volume_left(), 7);
    mgr.update_frame();
    assert_eq!(mgr.master_volume_left(), 7);

    // Third frame: counter reload, volume decrements
    mgr.update_frame();
    assert_eq!(mgr.master_volume_left(), 6);
}

#[test]
fn test_fade_completes_and_switches_music() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    mgr.play_music_with_fade(MusicId::BIKE_RIDING, 0);

    // 8 frames to go from volume 7 to 0 (decrement each frame), then one more for completion
    for _ in 0..7 {
        assert!(mgr.is_fading());
        mgr.update_frame();
    }
    assert_eq!(mgr.master_volume_left(), 0);
    assert_eq!(mgr.master_volume_right(), 0);

    // Next frame triggers fade_complete
    mgr.update_frame();
    assert_eq!(mgr.fade_state(), FadeState::None);
    assert_eq!(mgr.last_music_id(), Some(MusicId::BIKE_RIDING));
    assert_eq!(mgr.master_volume_left(), 7);
    assert_eq!(mgr.master_volume_right(), 7);
}

#[test]
fn test_fade_with_no_queued_music_just_stops() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);

    mgr.fade_state = FadeState::FadingOut;
    mgr.fade_queued_music = None;
    mgr.fade_counter = 0;
    mgr.fade_counter_reload = 0;

    for _ in 0..8 {
        mgr.update_frame();
    }

    assert_eq!(mgr.fade_state(), FadeState::None);
    assert!(!mgr.is_music_playing());
}

#[test]
fn test_update_frame_no_crash_when_idle() {
    let mut mgr = AudioManager::new();
    for _ in 0..100 {
        mgr.update_frame();
    }
    assert_eq!(mgr.fade_state(), FadeState::None);
}

#[test]
fn test_update_frame_with_music() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    for _ in 0..60 {
        mgr.update_frame();
    }
    assert_eq!(mgr.master_volume_left(), 7);
}

#[test]
fn test_no_audio_fade_out_flag() {
    let mut mgr = AudioManager::new();
    assert!(!mgr.no_audio_fade_out());
    mgr.set_no_audio_fade_out(true);
    assert!(mgr.no_audio_fade_out());
}

#[test]
fn test_play_music_resets_fade_state() {
    let mut mgr = AudioManager::new();
    mgr.play_music(MusicId::PALLET_TOWN);
    mgr.play_music_with_fade(MusicId::BIKE_RIDING, 5);
    assert!(mgr.is_fading());

    mgr.play_music(MusicId::SURFING);
    assert_eq!(mgr.fade_state(), FadeState::None);
    assert_eq!(mgr.last_music_id(), Some(MusicId::SURFING));
}

#[test]
fn test_play_music_restores_volume() {
    let mut mgr = AudioManager::new();
    mgr.set_master_volume(2, 2);
    mgr.play_music(MusicId::PALLET_TOWN);
    assert_eq!(mgr.master_volume_left(), 7);
    assert_eq!(mgr.master_volume_right(), 7);
}

#[test]
fn test_nr50_reflects_volume() {
    let mut mgr = AudioManager::new();
    assert_eq!(mgr.nr50(), 0x77);
    mgr.set_master_volume(3, 5);
    assert_eq!(mgr.nr50(), 0x35);
}

#[test]
fn test_play_sfx_noise_instrument_single_channel() {
    let mut mgr = AudioManager::new();
    mgr.play_sfx(SfxId::NoiseInstrument01);
    assert!(mgr.is_sfx_playing());
}

#[test]
fn test_play_sfx_cry_multi_channel() {
    let mut mgr = AudioManager::new();
    mgr.play_sfx(SfxId::Cry00);
    assert!(mgr.is_sfx_playing());
}

#[test]
fn test_default_impl() {
    let mgr = AudioManager::default();
    assert_eq!(mgr.master_volume_left(), 7);
    assert_eq!(mgr.nr50(), 0x77);
}
