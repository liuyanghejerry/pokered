use crate::naming_screen::{NamingInput, NamingScreenResult, NamingScreenState, NamingScreenType};

pub const DEFAULT_PLAYER_NAMES: [&str; 4] = ["RED", "ASH", "JACK", "NEW NAME"];
pub const DEFAULT_RIVAL_NAMES: [&str; 4] = ["BLUE", "GARY", "JOHN", "NEW NAME"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextPage {
    pub line1: &'static str,
    pub line2: &'static str,
}

impl TextPage {
    pub const fn new(line1: &'static str, line2: &'static str) -> Self {
        Self { line1, line2 }
    }

    pub const fn single(line: &'static str) -> Self {
        Self {
            line1: line,
            line2: "",
        }
    }
}

pub const OAK_SPEECH_TEXT1_PAGES: &[TextPage] = &[
    TextPage::new("Hello there!", "Welcome to the"),
    TextPage::new("world of #MON!", ""),
    TextPage::new("My name is OAK!", "People call me"),
    TextPage::new("the #MON PROF!", ""),
];

pub const OAK_SPEECH_TEXT2A_PAGES: &[TextPage] = &[
    TextPage::new("This world is", "inhabited by"),
    TextPage::new("creatures called", "#MON!"),
];

pub const OAK_SPEECH_TEXT2B_PAGES: &[TextPage] = &[
    TextPage::new("For some people,", "#MON are"),
    TextPage::new("pets. Others use", "them for fights."),
    TextPage::new("Myself...", ""),
    TextPage::new("I study #MON", "as a profession."),
];

pub const INTRODUCE_PLAYER_TEXT_PAGES: &[TextPage] =
    &[TextPage::new("First, what is", "your name?")];

pub const INTRODUCE_RIVAL_TEXT_PAGES: &[TextPage] = &[
    TextPage::new("This is my grand-", "son. He's been"),
    TextPage::new("your rival since", "you were a baby."),
    TextPage::new("...Erm, what is", "his name again?"),
];

pub const OAK_SPEECH_TEXT3_PAGES: &[TextPage] = &[
    TextPage::new("<PLAYER>!", ""),
    TextPage::new("Your very own", "#MON legend is"),
    TextPage::new("about to unfold!", ""),
    TextPage::new("A world of dreams", "and adventures"),
    TextPage::new("with #MON", "awaits! Let's go!"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OakSpeechPhase {
    Greeting {
        page_index: usize,
        waiting_for_input: bool,
    },
    ShowNidorino {
        page_index: usize,
        waiting_for_input: bool,
    },
    Explanation {
        page_index: usize,
        waiting_for_input: bool,
    },
    IntroducePlayer {
        page_index: usize,
        waiting_for_input: bool,
    },
    PlayerNameChoice {
        cursor: usize,
    },
    PlayerNaming,
    IntroduceRival {
        page_index: usize,
        waiting_for_input: bool,
    },
    RivalNameChoice {
        cursor: usize,
    },
    RivalNaming,
    FinalSpeech {
        page_index: usize,
        waiting_for_input: bool,
    },
    ShrinkPlayer {
        wait_frames: u16,
    },
    Done,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OakSpeechResult {
    Active,
    PlayerNameSet(String),
    RivalNameSet(String),
    Finished,
}

#[derive(Debug, Clone)]
pub struct OakSpeechState {
    pub phase: OakSpeechPhase,
    pub player_name: Option<String>,
    pub rival_name: Option<String>,
    pub naming_screen: Option<NamingScreenState>,
}

impl OakSpeechState {
    pub fn new() -> Self {
        Self {
            phase: OakSpeechPhase::Greeting {
                page_index: 0,
                waiting_for_input: false,
            },
            player_name: None,
            rival_name: None,
            naming_screen: None,
        }
    }

    pub fn update_frame(&mut self, input: OakSpeechInput) -> OakSpeechResult {
        let new_phase = match &self.phase {
            OakSpeechPhase::Greeting {
                page_index,
                waiting_for_input,
            } => Self::advance_text_phase(
                input,
                *page_index,
                *waiting_for_input,
                OAK_SPEECH_TEXT1_PAGES.len(),
            )
            .map(|_| OakSpeechPhase::ShowNidorino {
                page_index: 0,
                waiting_for_input: false,
            }),
            OakSpeechPhase::ShowNidorino {
                page_index,
                waiting_for_input,
            } => Self::advance_text_phase(
                input,
                *page_index,
                *waiting_for_input,
                OAK_SPEECH_TEXT2A_PAGES.len(),
            )
            .map(|_| OakSpeechPhase::Explanation {
                page_index: 0,
                waiting_for_input: false,
            }),
            OakSpeechPhase::Explanation {
                page_index,
                waiting_for_input,
            } => Self::advance_text_phase(
                input,
                *page_index,
                *waiting_for_input,
                OAK_SPEECH_TEXT2B_PAGES.len(),
            )
            .map(|_| OakSpeechPhase::IntroducePlayer {
                page_index: 0,
                waiting_for_input: false,
            }),
            OakSpeechPhase::IntroducePlayer {
                page_index,
                waiting_for_input,
            } => Self::advance_text_phase(
                input,
                *page_index,
                *waiting_for_input,
                INTRODUCE_PLAYER_TEXT_PAGES.len(),
            )
            .map(|_| OakSpeechPhase::PlayerNameChoice { cursor: 0 }),
            OakSpeechPhase::IntroduceRival {
                page_index,
                waiting_for_input,
            } => Self::advance_text_phase(
                input,
                *page_index,
                *waiting_for_input,
                INTRODUCE_RIVAL_TEXT_PAGES.len(),
            )
            .map(|_| OakSpeechPhase::RivalNameChoice { cursor: 0 }),
            OakSpeechPhase::FinalSpeech {
                page_index,
                waiting_for_input,
            } => Self::advance_text_phase(
                input,
                *page_index,
                *waiting_for_input,
                OAK_SPEECH_TEXT3_PAGES.len(),
            )
            .map(|_| OakSpeechPhase::ShrinkPlayer { wait_frames: 60 }),
            _ => None,
        };

        if let Some(phase) = new_phase {
            self.phase = phase;
            return OakSpeechResult::Active;
        }

        match &mut self.phase {
            OakSpeechPhase::PlayerNameChoice { cursor } => {
                if input.up && *cursor > 0 {
                    *cursor -= 1;
                } else if input.down && *cursor < DEFAULT_PLAYER_NAMES.len() - 1 {
                    *cursor += 1;
                }
                if input.a {
                    let choice = *cursor;
                    if choice == DEFAULT_PLAYER_NAMES.len() - 1 {
                        self.naming_screen = Some(NamingScreenState::new(NamingScreenType::Player));
                        self.phase = OakSpeechPhase::PlayerNaming;
                    } else {
                        let name = DEFAULT_PLAYER_NAMES[choice].to_string();
                        self.player_name = Some(name.clone());
                        self.phase = OakSpeechPhase::IntroduceRival {
                            page_index: 0,
                            waiting_for_input: false,
                        };
                        return OakSpeechResult::PlayerNameSet(name);
                    }
                }
                OakSpeechResult::Active
            }
            OakSpeechPhase::PlayerNaming => OakSpeechResult::Active,
            OakSpeechPhase::RivalNameChoice { cursor } => {
                if input.up && *cursor > 0 {
                    *cursor -= 1;
                } else if input.down && *cursor < DEFAULT_RIVAL_NAMES.len() - 1 {
                    *cursor += 1;
                }
                if input.a {
                    let choice = *cursor;
                    if choice == DEFAULT_RIVAL_NAMES.len() - 1 {
                        self.naming_screen = Some(NamingScreenState::new(NamingScreenType::Rival));
                        self.phase = OakSpeechPhase::RivalNaming;
                    } else {
                        let name = DEFAULT_RIVAL_NAMES[choice].to_string();
                        self.rival_name = Some(name.clone());
                        self.phase = OakSpeechPhase::FinalSpeech {
                            page_index: 0,
                            waiting_for_input: false,
                        };
                        return OakSpeechResult::RivalNameSet(name);
                    }
                }
                OakSpeechResult::Active
            }
            OakSpeechPhase::RivalNaming => OakSpeechResult::Active,
            OakSpeechPhase::ShrinkPlayer { wait_frames } => {
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    OakSpeechResult::Active
                } else {
                    self.phase = OakSpeechPhase::Done;
                    OakSpeechResult::Finished
                }
            }
            OakSpeechPhase::Done => OakSpeechResult::Finished,
            _ => OakSpeechResult::Active,
        }
    }

    fn advance_text_phase(
        input: OakSpeechInput,
        page_index: usize,
        waiting_for_input: bool,
        total_pages: usize,
    ) -> Option<()> {
        if waiting_for_input {
            if input.a || input.b {
                if page_index + 1 >= total_pages {
                    return Some(());
                }
            }
        }
        None
    }

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
            NamingScreenResult::Cancelled => match self.phase {
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
            },
        }
    }

    pub fn is_naming_active(&self) -> bool {
        matches!(
            self.phase,
            OakSpeechPhase::PlayerNaming | OakSpeechPhase::RivalNaming
        )
    }

    pub fn current_text_page(&self) -> Option<TextPage> {
        match &self.phase {
            OakSpeechPhase::Greeting { page_index, .. } => {
                OAK_SPEECH_TEXT1_PAGES.get(*page_index).cloned()
            }
            OakSpeechPhase::ShowNidorino { page_index, .. } => {
                OAK_SPEECH_TEXT2A_PAGES.get(*page_index).cloned()
            }
            OakSpeechPhase::Explanation { page_index, .. } => {
                OAK_SPEECH_TEXT2B_PAGES.get(*page_index).cloned()
            }
            OakSpeechPhase::IntroducePlayer { page_index, .. } => {
                INTRODUCE_PLAYER_TEXT_PAGES.get(*page_index).cloned()
            }
            OakSpeechPhase::IntroduceRival { page_index, .. } => {
                INTRODUCE_RIVAL_TEXT_PAGES.get(*page_index).cloned()
            }
            OakSpeechPhase::FinalSpeech { page_index, .. } => {
                OAK_SPEECH_TEXT3_PAGES.get(*page_index).cloned()
            }
            _ => None,
        }
    }

    pub fn is_waiting_for_input(&self) -> bool {
        match &self.phase {
            OakSpeechPhase::Greeting {
                waiting_for_input, ..
            } => *waiting_for_input,
            OakSpeechPhase::ShowNidorino {
                waiting_for_input, ..
            } => *waiting_for_input,
            OakSpeechPhase::Explanation {
                waiting_for_input, ..
            } => *waiting_for_input,
            OakSpeechPhase::IntroducePlayer {
                waiting_for_input, ..
            } => *waiting_for_input,
            OakSpeechPhase::IntroduceRival {
                waiting_for_input, ..
            } => *waiting_for_input,
            OakSpeechPhase::FinalSpeech {
                waiting_for_input, ..
            } => *waiting_for_input,
            _ => false,
        }
    }

    pub fn get_display_text(&self, text: &str) -> String {
        if let Some(ref name) = self.player_name {
            text.replace("<PLAYER>", name)
        } else {
            text.replace("<PLAYER>", "RED")
        }
    }
}

impl Default for OakSpeechState {
    fn default() -> Self {
        Self::new()
    }
}
