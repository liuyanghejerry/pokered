use pokered_audio::music_data::MusicId;
use pokered_audio::sfx_data::SfxId;
use pokered_core::battle::{BattleInput, BattleScreen};
use pokered_core::data::maps::MapId;
use pokered_core::data::wild_data::GameVersion;
use pokered_core::game_state::{GameScreen, GameState, SaveFileSummary, ScreenAction};
use pokered_core::main_menu::{MainMenuState, MenuInput};
use pokered_core::naming_screen::NamingInput;
use pokered_core::oak_speech::{OakSpeechInput, OakSpeechPhase, OakSpeechResult, OakSpeechState};
use pokered_core::options_menu::{GameOptions, OptionsInput, OptionsMenuResult, OptionsMenuState};
use pokered_core::overworld::{OverworldInput, OverworldScreen};
use pokered_core::save::sram_export::export_sram;
use pokered_core::save::sram_import::import_sram;
use pokered_core::save::SaveData;
use pokered_core::save_menu::{SaveMenuResult, SaveMenuState, SaveScreenInfo, YesNoInput};
use pokered_core::start_menu::{StartMenuAction, StartMenuInput, StartMenuState};
use pokered_core::title_screen::{TitlePhase, TitleScreenState};
use pokered_renderer::input::{GbButton, InputState};
use pokered_renderer::resource::{AssetRoot, ResourceManager};
use pokered_renderer::window::GameLoop;
use pokered_renderer::{FrameBuffer, Rgba};

use crate::audio::{species_to_cry, AudioOutput};
use crate::render::{
    draw_battle, draw_main_menu, draw_oak_speech, draw_options_menu, draw_overworld,
    draw_save_menu, draw_start_menu, draw_title_screen,
};

const SAVE_FILE_NAME: &str = "pokered.sav";

fn oak_phase_tag(phase: &OakSpeechPhase) -> u8 {
    match phase {
        OakSpeechPhase::Greeting { .. } => 1,
        OakSpeechPhase::ShowNidorino { .. } => 2,
        OakSpeechPhase::Explanation { .. } => 3,
        OakSpeechPhase::IntroducePlayer { .. } => 4,
        OakSpeechPhase::PlayerNameChoice { .. } => 5,
        OakSpeechPhase::PlayerNaming => 6,
        OakSpeechPhase::IntroduceRival { .. } => 7,
        OakSpeechPhase::RivalNameChoice { .. } => 8,
        OakSpeechPhase::RivalNaming => 9,
        OakSpeechPhase::FinalSpeech { .. } => 10,
        OakSpeechPhase::ShrinkPlayer { .. } => 11,
        OakSpeechPhase::Done => 12,
    }
}

fn save_file_path() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join(SAVE_FILE_NAME)))
        .unwrap_or_else(|| std::path::PathBuf::from(SAVE_FILE_NAME))
}

pub struct PokemonGame {
    pub state: GameState,
    pub title_screen: TitleScreenState,
    pub main_menu: MainMenuState,
    pub oak_speech: OakSpeechState,
    pub overworld: OverworldScreen,
    pub battle: BattleScreen,
    pub start_menu: StartMenuState,
    pub options_menu: OptionsMenuState,
    pub save_menu: SaveMenuState,
    pub save_data: SaveData,
    pub player_name: String,
    pub rival_name: String,
    pub frame_count: u64,
    pub exit_requested: bool,
    pub resources: Option<ResourceManager>,
    prev_title_phase: Option<TitlePhase>,
    prev_oak_phase_tag: u8,
    #[cfg(not(target_arch = "wasm32"))]
    pub audio: Option<AudioOutput>,
}

