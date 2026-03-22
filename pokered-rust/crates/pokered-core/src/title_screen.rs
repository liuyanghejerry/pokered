//! Title screen state machine.
//!
//! Replicates the title screen from `engine/movie/title.asm`:
//! 1. Copyright splash (brief)
//! 2. Pokémon logo loads, bounces down into place
//! 3. Game version text scrolls in from the right
//! 4. Title music starts
//! 5. Pokémon sprites cycle on screen while waiting for user input
//! 6. On any button press → play the current mon's cry → transition to MainMenu
//!
//! This module handles the logical state (animation phase, timers, current mon).
//! Rendering is handled by the renderer crate based on the state exposed here.

use pokered_data::species::Species;
use pokered_data::wild_data::GameVersion;

use crate::game_state::{GameScreen, ScreenAction};

/// Pokémon shown on the Red version title screen.
pub const TITLE_MONS_RED: [Species; 16] = [
    Species::Charmander,  // STARTER1
    Species::Squirtle,    // STARTER2
    Species::Bulbasaur,   // STARTER3
    Species::Weedle,
    Species::NidoranM,
    Species::Scyther,
    Species::Pikachu,
    Species::Clefairy,
    Species::Rhydon,
    Species::Abra,
    Species::Gastly,
    Species::Ditto,
    Species::Pidgeotto,
    Species::Onix,
    Species::Ponyta,
    Species::Magikarp,
];

/// Pokémon shown on the Blue version title screen.
pub const TITLE_MONS_BLUE: [Species; 16] = [
    Species::Squirtle,    // STARTER2
    Species::Charmander,  // STARTER1
    Species::Bulbasaur,   // STARTER3
    Species::Mankey,
    Species::Hitmonlee,
    Species::Vulpix,
    Species::Chansey,
    Species::Aerodactyl,
    Species::Jolteon,
    Species::Snorlax,
    Species::Gloom,
    Species::Poliwag,
    Species::Doduo,
    Species::Porygon,
    Species::Gengar,
    Species::Raichu,
];

/// Animation phases of the title screen, in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitlePhase {
    /// Load graphics, prepare screen (instantaneous in our Rust version).
    Init,
    /// Pokémon logo bounces into position.
    LogoBounce,
    /// Brief pause after logo lands.
    LogoPause,
    /// Game version text scrolls in from the right.
    VersionScroll,
    /// Title music plays, Pokémon sprites cycle. Waiting for user input.
    WaitingForInput,
    /// User pressed a button — playing the current mon's cry.
    PlayingCry,
    /// Fade to white, then transition to main menu.
    FadeOut,
    /// Done — ready to transition.
    Done,
}

/// Logo bounce data: (scroll_delta, repeat_count) pairs.
/// Matches .TitleScreenPokemonLogoYScrolls from title.asm
pub const LOGO_BOUNCE_TABLE: &[(i8, u8)] = &[
    (-4, 16),
    (3, 4),
    (-3, 4),
    (2, 2),
    (-2, 2),
    (1, 2),
    (-1, 2),
];

/// Total frames the logo bounce takes.
pub const LOGO_BOUNCE_TOTAL_FRAMES: u32 = {
    let mut total = 0u32;
    let mut i = 0;
    while i < 7 {
        total += LOGO_BOUNCE_TABLE[i].1 as u32;
        i += 1;
    }
    total
};

/// Frames to pause after logo lands (matches: ld c, 36 / call DelayFrames).
pub const LOGO_PAUSE_FRAMES: u32 = 36;

/// Frames for the version text scroll animation.
pub const VERSION_SCROLL_FRAMES: u32 = 36;

/// Frames between Pokémon switches on the title screen.
/// In the original: `ld c, 200 / call CheckForUserInterruption`.
pub const MON_DISPLAY_FRAMES: u32 = 200;

/// Frames for playing a cry before transitioning.
pub const CRY_PLAY_FRAMES: u32 = 60;

/// Frames for the fade-out effect.
pub const FADE_OUT_FRAMES: u32 = 16;

/// Title screen state.
#[derive(Debug, Clone)]
pub struct TitleScreenState {
    /// Current animation phase.
    pub phase: TitlePhase,
    /// Frame counter within the current phase.
    pub frame_counter: u32,
    /// Current background scroll Y (for logo bounce animation).
    pub scroll_y: i32,
    /// Which Pokémon is currently displayed.
    pub current_mon: Species,
    /// Index into the title mons list for the current mon.
    pub current_mon_index: usize,
    /// Game version (affects mon list and version text).
    pub version: GameVersion,
    /// Whether the player character sprite is visible.
    pub player_visible: bool,
    /// Whether the Pokémon logo is visible.
    pub logo_visible: bool,
    /// Whether the version text is visible.
    pub version_text_visible: bool,
    /// Progress of the version text scroll (0.0 = offscreen, 1.0 = in place).
    pub version_scroll_progress: f32,
    /// Current logo bounce step index.
    bounce_step: usize,
    /// Frames remaining in the current bounce step.
    bounce_frames_remaining: u8,
    /// Simple RNG state for picking random mons.
    rng_state: u32,
}

impl TitleScreenState {
    pub fn new(version: GameVersion) -> Self {
        let first_mon = match version {
            GameVersion::Red => TITLE_MONS_RED[0],
            GameVersion::Blue => TITLE_MONS_BLUE[0],
        };
        Self {
            phase: TitlePhase::Init,
            frame_counter: 0,
            scroll_y: 64, // initial hSCY = $40
            current_mon: first_mon,
            current_mon_index: 0,
            version,
            player_visible: false,
            logo_visible: false,
            version_text_visible: false,
            version_scroll_progress: 0.0,
            bounce_step: 0,
            bounce_frames_remaining: 0,
            rng_state: 0x12345678,
        }
    }

