//! Oak's intro speech state machine.
//!
//! Replicates `engine/movie/oak_speech/oak_speech.asm`:
//! 1. Oak appears, gives welcome text
//! 2. Shows a Nidorino (world introduction)
//! 3. Player naming screen (or default names)
//! 4. Rival naming screen (or default names)
//! 5. Shrink player sprite, transition to overworld
//!
//! The naming sub-screens are handled inline — when the phase is
//! `PlayerNaming` or `RivalNaming`, the owning game loop should
//! delegate input to the embedded `NamingScreenState`.

use crate::naming_screen::{NamingInput, NamingScreenResult, NamingScreenState, NamingScreenType};

pub const DEFAULT_PLAYER_NAMES: [&str; 4] = ["RED", "ASH", "JACK", "NEW NAME"];
pub const DEFAULT_RIVAL_NAMES: [&str; 4] = ["BLUE", "GARY", "JOHN", "NEW NAME"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OakSpeechPhase {
    Intro { text_index: usize, wait_frames: u16 },
    ShowNidorino { wait_frames: u16 },
    PlayerNameChoice { cursor: usize },
    PlayerNaming,
    RivalNameChoice { cursor: usize },
    RivalNaming,
    ShrinkPlayer { wait_frames: u16 },
    Done,
}

/// Input for non-naming phases of Oak's speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OakSpeechInput {
    pub up: bool,
    pub down: bool,
    pub a: bool,
    pub b: bool,
}

impl OakSpeechInput {
    pub fn none() -> Self {
        Self {
            up: false,
            down: false,
            a: false,
            b: false,
        }
    }
}

/// Result of an Oak speech frame update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OakSpeechResult {
    /// Still in Oak speech sequence.
    Active,
    /// Player name was chosen/entered.
    PlayerNameSet(String),
    /// Rival name was chosen/entered.
    RivalNameSet(String),
    /// Speech is complete — transition to overworld.
    Finished,
}

/// The Oak speech state machine.
#[derive(Debug, Clone)]
pub struct OakSpeechState {
    pub phase: OakSpeechPhase,
    /// Player's chosen name (set during PlayerNaming phase).
    pub player_name: Option<String>,
    /// Rival's chosen name (set during RivalNaming phase).
    pub rival_name: Option<String>,
    /// Embedded naming screen (active during PlayerNaming / RivalNaming phases).
    pub naming_screen: Option<NamingScreenState>,
}

/// Intro text lines (simplified from the original multi-page text).
pub const INTRO_TEXTS: [&str; 5] = [
    "Hello there!",
    "Welcome to the",
    "world of POKeMON!",
    "My name is OAK!",
    "People call me",
];

impl OakSpeechState {
    pub fn new() -> Self {
        Self {
            phase: OakSpeechPhase::Intro {
                text_index: 0,
                wait_frames: 120, // ~2 seconds per text block at 60fps
            },
            player_name: None,
            rival_name: None,
            naming_screen: None,
        }
    }