impl PokemonGame {
    pub fn new(version: GameVersion) -> Self {
        let (save_data, save_summary) = Self::try_load_save();
        let state = GameState {
            screen: GameScreen::CopyrightSplash,
            config: pokered_core::game_state::GameConfig::new(version),
            save_summary: save_summary.clone(),
        };
        let title_screen = TitleScreenState::new(version);
        let main_menu = MainMenuState::new(save_summary);
        let oak_speech = OakSpeechState::new();
        let overworld = OverworldScreen::new(MapId::PalletTown);
        let battle = BattleScreen::new(true);
        let start_menu = StartMenuState::new(false, false, false);
        let options_menu = OptionsMenuState::new(GameOptions::default());
        let save_menu = SaveMenuState::new(
            SaveScreenInfo {
                player_name: "RED".to_string(),
                num_badges: 0,
                pokedex_owned: 0,
                play_time_hours: 0,
                play_time_minutes: 0,
            },
            false,
            false,
        );

        let resources = match AssetRoot::auto_detect() {
            Ok(root) => {
                eprintln!("Asset root found: {:?}", root.gfx_dir());
                Some(ResourceManager::new(root))
            }
            Err(e) => {
                eprintln!("Warning: Could not find gfx/ directory: {}", e);
                eprintln!("Falling back to text-only placeholder rendering.");
                None
            }
        };

        #[cfg(not(target_arch = "wasm32"))]
        let audio = match AudioOutput::new() {
            Some(ao) => {
                eprintln!("Audio output initialized (cpal 44100 Hz stereo)");
                Some(ao)
            }
            None => {
                eprintln!("Warning: Could not initialize audio output.");
                None
            }
        };

        Self {
            state,
            title_screen,
            main_menu,
            oak_speech,
            overworld,
            battle,
            start_menu,
            options_menu,
            save_menu,
            save_data,
            player_name: "RED".to_string(),
            rival_name: "BLUE".to_string(),
            frame_count: 0,
            exit_requested: false,
            resources,
            prev_title_phase: None,
            prev_oak_phase_tag: 0,
            #[cfg(not(target_arch = "wasm32"))]
            audio,
        }
    }

    fn try_load_save() -> (SaveData, Option<SaveFileSummary>) {
        let path = save_file_path();
        let data = match std::fs::read(&path) {
            Ok(d) => d,
            Err(_) => return (SaveData::new(), None),
        };
        match import_sram(&data) {
            Ok(save) => {
                let summary = SaveFileSummary {
                    player_name: save.player_name.clone(),
                    badges: save.game_data.obtained_badges,
                    pokedex_owned: save.game_data.pokedex.owned_count() as u8,
                    play_time_hours: save.game_data.play_time.hours as u16,
                    play_time_minutes: save.game_data.play_time.minutes,
                    play_time_seconds: save.game_data.play_time.seconds,
                };
                eprintln!("Save file loaded: {:?}", path);
                (save, Some(summary))
            }
            Err(e) => {
                eprintln!("Warning: save file exists but failed to load: {:?}", e);
                (SaveData::new(), None)
            }
        }
    }

    fn build_save_data(&self) -> SaveData {
        let mut save = self.save_data.clone();
        if let Some(encoded) = pokered_data::charmap::encode_string(&self.player_name) {
            save.player_name = encoded;
        }
        if let Some(encoded) = pokered_data::charmap::encode_string(&self.rival_name) {
            save.game_data.rival_name = encoded;
        }
        save
    }

    fn save_to_file(&mut self) {
        let save = self.build_save_data();
        let sram = export_sram(&save);
        let path = save_file_path();
        match std::fs::write(&path, &sram) {
            Ok(()) => {
                eprintln!("Game saved to {:?} ({} bytes)", path, sram.len());
                self.save_data = save;
            }
            Err(e) => {
                eprintln!("Error: failed to write save file: {}", e);
            }
        }
    }

    pub fn handle_transition(&mut self, screen: GameScreen) {
        match screen {
            GameScreen::TitleScreen => {
                self.title_screen.reset();
                self.prev_title_phase = Some(TitlePhase::Copyright);
            }
            GameScreen::MainMenu => {
                self.main_menu = MainMenuState::new(self.state.save_summary.clone());
            }
            GameScreen::OakSpeech => {
                self.oak_speech = OakSpeechState::new();
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(ref audio) = self.audio {
                    audio.stop_all();
                    audio.play_music(MusicId::ROUTES2);
                }
            }
            GameScreen::Overworld => {
                use pokered_core::data::fly_warp_data::NEW_GAME_WARP;
                let mut overworld = OverworldScreen::new(NEW_GAME_WARP.map_id);
                overworld.state.player.x = NEW_GAME_WARP.coords.x as u16;
                overworld.state.player.y = NEW_GAME_WARP.coords.y as u16;
                overworld.start_bedroom_dialogue(&self.player_name);
                self.overworld = overworld;
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(ref audio) = self.audio {
                    audio.play_music(MusicId::PALLET_TOWN);
                }
            }
            GameScreen::Battle => {
                self.battle = BattleScreen::new(true);
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(ref audio) = self.audio {
                    audio.play_music(MusicId::WILD_BATTLE);
                }
            }
            GameScreen::StartMenu => {
                self.start_menu.open(false, false, false);
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(ref audio) = self.audio {
                    audio.play_sfx(SfxId::StartMenu);
                }
            }
            GameScreen::OptionsMenu => {
                self.options_menu = OptionsMenuState::new(GameOptions::default());
            }
            GameScreen::SaveMenu => {
                let has_previous = self.state.has_save_file();
                self.save_menu = SaveMenuState::new(
                    SaveScreenInfo {
                        player_name: self.player_name.clone(),
                        num_badges: self.save_data.game_data.badge_count(),
                        pokedex_owned: self.save_data.game_data.pokedex.owned_count() as u16,
                        play_time_hours: self.save_data.game_data.play_time.hours as u16,
                        play_time_minutes: self.save_data.game_data.play_time.minutes,
                    },
                    has_previous,
                    false,
                );
            }
            GameScreen::CopyrightSplash => {
                self.title_screen.reset();
            }
        }
        self.state.transition_to(screen);
    }
}

