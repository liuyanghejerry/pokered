mod game_flow_helpers;

use game_flow_helpers::*;
use pokered_core::game_state::*;
use pokered_core::main_menu::{MainMenuState, MenuInput};
use pokered_core::title_screen::{TitlePhase, TitleScreenState};
use pokered_data::wild_data::GameVersion;

#[test]
fn game_starts_at_copyright_splash() {
    let state = new_game_state_red();
    assert_eq!(state.screen, GameScreen::CopyrightSplash);
    assert_eq!(state.config.version, GameVersion::Red);
    assert!(state.config.battle_animation);
    assert_eq!(state.config.battle_style, BattleStyle::Shift);
    assert_eq!(state.config.text_speed, TextSpeed::Medium);
}

#[test]
fn title_screen_progresses_through_all_phases() {
    let mut title = TitleScreenState::new(GameVersion::Red);
    assert_eq!(title.phase, TitlePhase::Init);

    title.update_frame(false);
    assert_eq!(title.phase, TitlePhase::LogoBounce);

    while title.phase == TitlePhase::LogoBounce {
        title.update_frame(false);
    }
    assert_eq!(title.phase, TitlePhase::LogoPause);

    while title.phase == TitlePhase::LogoPause {
        title.update_frame(false);
    }
    assert_eq!(title.phase, TitlePhase::VersionScroll);

    while title.phase == TitlePhase::VersionScroll {
        title.update_frame(false);
    }
    assert_eq!(title.phase, TitlePhase::WaitingForInput);
}

#[test]
fn title_screen_button_press_triggers_cry_and_fade() {
    let mut title = TitleScreenState::new(GameVersion::Red);

    while title.phase != TitlePhase::WaitingForInput {
        title.update_frame(false);
    }

    let action = title.update_frame(true);
    assert_eq!(action, ScreenAction::Continue);
    assert_eq!(title.phase, TitlePhase::PlayingCry);

    while title.phase == TitlePhase::PlayingCry {
        title.update_frame(false);
    }
    assert_eq!(title.phase, TitlePhase::FadeOut);

    let mut transition = ScreenAction::Continue;
    while title.phase == TitlePhase::FadeOut {
        transition = title.update_frame(false);
    }
    assert_eq!(transition, ScreenAction::Transition(GameScreen::MainMenu));
    assert_eq!(title.phase, TitlePhase::Done);
}

#[test]
fn title_screen_blue_version_starts_with_squirtle() {
    let title = TitleScreenState::new(GameVersion::Blue);
    assert_eq!(title.current_mon, pokered_data::species::Species::Squirtle);
}

#[test]
fn main_menu_new_game_no_save() {
    let mut menu = MainMenuState::new(None);
    assert_eq!(menu.item_count(), 2);
    assert!(!menu.has_save);

    while !menu.init_delay_done {
        menu.update_frame(MenuInput::none());
    }

    let action = menu.update_frame(MenuInput {
        a: true,
        ..MenuInput::none()
    });
    assert_eq!(action, ScreenAction::Transition(GameScreen::OakSpeech));
    assert_eq!(menu.last_choice, Some(MainMenuChoice::NewGame));
}

#[test]
fn main_menu_with_save_file_shows_continue() {
    let summary = SaveFileSummary {
        player_name: vec![0x80, 0x81, 0x82],
        badges: 0b00000011,
        pokedex_owned: 15,
        play_time_hours: 5,
        play_time_minutes: 30,
        play_time_seconds: 0,
    };
    let mut menu = MainMenuState::new(Some(summary));
    assert_eq!(menu.item_count(), 3);
    assert!(menu.has_save);

    while !menu.init_delay_done {
        menu.update_frame(MenuInput::none());
    }

    let action = menu.update_frame(MenuInput {
        a: true,
        ..MenuInput::none()
    });
    assert_eq!(action, ScreenAction::Continue);
    assert!(menu.is_showing_continue_info());

    let action = menu.update_frame(MenuInput {
        a: true,
        ..MenuInput::none()
    });
    assert_eq!(action, ScreenAction::Transition(GameScreen::Overworld));
    assert_eq!(menu.last_choice, Some(MainMenuChoice::Continue));
}

