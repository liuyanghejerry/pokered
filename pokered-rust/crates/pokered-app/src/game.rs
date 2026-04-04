use std::path::{Path, PathBuf};

use pokered_audio::music_data::MusicId;
use pokered_audio::sfx_data::SfxId;
use pokered_core::battle::{BattleInput, BattleScreen};
use pokered_core::data::maps::MapId;
use pokered_core::data::wild_data::GameVersion;
use pokered_core::game_state::{GameScreen, GameState, SaveFileSummary, ScreenAction};
use pokered_core::intro_scene::{IntroSceneState, IntroSfxEvent};
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
    draw_battle, draw_intro_scene, draw_main_menu, draw_oak_speech, draw_options_menu,
    draw_overworld, draw_save_menu, draw_start_menu, draw_title_screen, BattleVisualEffects,
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

fn save_summary_from_data(save: &SaveData) -> SaveFileSummary {
    SaveFileSummary {
        player_name: save.player_name.clone(),
        badges: save.game_data.obtained_badges,
        pokedex_owned: save.game_data.pokedex.owned_count() as u8,
        play_time_hours: save.game_data.play_time.hours as u16,
        play_time_minutes: save.game_data.play_time.minutes,
        play_time_seconds: save.game_data.play_time.seconds,
    }
}

pub struct PokemonGame {
    pub state: GameState,
    pub title_screen: TitleScreenState,
    pub intro_scene: IntroSceneState,
    pub main_menu: MainMenuState,
    pub oak_speech: OakSpeechState,
    pub overworld: OverworldScreen,
    pub battle: BattleScreen,
    pub battle_vfx: BattleVisualEffects,
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
    pub black_screen_frames: u32,
    pub pending_screen: Option<GameScreen>,
    pub scripts_dir: Option<PathBuf>,
    #[cfg(not(target_arch = "wasm32"))]
    pub audio: Option<AudioOutput>,
}