    /// Seed the RNG (for deterministic testing).
    pub fn set_rng_seed(&mut self, seed: u32) {
        self.rng_state = seed;
    }

    /// Get the title mons list for the current version.
    pub fn title_mons(&self) -> &[Species; 16] {
        match self.version {
            GameVersion::Red => &TITLE_MONS_RED,
            GameVersion::Blue => &TITLE_MONS_BLUE,
        }
    }

    /// Simple xorshift RNG.
    fn next_random(&mut self) -> u32 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng_state = x;
        x
    }

    /// Pick a random mon different from the current one (matching original logic).
    fn pick_new_mon(&mut self) -> Species {
        let mons = match self.version {
            GameVersion::Red => &TITLE_MONS_RED,
            GameVersion::Blue => &TITLE_MONS_BLUE,
        };
        loop {
            let index = (self.next_random() & 0xF) as usize;
            let candidate = mons[index];
            if candidate != self.current_mon {
                self.current_mon_index = index;
                return candidate;
            }
        }
    }

    /// Update one frame. Returns what action to take.
    ///
    /// `any_button_pressed`: true if the player pressed any button this frame.
    pub fn update_frame(&mut self, any_button_pressed: bool) -> ScreenAction {
        match self.phase {
            TitlePhase::Init => {
                self.logo_visible = true;
                self.player_visible = true;
                self.phase = TitlePhase::LogoBounce;
                self.frame_counter = 0;
                self.bounce_step = 0;
                if !LOGO_BOUNCE_TABLE.is_empty() {
                    self.bounce_frames_remaining = LOGO_BOUNCE_TABLE[0].1;
                }
                ScreenAction::Continue
            }

            TitlePhase::LogoBounce => {
                if self.bounce_step < LOGO_BOUNCE_TABLE.len() {
                    let (delta, _count) = LOGO_BOUNCE_TABLE[self.bounce_step];
                    self.scroll_y += delta as i32;
                    self.bounce_frames_remaining -= 1;
                    if self.bounce_frames_remaining == 0 {
                        self.bounce_step += 1;
                        if self.bounce_step < LOGO_BOUNCE_TABLE.len() {
                            self.bounce_frames_remaining = LOGO_BOUNCE_TABLE[self.bounce_step].1;
                        }
                    }
                    self.frame_counter += 1;
                }

                if self.bounce_step >= LOGO_BOUNCE_TABLE.len() {
                    self.phase = TitlePhase::LogoPause;
                    self.frame_counter = 0;
                }
                ScreenAction::Continue
            }

            TitlePhase::LogoPause => {
                self.frame_counter += 1;
                if self.frame_counter >= LOGO_PAUSE_FRAMES {
                    self.phase = TitlePhase::VersionScroll;
                    self.frame_counter = 0;
                    self.version_text_visible = true;
                }
                ScreenAction::Continue
            }

            TitlePhase::VersionScroll => {
                self.frame_counter += 1;
                self.version_scroll_progress =
                    (self.frame_counter as f32) / (VERSION_SCROLL_FRAMES as f32);
                if self.version_scroll_progress > 1.0 {
                    self.version_scroll_progress = 1.0;
                }
                if self.frame_counter >= VERSION_SCROLL_FRAMES {
                    self.phase = TitlePhase::WaitingForInput;
                    self.frame_counter = 0;
                }
                ScreenAction::Continue
            }

            TitlePhase::WaitingForInput => {
                if any_button_pressed {
                    self.phase = TitlePhase::PlayingCry;
                    self.frame_counter = 0;
                    return ScreenAction::Continue;
                }

                self.frame_counter += 1;
                if self.frame_counter >= MON_DISPLAY_FRAMES {
                    // Switch to a new random Pokémon
                    self.current_mon = self.pick_new_mon();
                    self.frame_counter = 0;
                }
                ScreenAction::Continue
            }

            TitlePhase::PlayingCry => {
                self.frame_counter += 1;
                if self.frame_counter >= CRY_PLAY_FRAMES {
                    self.phase = TitlePhase::FadeOut;
                    self.frame_counter = 0;
                }
                ScreenAction::Continue
            }

            TitlePhase::FadeOut => {
                self.frame_counter += 1;
                if self.frame_counter >= FADE_OUT_FRAMES {
                    self.phase = TitlePhase::Done;
                    return ScreenAction::Transition(GameScreen::MainMenu);
                }
                ScreenAction::Continue
            }

            TitlePhase::Done => ScreenAction::Transition(GameScreen::MainMenu),
        }
    }

    /// Reset to initial state (for returning from main menu).
    pub fn reset(&mut self) {
        let first_mon = match self.version {
            GameVersion::Red => TITLE_MONS_RED[0],
            GameVersion::Blue => TITLE_MONS_BLUE[0],
        };
        self.phase = TitlePhase::Init;
        self.frame_counter = 0;
        self.scroll_y = 64;
        self.current_mon = first_mon;
        self.current_mon_index = 0;
        self.player_visible = false;
        self.logo_visible = false;
        self.version_text_visible = false;
        self.version_scroll_progress = 0.0;
        self.bounce_step = 0;
        self.bounce_frames_remaining = 0;
    }

    /// Get fade progress (0.0 = no fade, 1.0 = fully white).
    pub fn fade_progress(&self) -> f32 {
        if self.phase == TitlePhase::FadeOut {
            (self.frame_counter as f32) / (FADE_OUT_FRAMES as f32)
        } else {
            0.0
        }
    }
}