#[test]
fn main_menu_b_returns_to_title() {
    let mut menu = MainMenuState::new(None);

    while !menu.init_delay_done {
        menu.update_frame(MenuInput::none());
    }

    let action = menu.update_frame(MenuInput {
        b: true,
        ..MenuInput::none()
    });
    assert_eq!(action, ScreenAction::Transition(GameScreen::TitleScreen));
    assert_eq!(menu.last_choice, Some(MainMenuChoice::BackToTitle));
}

#[test]
fn main_menu_navigate_to_option() {
    let mut menu = MainMenuState::new(None);

    while !menu.init_delay_done {
        menu.update_frame(MenuInput::none());
    }

    menu.update_frame(MenuInput {
        down: true,
        ..MenuInput::none()
    });

    let action = menu.update_frame(MenuInput {
        a: true,
        ..MenuInput::none()
    });
    assert_eq!(action, ScreenAction::Continue);
    assert_eq!(menu.last_choice, Some(MainMenuChoice::Option));
}

#[test]
fn full_opening_sequence_copyright_to_oak_speech() {
    let mut game = new_game_state_red();
    assert_eq!(game.screen, GameScreen::CopyrightSplash);

    game.transition_to(GameScreen::TitleScreen);
    assert_eq!(game.screen, GameScreen::TitleScreen);

    let mut title = TitleScreenState::new(game.config.version);
    while title.phase != TitlePhase::WaitingForInput {
        title.update_frame(false);
    }
    title.update_frame(true);
    while title.phase != TitlePhase::Done {
        title.update_frame(false);
    }

    game.transition_to(GameScreen::MainMenu);
    assert_eq!(game.screen, GameScreen::MainMenu);

    let mut menu = MainMenuState::new(None);
    while !menu.init_delay_done {
        menu.update_frame(MenuInput::none());
    }
    let action = menu.update_frame(MenuInput {
        a: true,
        ..MenuInput::none()
    });
    assert_eq!(action, ScreenAction::Transition(GameScreen::OakSpeech));

    game.transition_to(GameScreen::OakSpeech);
    assert_eq!(game.screen, GameScreen::OakSpeech);
}

#[test]
fn game_state_transitions_are_bidirectional() {
    let mut game = new_game_state_red();

    game.transition_to(GameScreen::TitleScreen);
    assert_eq!(game.screen, GameScreen::TitleScreen);

    game.transition_to(GameScreen::MainMenu);
    assert_eq!(game.screen, GameScreen::MainMenu);

    game.transition_to(GameScreen::TitleScreen);
    assert_eq!(game.screen, GameScreen::TitleScreen);

    game.transition_to(GameScreen::MainMenu);
    game.transition_to(GameScreen::OakSpeech);
    assert_eq!(game.screen, GameScreen::OakSpeech);

    game.transition_to(GameScreen::Overworld);
    assert_eq!(game.screen, GameScreen::Overworld);
}

#[test]
fn save_file_summary_badge_count() {
    let summary = SaveFileSummary {
        player_name: vec![],
        badges: 0b00000000,
        pokedex_owned: 0,
        play_time_hours: 0,
        play_time_minutes: 0,
        play_time_seconds: 0,
    };
    assert_eq!(summary.badge_count(), 0);

    let summary = SaveFileSummary {
        player_name: vec![],
        badges: ALL_BADGES,
        pokedex_owned: 151,
        play_time_hours: 99,
        play_time_minutes: 59,
        play_time_seconds: 59,
    };
    assert_eq!(summary.badge_count(), 8);
}

#[test]
fn title_screen_resets_correctly() {
    let mut title = TitleScreenState::new(GameVersion::Red);

    while title.phase != TitlePhase::WaitingForInput {
        title.update_frame(false);
    }

    title.reset();
    assert_eq!(title.phase, TitlePhase::Init);
    assert_eq!(title.frame_counter, 0);
    assert!(!title.logo_visible);
    assert!(!title.version_text_visible);
}