impl GameLoop for PokemonGame {
    fn update(&mut self, input: &InputState) {
        self.frame_count += 1;

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(ref audio) = self.audio {
            audio.update_frame();
        }

        let action = match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                let prev_phase = self.title_screen.phase;
                let any_pressed = input.any_just_pressed();
                let action = self.title_screen.update_frame(any_pressed);
                let new_phase = self.title_screen.phase;

                #[cfg(not(target_arch = "wasm32"))]
                if prev_phase != new_phase {
                    if let Some(ref audio) = self.audio {
                        match new_phase {
                            TitlePhase::LogoBounce => {
                                audio.play_sfx(SfxId::IntroCrash);
                            }
                            TitlePhase::LogoPause => {
                                audio.play_sfx(SfxId::IntroWhoosh);
                            }
                            TitlePhase::WaitingForInput
                                if prev_phase == TitlePhase::VersionScroll =>
                            {
                                audio.play_music(MusicId::TITLE_SCREEN);
                            }
                            TitlePhase::PlayingCry => {
                                let cry = species_to_cry(self.title_screen.current_mon);
                                audio.play_sfx(cry);
                            }
                            _ => {}
                        }
                    }
                }
                action
            }
            GameScreen::MainMenu => {
                let menu_input = MenuInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    a: input.is_just_pressed(GbButton::A) || input.is_just_pressed(GbButton::Start),
                    b: input.is_just_pressed(GbButton::B),
                };
                if input.is_just_pressed(GbButton::A) || input.is_just_pressed(GbButton::Start) {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(ref audio) = self.audio {
                        audio.play_sfx(SfxId::PressAB);
                    }
                }
                self.main_menu.update_frame(menu_input)
            }
            GameScreen::OakSpeech => {
                let prev_tag = self.prev_oak_phase_tag;
                let result = if self.oak_speech.is_naming_active() {
                    let naming_input = NamingInput {
                        up: input.is_just_pressed(GbButton::Up),
                        down: input.is_just_pressed(GbButton::Down),
                        left: input.is_just_pressed(GbButton::Left),
                        right: input.is_just_pressed(GbButton::Right),
                        a: input.is_just_pressed(GbButton::A),
                        b: input.is_just_pressed(GbButton::B),
                        start: input.is_just_pressed(GbButton::Start),
                        select: input.is_just_pressed(GbButton::Select),
                    };
                    self.oak_speech.update_naming_frame(naming_input)
                } else {
                    let oak_input = OakSpeechInput {
                        up: input.is_just_pressed(GbButton::Up),
                        down: input.is_just_pressed(GbButton::Down),
                        a: input.is_just_pressed(GbButton::A),
                        b: input.is_just_pressed(GbButton::B),
                    };
                    self.oak_speech.update_frame(oak_input)
                };
                let new_tag = oak_phase_tag(&self.oak_speech.phase);

                #[cfg(not(target_arch = "wasm32"))]
                if prev_tag != new_tag {
                    if let Some(ref audio) = self.audio {
                        match &self.oak_speech.phase {
                            OakSpeechPhase::ShowNidorino { .. } if prev_tag != new_tag => {
                                audio.play_sfx(species_to_cry(
                                    pokered_data::species::Species::Nidorino,
                                ));
                            }
                            OakSpeechPhase::ShrinkPlayer { .. } => {
                                audio.play_sfx(SfxId::Shrink);
                            }
                            _ => {}
                        }
                    }
                    self.prev_oak_phase_tag = new_tag;
                }

                if input.is_just_pressed(GbButton::A) {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(ref audio) = self.audio {
                        audio.play_sfx(SfxId::PressAB);
                    }
                }

                match result {
                    OakSpeechResult::PlayerNameSet(name) => {
                        self.player_name = name;
                        ScreenAction::Continue
                    }
                    OakSpeechResult::RivalNameSet(name) => {
                        self.rival_name = name;
                        ScreenAction::Continue
                    }
                    OakSpeechResult::Finished => ScreenAction::Transition(GameScreen::Overworld),
                    OakSpeechResult::Active => ScreenAction::Continue,
                }
            }
            GameScreen::Overworld => {
                let ow_input = OverworldInput {
                    up: input.is_held(GbButton::Up),
                    down: input.is_held(GbButton::Down),
                    left: input.is_held(GbButton::Left),
                    right: input.is_held(GbButton::Right),
                    a: input.is_held(GbButton::A),
                    b: input.is_held(GbButton::B),
                    start: input.is_just_pressed(GbButton::Start),
                    select: input.is_held(GbButton::Select),
                };
                self.overworld.update_frame(ow_input)
            }
            GameScreen::Battle => {
                let battle_input = BattleInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    left: input.is_just_pressed(GbButton::Left),
                    right: input.is_just_pressed(GbButton::Right),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                };
                self.battle.update_frame(battle_input)
            }
            GameScreen::StartMenu => {
                let sm_input = StartMenuInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                    start: input.is_just_pressed(GbButton::Start),
                };
                match self.start_menu.update_frame(sm_input) {
                    StartMenuAction::Close => ScreenAction::Transition(GameScreen::Overworld),
                    StartMenuAction::OpenOption => {
                        ScreenAction::Transition(GameScreen::OptionsMenu)
                    }
                    StartMenuAction::OpenSave => ScreenAction::Transition(GameScreen::SaveMenu),
                    // Sub-screens not yet implemented — redisplay start menu
                    // (matches ASM behavior: empty bag returns to menu,
                    // no party returns to menu, etc.)
                    StartMenuAction::OpenItem
                    | StartMenuAction::OpenPokedex
                    | StartMenuAction::OpenPokemon
                    | StartMenuAction::OpenTrainerInfo => {
                        self.start_menu.redisplay();
                        ScreenAction::Continue
                    }
                    _ => ScreenAction::Continue,
                }
            }
            GameScreen::OptionsMenu => {
                let opt_input = OptionsInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    left: input.is_just_pressed(GbButton::Left),
                    right: input.is_just_pressed(GbButton::Right),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                    start: input.is_just_pressed(GbButton::Start),
                };
                match self.options_menu.tick(opt_input) {
                    OptionsMenuResult::Closed => ScreenAction::Transition(GameScreen::StartMenu),
                    OptionsMenuResult::Active => ScreenAction::Continue,
                }
            }
            GameScreen::SaveMenu => {
                let save_input = YesNoInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                };
                match self.save_menu.tick(save_input) {
                    SaveMenuResult::Saved => {
                        self.save_to_file();
                        ScreenAction::Transition(GameScreen::StartMenu)
                    }
                    SaveMenuResult::Cancelled => ScreenAction::Transition(GameScreen::StartMenu),
                    SaveMenuResult::Active => ScreenAction::Continue,
                }
            }
        };

        if let ScreenAction::Transition(new_screen) = action {
            self.handle_transition(new_screen);
        }
    }

    fn draw(&mut self, frame_buffer: &mut FrameBuffer) {
        frame_buffer.clear(Rgba::WHITE);

        match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                draw_title_screen(
                    &self.title_screen,
                    self.title_screen.is_copyright(),
                    &mut self.resources,
                    frame_buffer,
                );
            }
            GameScreen::MainMenu => {
                draw_main_menu(&self.main_menu, frame_buffer);
            }
            GameScreen::OakSpeech => {
                draw_oak_speech(&self.oak_speech, &mut self.resources, frame_buffer);
            }
            GameScreen::Overworld => {
                draw_overworld(&self.overworld, &mut self.resources, frame_buffer);
            }
            GameScreen::Battle => {
                draw_battle(&self.battle, &mut self.resources, frame_buffer);
            }
            GameScreen::StartMenu => {
                draw_overworld(&self.overworld, &mut self.resources, frame_buffer);
                draw_start_menu(&self.start_menu, &self.player_name, frame_buffer);
            }
            GameScreen::OptionsMenu => {
                draw_options_menu(&self.options_menu, frame_buffer);
            }
            GameScreen::SaveMenu => {
                draw_overworld(&self.overworld, &mut self.resources, frame_buffer);
                draw_save_menu(&self.save_menu, frame_buffer);
            }
        }
    }

    fn should_exit(&self) -> bool {
        self.exit_requested
    }
}
