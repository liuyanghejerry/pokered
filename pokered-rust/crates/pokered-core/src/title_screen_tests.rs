use crate::game_state::{GameScreen, ScreenAction};
use crate::title_screen::*;
use pokered_data::species::Species;
use pokered_data::wild_data::GameVersion;

/// Helper to skip the Copyright phase and get to Init phase.
fn skip_copyright(ts: &mut TitleScreenState) {
    assert_eq!(ts.phase, TitlePhase::Copyright);
    // Skip all copyright frames
    for _ in 0..COPYRIGHT_FRAMES {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::Init);
}

#[test]
fn init_transitions_to_logo_bounce() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    assert_eq!(ts.phase, TitlePhase::Copyright);
    skip_copyright(&mut ts);
    let action = ts.update_frame(false);
    assert_eq!(action, ScreenAction::Continue);
    assert_eq!(ts.phase, TitlePhase::LogoBounce);
    assert!(ts.logo_visible);
    assert!(ts.player_visible);
}

#[test]
fn logo_bounce_progresses_scroll_y() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    skip_copyright(&mut ts);
    ts.update_frame(false);
    assert_eq!(ts.phase, TitlePhase::LogoBounce);

    let initial_scroll = ts.scroll_y;
    ts.update_frame(false);
    assert_ne!(ts.scroll_y, initial_scroll);
}

#[test]
fn logo_bounce_completes_to_logo_pause() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    skip_copyright(&mut ts);
    ts.update_frame(false);

    for _ in 0..LOGO_BOUNCE_TOTAL_FRAMES {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::LogoPause);
}

#[test]
fn logo_pause_lasts_correct_frames() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    skip_copyright(&mut ts);
    ts.update_frame(false);
    for _ in 0..LOGO_BOUNCE_TOTAL_FRAMES {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::LogoPause);

    for _ in 0..(LOGO_PAUSE_FRAMES - 1) {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::LogoPause);

    ts.update_frame(false);
    assert_eq!(ts.phase, TitlePhase::VersionScroll);
    assert!(ts.version_text_visible);
}

#[test]
fn version_scroll_updates_progress() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    skip_copyright(&mut ts);
    ts.update_frame(false);
    for _ in 0..LOGO_BOUNCE_TOTAL_FRAMES {
        ts.update_frame(false);
    }
    for _ in 0..LOGO_PAUSE_FRAMES {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::VersionScroll);

    ts.update_frame(false);
    assert!(ts.version_scroll_progress > 0.0);

    for _ in 0..VERSION_SCROLL_FRAMES {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::WaitingForInput);
    assert!((ts.version_scroll_progress - 1.0).abs() < f32::EPSILON);
}

#[test]
fn waiting_for_input_cycles_mons() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    advance_to_waiting(&mut ts);

    let first_mon = ts.current_mon;
    for _ in 0..MON_DISPLAY_FRAMES {
        ts.update_frame(false);
    }
    assert_ne!(ts.current_mon, first_mon);
}

#[test]
fn button_press_during_waiting_starts_cry() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    advance_to_waiting(&mut ts);

    let action = ts.update_frame(true);
    assert_eq!(action, ScreenAction::Continue);
    assert_eq!(ts.phase, TitlePhase::PlayingCry);
}

#[test]
fn cry_then_fadeout_then_transition() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    advance_to_waiting(&mut ts);
    ts.update_frame(true);
    assert_eq!(ts.phase, TitlePhase::PlayingCry);

    for _ in 0..CRY_PLAY_FRAMES {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::FadeOut);

    for i in 0..FADE_OUT_FRAMES {
        let action = ts.update_frame(false);
        if i < FADE_OUT_FRAMES - 1 {
            assert_eq!(action, ScreenAction::Continue);
        } else {
            assert_eq!(action, ScreenAction::Transition(GameScreen::MainMenu));
        }
    }
    assert_eq!(ts.phase, TitlePhase::Done);
}

#[test]
fn done_always_returns_transition() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    advance_to_waiting(&mut ts);
    ts.update_frame(true);
    for _ in 0..CRY_PLAY_FRAMES {
        ts.update_frame(false);
    }
    for _ in 0..FADE_OUT_FRAMES {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::Done);

    let action = ts.update_frame(false);
    assert_eq!(action, ScreenAction::Transition(GameScreen::MainMenu));
}

#[test]
fn fade_progress_only_during_fadeout() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    advance_to_waiting(&mut ts);
    assert!((ts.fade_progress() - 0.0).abs() < f32::EPSILON);

    ts.update_frame(true);
    for _ in 0..CRY_PLAY_FRAMES {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::FadeOut);

    ts.update_frame(false);
    assert!(ts.fade_progress() > 0.0);
}

#[test]
fn reset_restores_initial_state() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    advance_to_waiting(&mut ts);
    ts.update_frame(true);

    ts.reset();
    assert_eq!(ts.phase, TitlePhase::Copyright);
    assert_eq!(ts.scroll_y, 64);
    assert_eq!(ts.current_mon, Species::Charmander);
    assert!(!ts.player_visible);
    assert!(!ts.logo_visible);
    assert!(!ts.version_text_visible);
}

#[test]
fn red_vs_blue_first_mon_differs() {
    let red = TitleScreenState::new(GameVersion::Red);
    let blue = TitleScreenState::new(GameVersion::Blue);
    assert_eq!(red.current_mon, Species::Charmander);
    assert_eq!(blue.current_mon, Species::Squirtle);
}

#[test]
fn red_vs_blue_mon_lists_differ() {
    let red = TitleScreenState::new(GameVersion::Red);
    let blue = TitleScreenState::new(GameVersion::Blue);
    assert_ne!(red.title_mons(), blue.title_mons());
}

#[test]
fn pick_new_mon_always_differs_from_current() {
    let mut ts = TitleScreenState::new(GameVersion::Red);
    advance_to_waiting(&mut ts);

    for _ in 0..50 {
        let prev = ts.current_mon;
        for _ in 0..MON_DISPLAY_FRAMES {
            ts.update_frame(false);
        }
        assert_ne!(ts.current_mon, prev);
    }
}

fn advance_to_waiting(ts: &mut TitleScreenState) {
    skip_copyright(ts);
    ts.update_frame(false);
    for _ in 0..LOGO_BOUNCE_TOTAL_FRAMES {
        ts.update_frame(false);
    }
    for _ in 0..LOGO_PAUSE_FRAMES {
        ts.update_frame(false);
    }
    for _ in 0..VERSION_SCROLL_FRAMES {
        ts.update_frame(false);
    }
    assert_eq!(ts.phase, TitlePhase::WaitingForInput);
}
