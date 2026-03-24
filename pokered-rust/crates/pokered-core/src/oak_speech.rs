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
    /// Initial greeting (OakSpeechText1)
    Greeting {
        wait_frames: u16,
    },
    /// Show Nidorino sprite (OakSpeechText2A)
    ShowNidorino {
        wait_frames: u16,
    },
    /// Continue explanation after Nidorino (OakSpeechText2B)
    Explanation {
        wait_frames: u16,
    },
    /// Ask for player's name
    IntroducePlayer {
        wait_frames: u16,
    },
    /// Player name selection
    PlayerNameChoice {
        cursor: usize,
    },
    PlayerNaming,
    /// Introduce rival
    IntroduceRival {
        wait_frames: u16,
    },
    /// Rival name selection
    RivalNameChoice {
        cursor: usize,
    },
    RivalNaming,
    /// Final motivational speech (OakSpeechText3)
    FinalSpeech {
        wait_frames: u16,
    },
    ShrinkPlayer {
        wait_frames: u16,
    },
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

/// Complete Oak speech text segments from the original game.
pub const OAK_SPEECH_TEXT1: &str = "Hello there!\n\
     Welcome to the\n\
     world of POKéMON!\n\n\
     My name is OAK!\n\
     People call me\n\
     the POKéMON PROF!";

pub const OAK_SPEECH_TEXT2A: &str = "This world is\n\
     inhabited by\n\
     creatures called\n\
     POKéMON!";

pub const OAK_SPEECH_TEXT2B: &str = "For some people,\n\
     POKéMON are\n\
     pets. Others use\n\
     them for fights.\n\n\
     Myself...\n\n\
     I study POKéMON\n\
     as a profession.";

pub const INTRODUCE_PLAYER_TEXT: &str = "First, what is\n\
     your name?";

pub const INTRODUCE_RIVAL_TEXT: &str = "This is my grand-\n\
     son. He's been\n\
     your rival since\n\
     you were a baby.\n\n\
     ...Erm, what is\n\
     his name again?";

pub const OAK_SPEECH_TEXT3: &str = "<PLAYER>!\n\n\
     Your very own\n\
     POKéMON legend is\n\
     about to unfold!\n\n\
     A world of dreams\n\
     and adventures\n\
     with POKéMON\n\
     awaits! Let's go!";

impl OakSpeechState {
    pub fn new() -> Self {
        Self {
            phase: OakSpeechPhase::Greeting { wait_frames: 180 }, // ~3 seconds at 60fps
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
            OakSpeechPhase::Greeting { wait_frames } => {
                if input.a || input.b {
                    *wait_frames = 0;
                }
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return OakSpeechResult::Active;
                }
                self.phase = OakSpeechPhase::ShowNidorino { wait_frames: 120 };
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
                self.phase = OakSpeechPhase::Explanation { wait_frames: 180 };
                OakSpeechResult::Active
            }
            OakSpeechPhase::Explanation { wait_frames } => {
                if input.a || input.b {
                    *wait_frames = 0;
                }
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return OakSpeechResult::Active;
                }
                self.phase = OakSpeechPhase::IntroducePlayer { wait_frames: 120 };
                OakSpeechResult::Active
            }
            OakSpeechPhase::IntroducePlayer { wait_frames } => {
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
                        self.phase = OakSpeechPhase::IntroduceRival { wait_frames: 120 };
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
            OakSpeechPhase::IntroduceRival { wait_frames } => {
                if input.a || input.b {
                    *wait_frames = 0;
                }
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return OakSpeechResult::Active;
                }
                self.phase = OakSpeechPhase::RivalNameChoice { cursor: 0 };
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
                        self.phase = OakSpeechPhase::FinalSpeech { wait_frames: 180 };
                        return OakSpeechResult::RivalNameSet(name);
                    }
                }
                OakSpeechResult::Active
            }
            OakSpeechPhase::RivalNaming => {
                // Input is handled via update_naming_frame()
                OakSpeechResult::Active
            }
            OakSpeechPhase::FinalSpeech { wait_frames } => {
                if input.a || input.b {
                    *wait_frames = 0;
                }
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return OakSpeechResult::Active;
                }
                self.phase = OakSpeechPhase::ShrinkPlayer { wait_frames: 60 };
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
            OakSpeechPhase::Greeting { .. } => Some(OAK_SPEECH_TEXT1),
            OakSpeechPhase::ShowNidorino { .. } => Some(OAK_SPEECH_TEXT2A),
            OakSpeechPhase::Explanation { .. } => Some(OAK_SPEECH_TEXT2B),
            OakSpeechPhase::IntroducePlayer { .. } => Some(INTRODUCE_PLAYER_TEXT),
            OakSpeechPhase::IntroduceRival { .. } => Some(INTRODUCE_RIVAL_TEXT),
            OakSpeechPhase::FinalSpeech { .. } => Some(OAK_SPEECH_TEXT3),
            _ => None,
        }
    }
}

impl Default for OakSpeechState {
    fn default() -> Self {
        Self::new()
    }
}