    /// Process one frame of the Oak speech sequence (non-naming phases).
    /// When in PlayerNaming/RivalNaming phase, the caller should send input
    /// to `naming_screen` directly via `update_naming_frame()`.
    pub fn update_frame(&mut self, input: OakSpeechInput) -> OakSpeechResult {
        match &mut self.phase {
            OakSpeechPhase::Intro {
                text_index,
                wait_frames,
            } => {
                if input.a || input.b {
                    // Skip to next text or advance phase
                    *wait_frames = 0;
                }
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return OakSpeechResult::Active;
                }
                // Advance to next text
                *text_index += 1;
                if *text_index >= INTRO_TEXTS.len() {
                    self.phase = OakSpeechPhase::ShowNidorino { wait_frames: 90 };
                } else {
                    *wait_frames = 120;
                }
                OakSpeechResult::Active
            }
            OakSpeechPhase::ShowNidorino { wait_frames } => {
                if input.a || input.b {
                    *wait_frames = 0;
                }
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return OakSpeechResult::Active;
                }
                self.phase = OakSpeechPhase::PlayerNameChoice { cursor: 0 };
                OakSpeechResult::Active
            }
            OakSpeechPhase::PlayerNameChoice { cursor } => {
                if input.up && *cursor > 0 {
                    *cursor -= 1;
                } else if input.down && *cursor < DEFAULT_PLAYER_NAMES.len() - 1 {
                    *cursor += 1;
                }
                if input.a {
                    let choice = *cursor;
                    if choice == DEFAULT_PLAYER_NAMES.len() - 1 {
                        // "NEW NAME" — open custom naming screen
                        self.naming_screen = Some(NamingScreenState::new(NamingScreenType::Player));
                        self.phase = OakSpeechPhase::PlayerNaming;
                    } else {
                        let name = DEFAULT_PLAYER_NAMES[choice].to_string();
                        self.player_name = Some(name.clone());
                        self.phase = OakSpeechPhase::RivalNameChoice { cursor: 0 };
                        return OakSpeechResult::PlayerNameSet(name);
                    }
                }
                OakSpeechResult::Active
            }
            OakSpeechPhase::PlayerNaming => {
                // Input is handled via update_naming_frame(); this branch
                // should not be reached if the caller uses it correctly.
                OakSpeechResult::Active
            }
            OakSpeechPhase::RivalNameChoice { cursor } => {
                if input.up && *cursor > 0 {
                    *cursor -= 1;
                } else if input.down && *cursor < DEFAULT_RIVAL_NAMES.len() - 1 {
                    *cursor += 1;
                }
                if input.a {
                    let choice = *cursor;
                    if choice == DEFAULT_RIVAL_NAMES.len() - 1 {
                        // "NEW NAME" — open custom naming screen
                        self.naming_screen = Some(NamingScreenState::new(NamingScreenType::Rival));
                        self.phase = OakSpeechPhase::RivalNaming;
                    } else {
                        let name = DEFAULT_RIVAL_NAMES[choice].to_string();
                        self.rival_name = Some(name.clone());
                        self.phase = OakSpeechPhase::ShrinkPlayer { wait_frames: 60 };
                        return OakSpeechResult::RivalNameSet(name);
                    }
                }
                OakSpeechResult::Active
            }
            OakSpeechPhase::RivalNaming => {
                // Input is handled via update_naming_frame()
                OakSpeechResult::Active
            }
            OakSpeechPhase::ShrinkPlayer { wait_frames } => {
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return OakSpeechResult::Active;
                }
                self.phase = OakSpeechPhase::Done;
                OakSpeechResult::Finished
            }
            OakSpeechPhase::Done => OakSpeechResult::Finished,
        }
    }

    /// Update the embedded naming screen. Call this when phase is
    /// `PlayerNaming` or `RivalNaming`.
    pub fn update_naming_frame(&mut self, input: NamingInput) -> OakSpeechResult {
        let Some(naming) = &mut self.naming_screen else {
            return OakSpeechResult::Active;
        };

        match naming.update_frame(input) {
            NamingScreenResult::Editing => OakSpeechResult::Active,
            NamingScreenResult::Submitted(name) => match self.phase {
                OakSpeechPhase::PlayerNaming => {
                    self.player_name = Some(name.clone());
                    self.naming_screen = None;
                    self.phase = OakSpeechPhase::RivalNameChoice { cursor: 0 };
                    OakSpeechResult::PlayerNameSet(name)
                }
                OakSpeechPhase::RivalNaming => {
                    self.rival_name = Some(name.clone());
                    self.naming_screen = None;
                    self.phase = OakSpeechPhase::ShrinkPlayer { wait_frames: 60 };
                    OakSpeechResult::RivalNameSet(name)
                }
                _ => OakSpeechResult::Active,
            },
            NamingScreenResult::Cancelled => {
                // Cancelled = use default name
                match self.phase {
                    OakSpeechPhase::PlayerNaming => {
                        let name = DEFAULT_PLAYER_NAMES[0].to_string();
                        self.player_name = Some(name.clone());
                        self.naming_screen = None;
                        self.phase = OakSpeechPhase::RivalNameChoice { cursor: 0 };
                        OakSpeechResult::PlayerNameSet(name)
                    }
                    OakSpeechPhase::RivalNaming => {
                        let name = DEFAULT_RIVAL_NAMES[0].to_string();
                        self.rival_name = Some(name.clone());
                        self.naming_screen = None;
                        self.phase = OakSpeechPhase::ShrinkPlayer { wait_frames: 60 };
                        OakSpeechResult::RivalNameSet(name)
                    }
                    _ => OakSpeechResult::Active,
                }
            }
        }
    }

    /// Check if we're in a naming sub-screen phase.
    pub fn is_naming_active(&self) -> bool {
        matches!(
            self.phase,
            OakSpeechPhase::PlayerNaming | OakSpeechPhase::RivalNaming
        )
    }

    /// Get current intro text to display (if in Intro phase).
    pub fn current_intro_text(&self) -> Option<&'static str> {
        match &self.phase {
            OakSpeechPhase::Intro { text_index, .. } => INTRO_TEXTS.get(*text_index).copied(),
            _ => None,
        }
    }
}

impl Default for OakSpeechState {
    fn default() -> Self {
        Self::new()
    }
}