impl PokemonGame {
    pub fn new(
        version: GameVersion,
        save_path: Option<PathBuf>,
        snapshot_path: Option<PathBuf>,
        scripts_dir: Option<PathBuf>,
    ) -> Self {
        let (save_data, save_summary) = if let Some(ref path) = snapshot_path {
            Self::load_snapshot_from_path(path)
        } else if let Some(ref path) = save_path {
            Self::load_sram_from_path(path)
        } else {
            Self::try_load_default_save()
        };
        let state = GameState {
            screen: GameScreen::CopyrightSplash,
            config: pokered_core::game_state::GameConfig::new(version),
            save_summary: save_summary.clone(),
        };
        let title_screen = TitleScreenState::new(version);
        let main_menu = MainMenuState::new(save_summary);
        let oak_speech = OakSpeechState::new();
        let overworld = OverworldScreen::new(MapId::PalletTown, scripts_dir.clone());
        let battle = BattleScreen::new(true);
        let battle_vfx = BattleVisualEffects::default();
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
            intro_scene: IntroSceneState::new(),
            main_menu,
            oak_speech,
            overworld,
            battle,
            battle_vfx,
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
            black_screen_frames: 0,
            pending_screen: None,
            scripts_dir,
            #[cfg(not(target_arch = "wasm32"))]
            audio,
        }
    }

    fn try_load_default_save() -> (SaveData, Option<SaveFileSummary>) {
        let path = save_file_path();
        match std::fs::read(&path) {
            Ok(data) => Self::parse_sram(&path, &data),
            Err(_) => (SaveData::new(), None),
        }
    }

    fn load_sram_from_path(path: &Path) -> (SaveData, Option<SaveFileSummary>) {
        match std::fs::read(path) {
            Ok(data) => Self::parse_sram(path, &data),
            Err(e) => {
                eprintln!("Error: failed to read save file {:?}: {}", path, e);
                (SaveData::new(), None)
            }
        }
    }

    fn load_snapshot_from_path(path: &Path) -> (SaveData, Option<SaveFileSummary>) {
        match std::fs::read(path) {
            Ok(data) => match serde_json::from_slice::<SaveData>(&data) {
                Ok(save) => {
                    let summary = save_summary_from_data(&save);
                    eprintln!("Snapshot loaded: {:?}", path);
                    (save, Some(summary))
                }
                Err(e) => {
                    eprintln!("Error: failed to parse snapshot {:?}: {}", path, e);
                    (SaveData::new(), None)
                }
            },
            Err(e) => {
                eprintln!("Error: failed to read snapshot {:?}: {}", path, e);
                (SaveData::new(), None)
            }
        }
    }

    fn parse_sram(path: &Path, data: &[u8]) -> (SaveData, Option<SaveFileSummary>) {
        match import_sram(data) {
            Ok(save) => {
                let summary = save_summary_from_data(&save);
                eprintln!("Save file loaded: {:?}", path);
                pokered_core::log_save!(
                    "position: map_id={}, x={}, y={}, dir={}",
                    save.game_data.position.map_id,
                    save.game_data.position.x,
                    save.game_data.position.y,
                    save.game_data.player_direction
                );
                (save, Some(summary))
            }
            Err(e) => {
                eprintln!("Warning: save file {:?} failed to load: {:?}", path, e);
                (SaveData::new(), None)
            }
        }
    }

    pub fn export_snapshot_from_sav(
        input_path: Option<&Path>,
        output_path: &Path,
    ) -> Result<(), String> {
        let sav_path = input_path
            .map(|p| p.to_path_buf())
            .unwrap_or_else(save_file_path);
        let data = std::fs::read(&sav_path)
            .map_err(|e| format!("Failed to read {:?}: {}", sav_path, e))?;
        let save = import_sram(&data)
            .map_err(|e| format!("Failed to parse SRAM from {:?}: {:?}", sav_path, e))?;
        let json = serde_json::to_string_pretty(&save)
            .map_err(|e| format!("Failed to serialize snapshot: {}", e))?;
        std::fs::write(output_path, json.as_bytes())
            .map_err(|e| format!("Failed to write {:?}: {}", output_path, e))?;
        eprintln!(
            "Exported snapshot: {:?} -> {:?} ({} bytes)",
            sav_path,
            output_path,
            json.len()
        );
        Ok(())
    }

    fn build_save_data(&self) -> SaveData {
        let mut save = self.save_data.clone();
        if let Some(encoded) = pokered_data::charmap::encode_string(&self.player_name) {
            save.player_name = encoded;
        }
        if let Some(encoded) = pokered_data::charmap::encode_string(&self.rival_name) {
            save.game_data.rival_name = encoded;
        }

        let player = &self.overworld.state.player;
        let current_map = self.overworld.state.current_map;

        save.game_data.position.map_id = current_map as u8;
        save.game_data.position.x = player.x as u8;
        save.game_data.position.y = player.y as u8;
        save.game_data.position.x_block = (player.x % 2) as u8;
        save.game_data.position.y_block = (player.y % 2) as u8;

        let facing = match player.facing {
            pokered_core::overworld::Direction::Down => 0u8,
            pokered_core::overworld::Direction::Up => 4u8,
            pokered_core::overworld::Direction::Left => 8u8,
            pokered_core::overworld::Direction::Right => 12u8,
        };
        save.game_data.player_direction = facing;
        save.game_data.player_last_stop_direction = facing;
        save.game_data.player_moving_direction = facing;

        pokered_core::log_save!(
            "build_save_data: map_id={}, x={}, y={}, dir={}, player.x={}, player.y={}",
            save.game_data.position.map_id,
            save.game_data.position.x,
            save.game_data.position.y,
            facing,
            player.x,
            player.y
        );

        // wCurrentMapHeight2/Width2 = block dimensions × 2
        let (map_w, map_h) = current_map.dimensions();
        save.game_data.current_map_height2 = map_h * 2;
        save.game_data.current_map_width2 = map_w * 2;

        if let Some(ref map_data) = self.overworld.map_data {
            save.game_data.map_header.tileset = map_data.tileset as u8;
            save.game_data.map_header.height = map_data.height;
            save.game_data.map_header.width = map_data.width;
        }

        save
    }

    fn save_to_file(&mut self) {
        let save = self.build_save_data();
        let sram = export_sram(&save);
        let path = save_file_path();
        match std::fs::write(&path, &sram) {
            Ok(()) => {
                pokered_core::log_save!("game saved to {:?} ({} bytes)", path, sram.len());
                self.save_data = save;
            }
            Err(e) => {
                eprintln!("Error: failed to write save file: {}", e);
            }
        }
    }

    pub fn handle_transition(&mut self, screen: GameScreen) {
        match screen {
            GameScreen::IntroScene => {
                self.intro_scene.reset();
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(ref audio) = self.audio {
                    audio.play_music(MusicId::INTRO_BATTLE);
                }
            }
            GameScreen::TitleScreen => {
                let coming_from_intro = self.state.screen == GameScreen::IntroScene;
                self.title_screen.reset();
                if coming_from_intro {
                    // Skip copyright — go straight to Init (logo bounce etc.)
                    self.title_screen.phase = TitlePhase::Init;
                }
                self.prev_title_phase = Some(self.title_screen.phase);
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
                use pokered_core::game_state::MainMenuChoice;

                // Only create a new OverworldScreen when entering from the main menu
                // (Continue or New Game). When returning from sub-screens (Start menu,
                // Options, Save, Battle), keep the existing overworld state intact.
                match self.main_menu.last_choice {
                    Some(MainMenuChoice::Continue)
                        if self.state.screen != GameScreen::Overworld
                            && self.state.screen != GameScreen::StartMenu
                            && self.state.screen != GameScreen::OptionsMenu
                            && self.state.screen != GameScreen::SaveMenu
                            && self.state.screen != GameScreen::Battle =>
                    {
                        let pos = &self.save_data.game_data.position;
                        pokered_core::log_save!(
                            "continue: loading from save: map_id={}, x={}, y={}, dir={}",
                            pos.map_id,
                            pos.x,
                            pos.y,
                            self.save_data.game_data.player_direction
                        );
                        let map_id = pokered_core::data::maps::MapId::from_u8(pos.map_id)
                            .unwrap_or(NEW_GAME_WARP.map_id);
                        let mut overworld = OverworldScreen::new(map_id, self.scripts_dir.clone());
                        overworld.state.player.x = pos.x as u16;
                        overworld.state.player.y = pos.y as u16;
                        overworld.state.player.facing =
                            match self.save_data.game_data.player_direction {
                                4 => pokered_core::overworld::Direction::Up,
                                8 => pokered_core::overworld::Direction::Left,
                                12 => pokered_core::overworld::Direction::Right,
                                _ => pokered_core::overworld::Direction::Down,
                            };
                        self.player_name =
                            pokered_data::charmap::decode_string(&self.save_data.player_name);
                        self.rival_name = pokered_data::charmap::decode_string(
                            &self.save_data.game_data.rival_name,
                        );
                        self.overworld = overworld;
                        pokered_core::log_save!(
                            "continue: overworld created: player x={}, y={}, map={:?}",
                            self.overworld.state.player.x,
                            self.overworld.state.player.y,
                            self.overworld.state.current_map
                        );
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Some(ref audio) = self.audio {
                            audio.play_music(MusicId::PALLET_TOWN);
                        }
                    }
                    Some(MainMenuChoice::NewGame)
                        if self.state.screen != GameScreen::Overworld
                            && self.state.screen != GameScreen::StartMenu
                            && self.state.screen != GameScreen::OptionsMenu
                            && self.state.screen != GameScreen::SaveMenu
                            && self.state.screen != GameScreen::Battle =>
                    {
                        let mut overworld =
                            OverworldScreen::new(NEW_GAME_WARP.map_id, self.scripts_dir.clone());
                        overworld.state.player.x = NEW_GAME_WARP.coords.x as u16;
                        overworld.state.player.y = NEW_GAME_WARP.coords.y as u16;
                        self.overworld = overworld;
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Some(ref audio) = self.audio {
                            audio.play_music(MusicId::PALLET_TOWN);
                        }
                    }
                    _ => {
                        // Returning from sub-screen — keep existing overworld intact
                    }
                }
            }
            GameScreen::Battle => {
                self.battle = BattleScreen::new(true);
                self.battle_vfx = BattleVisualEffects::default();
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

const BLACK_SCREEN_DURATION: u32 = 30;

impl GameLoop for PokemonGame {
    fn update(&mut self, input: &InputState) {
        self.frame_count += 1;

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(ref audio) = self.audio {
            audio.update_frame();
        }

        if self.black_screen_frames > 0 {
            self.black_screen_frames -= 1;
            if self.black_screen_frames == 0 {
                if let Some(screen) = self.pending_screen.take() {
                    self.handle_transition(screen);
                }
            }
            return;
        }

        let action = match self.state.screen {
            GameScreen::CopyrightSplash => {
                let any_pressed = input.any_just_pressed();
                let action = self.title_screen.update_frame(any_pressed);
                if self.title_screen.phase == TitlePhase::Init {
                    ScreenAction::Transition(GameScreen::IntroScene)
                } else {
                    action
                }
            }
            GameScreen::IntroScene => {
                let any_pressed = input.any_just_pressed();
                let action = self.intro_scene.update_frame(any_pressed);
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(ref audio) = self.audio {
                    match self.intro_scene.sfx_event {
                        IntroSfxEvent::IntroHip => audio.play_sfx(SfxId::IntroHip),
                        IntroSfxEvent::IntroHop => audio.play_sfx(SfxId::IntroHop),
                        IntroSfxEvent::IntroRaise => audio.play_sfx(SfxId::IntroRaise),
                        IntroSfxEvent::IntroCrash => audio.play_sfx(SfxId::IntroCrash),
                        IntroSfxEvent::IntroLunge => audio.play_sfx(SfxId::IntroLunge),
                        IntroSfxEvent::None => {}
                    }
                }
                action
            }
            GameScreen::TitleScreen => {
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
                let action = self.battle.update_frame(battle_input);
                self.battle_vfx.update(&self.battle);
                action
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
            use pokered_core::game_state::MainMenuChoice;
            let needs_black_screen = new_screen == GameScreen::Overworld
                && self.state.screen == GameScreen::MainMenu
                && self.main_menu.last_choice == Some(MainMenuChoice::Continue);

            if needs_black_screen {
                self.black_screen_frames = BLACK_SCREEN_DURATION;
                self.pending_screen = Some(new_screen);
            } else {
                self.handle_transition(new_screen);
            }
        }
    }

    fn draw(&mut self, frame_buffer: &mut FrameBuffer) {
        if self.black_screen_frames > 0 {
            frame_buffer.clear(Rgba::BLACK);
            return;
        }

        frame_buffer.clear(Rgba::WHITE);

        match self.state.screen {
            GameScreen::CopyrightSplash => {
                draw_title_screen(&self.title_screen, true, &mut self.resources, frame_buffer);
            }
            GameScreen::IntroScene => {
                draw_intro_scene(&self.intro_scene, &mut self.resources, frame_buffer);
            }
            GameScreen::TitleScreen => {
                draw_title_screen(&self.title_screen, false, &mut self.resources, frame_buffer);
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
                draw_battle(
                    &self.battle,
                    &mut self.resources,
                    frame_buffer,
                    &mut self.battle_vfx,
                );
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
