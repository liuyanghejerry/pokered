use crate::naming_screen::{NamingInput, NamingScreenResult, NamingScreenState, NamingScreenType};

pub const DEFAULT_PLAYER_NAMES: [&str; 4] = ["RED", "ASH", "JACK", "NEW NAME"];
pub const DEFAULT_RIVAL_NAMES: [&str; 4] = ["BLUE", "GARY", "JOHN", "NEW NAME"];

const TYPEWRITER_CHARS_PER_FRAME: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextPage {
    pub line1: &'static str,
    pub line2: &'static str,
}

impl TextPage {
    pub const fn new(line1: &'static str, line2: &'static str) -> Self {
        Self { line1, line2 }
    }

    pub fn total_chars(&self, player_name: Option<&str>) -> usize {
        let l1 = Self::replace_player(self.line1, player_name);
        let l2 = Self::replace_player(self.line2, player_name);
        l1.len() + l2.len()
    }

    fn replace_player(text: &str, player_name: Option<&str>) -> String {
        text.replace("<PLAYER>", player_name.unwrap_or("RED"))
    }

    pub fn get_display_text(&self, player_name: Option<&str>, char_index: u16) -> (String, String) {
        let l1 = Self::replace_player(self.line1, player_name);
        let l2 = Self::replace_player(self.line2, player_name);

        let total = l1.len() + l2.len();
        let idx = char_index as usize;

        if idx >= total {
            (l1, l2)
        } else if idx <= l1.len() {
            (l1[..idx].to_string(), String::new())
        } else {
            let l2_idx = idx - l1.len();
            (l1.clone(), l2[..l2_idx].to_string())
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
        char_index: u16,
        waiting_for_input: bool,
    },
    ShowNidorino {
        page_index: usize,
        char_index: u16,
        waiting_for_input: bool,
    },
    Explanation {
        page_index: usize,
        char_index: u16,
        waiting_for_input: bool,
    },
    IntroducePlayer {
        page_index: usize,
        char_index: u16,
        waiting_for_input: bool,
    },
    PlayerNameChoice {
        cursor: usize,
    },
    PlayerNaming,
    IntroduceRival {
        page_index: usize,
        char_index: u16,
        waiting_for_input: bool,
    },
    RivalNameChoice {
        cursor: usize,
    },
    RivalNaming,
    FinalSpeech {
        page_index: usize,
        char_index: u16,
        waiting_for_input: bool,
    },
    ShrinkPlayer {
        wait_frames: u16,
    },
    Done,
}

impl OakSpeechPhase {
    fn text_pages(&self) -> Option<&'static [TextPage]> {
        match self {
            OakSpeechPhase::Greeting { .. } => Some(OAK_SPEECH_TEXT1_PAGES),
            OakSpeechPhase::ShowNidorino { .. } => Some(OAK_SPEECH_TEXT2A_PAGES),
            OakSpeechPhase::Explanation { .. } => Some(OAK_SPEECH_TEXT2B_PAGES),
            OakSpeechPhase::IntroducePlayer { .. } => Some(INTRODUCE_PLAYER_TEXT_PAGES),
            OakSpeechPhase::IntroduceRival { .. } => Some(INTRODUCE_RIVAL_TEXT_PAGES),
            OakSpeechPhase::FinalSpeech { .. } => Some(OAK_SPEECH_TEXT3_PAGES),
            _ => None,
        }
    }
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
                char_index: 0,
                waiting_for_input: false,
            },
            player_name: None,
            rival_name: None,
            naming_screen: None,
        }
    }

    pub fn update_frame(&mut self, input: OakSpeechInput) -> OakSpeechResult {
        if let Some(new_phase) = self.process_phase(input) {
            self.phase = new_phase;
        }

        if self.phase == OakSpeechPhase::Done {
            OakSpeechResult::Finished
        } else {
            OakSpeechResult::Active
        }
    }

    fn process_phase(&mut self, input: OakSpeechInput) -> Option<OakSpeechPhase> {
        let new_phase = match &self.phase {
            OakSpeechPhase::Greeting {
                page_index,
                char_index,
                waiting_for_input,
            } => self.process_text_phase_inline(
                input,
                *page_index,
                *char_index,
                *waiting_for_input,
                OAK_SPEECH_TEXT1_PAGES,
                |pi, ci, wfi| OakSpeechPhase::Greeting {
                    page_index: pi,
                    char_index: ci,
                    waiting_for_input: wfi,
                },
                || OakSpeechPhase::ShowNidorino {
                    page_index: 0,
                    char_index: 0,
                    waiting_for_input: false,
                },
            ),
            OakSpeechPhase::ShowNidorino {
                page_index,
                char_index,
                waiting_for_input,
            } => self.process_text_phase_inline(
                input,
                *page_index,
                *char_index,
                *waiting_for_input,
                OAK_SPEECH_TEXT2A_PAGES,
                |pi, ci, wfi| OakSpeechPhase::ShowNidorino {
                    page_index: pi,
                    char_index: ci,
                    waiting_for_input: wfi,
                },
                || OakSpeechPhase::Explanation {
                    page_index: 0,
                    char_index: 0,
                    waiting_for_input: false,
                },
            ),
            OakSpeechPhase::Explanation {
                page_index,
                char_index,
                waiting_for_input,
            } => self.process_text_phase_inline(
                input,
                *page_index,
                *char_index,
                *waiting_for_input,
                OAK_SPEECH_TEXT2B_PAGES,
                |pi, ci, wfi| OakSpeechPhase::Explanation {
                    page_index: pi,
                    char_index: ci,
                    waiting_for_input: wfi,
                },
                || OakSpeechPhase::IntroducePlayer {
                    page_index: 0,
                    char_index: 0,
                    waiting_for_input: false,
                },
            ),
            OakSpeechPhase::IntroducePlayer {
                page_index,
                char_index,
                waiting_for_input,
            } => self.process_text_phase_inline(
                input,
                *page_index,
                *char_index,
                *waiting_for_input,
                INTRODUCE_PLAYER_TEXT_PAGES,
                |pi, ci, wfi| OakSpeechPhase::IntroducePlayer {
                    page_index: pi,
                    char_index: ci,
                    waiting_for_input: wfi,
                },
                || OakSpeechPhase::PlayerNameChoice { cursor: 0 },
            ),
            OakSpeechPhase::IntroduceRival {
                page_index,
                char_index,
                waiting_for_input,
            } => self.process_text_phase_inline(
                input,
                *page_index,
                *char_index,
                *waiting_for_input,
                INTRODUCE_RIVAL_TEXT_PAGES,
                |pi, ci, wfi| OakSpeechPhase::IntroduceRival {
                    page_index: pi,
                    char_index: ci,
                    waiting_for_input: wfi,
                },
                || OakSpeechPhase::RivalNameChoice { cursor: 0 },
            ),
            OakSpeechPhase::FinalSpeech {
                page_index,
                char_index,
                waiting_for_input,
            } => self.process_text_phase_inline(
                input,
                *page_index,
                *char_index,
                *waiting_for_input,
                OAK_SPEECH_TEXT3_PAGES,
                |pi, ci, wfi| OakSpeechPhase::FinalSpeech {
                    page_index: pi,
                    char_index: ci,
                    waiting_for_input: wfi,
                },
                || OakSpeechPhase::ShrinkPlayer { wait_frames: 60 },
            ),
            OakSpeechPhase::PlayerNameChoice { cursor } => {
                let mut new_cursor = *cursor;
                if input.up && new_cursor > 0 {
                    new_cursor -= 1;
                } else if input.down && new_cursor < DEFAULT_PLAYER_NAMES.len() - 1 {
                    new_cursor += 1;
                }
                if input.a {
                    if new_cursor == DEFAULT_PLAYER_NAMES.len() - 1 {
                        self.naming_screen = Some(NamingScreenState::new(NamingScreenType::Player));
                        return Some(OakSpeechPhase::PlayerNaming);
                    } else {
                        let name = DEFAULT_PLAYER_NAMES[new_cursor].to_string();
                        self.player_name = Some(name.clone());
                        return Some(OakSpeechPhase::IntroduceRival {
                            page_index: 0,
                            char_index: 0,
                            waiting_for_input: false,
                        });
                    }
                }
                return Some(OakSpeechPhase::PlayerNameChoice { cursor: new_cursor });
            }
            OakSpeechPhase::PlayerNaming => None,
            OakSpeechPhase::RivalNameChoice { cursor } => {
                let mut new_cursor = *cursor;
                if input.up && new_cursor > 0 {
                    new_cursor -= 1;
                } else if input.down && new_cursor < DEFAULT_RIVAL_NAMES.len() - 1 {
                    new_cursor += 1;
                }
                if input.a {
                    if new_cursor == DEFAULT_RIVAL_NAMES.len() - 1 {
                        self.naming_screen = Some(NamingScreenState::new(NamingScreenType::Rival));
                        return Some(OakSpeechPhase::RivalNaming);
                    } else {
                        let name = DEFAULT_RIVAL_NAMES[new_cursor].to_string();
                        self.rival_name = Some(name.clone());
                        return Some(OakSpeechPhase::FinalSpeech {
                            page_index: 0,
                            char_index: 0,
                            waiting_for_input: false,
                        });
                    }
                }
                return Some(OakSpeechPhase::RivalNameChoice { cursor: new_cursor });
            }
            OakSpeechPhase::RivalNaming => None,
            OakSpeechPhase::ShrinkPlayer { wait_frames } => {
                if *wait_frames > 0 {
                    return Some(OakSpeechPhase::ShrinkPlayer {
                        wait_frames: wait_frames - 1,
                    });
                } else {
                    return Some(OakSpeechPhase::Done);
                }
            }
            OakSpeechPhase::Done => None,
        };
        new_phase
    }

    fn process_text_phase_inline<F, G>(
        &self,
        input: OakSpeechInput,
        page_index: usize,
        char_index: u16,
        waiting_for_input: bool,
        pages: &[TextPage],
        make_current: F,
        make_next: G,
    ) -> Option<OakSpeechPhase>
    where
        F: FnOnce(usize, u16, bool) -> OakSpeechPhase,
        G: FnOnce() -> OakSpeechPhase,
    {
        if waiting_for_input {
            if input.a || input.b {
                let total_pages = pages.len();
                if page_index + 1 >= total_pages {
                    return Some(make_next());
                } else {
                    return Some(make_current(page_index + 1, 0, false));
                }
            }
            return None;
        }

        let page = &pages[page_index];
        let total_chars = page.total_chars(self.player_name.as_deref()) as u16;

        if input.a || input.b {
            return Some(make_current(page_index, total_chars, true));
        }

        let new_char_index = char_index + TYPEWRITER_CHARS_PER_FRAME;
        if new_char_index >= total_chars {
            Some(make_current(page_index, total_chars, true))
        } else {
            Some(make_current(page_index, new_char_index, false))
        }
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

    pub fn current_char_index(&self) -> u16 {
        match &self.phase {
            OakSpeechPhase::Greeting { char_index, .. } => *char_index,
            OakSpeechPhase::ShowNidorino { char_index, .. } => *char_index,
            OakSpeechPhase::Explanation { char_index, .. } => *char_index,
            OakSpeechPhase::IntroducePlayer { char_index, .. } => *char_index,
            OakSpeechPhase::IntroduceRival { char_index, .. } => *char_index,
            OakSpeechPhase::FinalSpeech { char_index, .. } => *char_index,
            _ => 0,
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

    pub fn current_intro_text(&self) -> Option<String> {
        let page = self.current_text_page()?;
        let player_name = self.player_name.as_deref();
        let l1 = page.line1.replace("<PLAYER>", player_name.unwrap_or("RED"));
        let l2 = page.line2.replace("<PLAYER>", player_name.unwrap_or("RED"));
        if l2.is_empty() {
            Some(l1)
        } else {
            Some(format!("{} {}", l1, l2))
        }
    }

    pub fn get_display_text(&self, text: &str) -> String {
        text.replace("<PLAYER>", self.player_name.as_deref().unwrap_or("RED"))
    }
}

impl Default for OakSpeechState {
    fn default() -> Self {
        Self::new()
    }
}
