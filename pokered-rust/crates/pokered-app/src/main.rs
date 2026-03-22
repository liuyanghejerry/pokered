use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use pokered_core::battle::{BattleInput, BattlePhase, BattleScreen};
use pokered_core::data::maps::MapId;
use pokered_core::data::wild_data::GameVersion;
use pokered_core::game_state::{GameScreen, GameState, ScreenAction};
use pokered_core::main_menu::{MainMenuState, MenuInput};
use pokered_core::naming_screen::NamingInput;
use pokered_core::oak_speech::{
    OakSpeechInput, OakSpeechPhase, OakSpeechResult, OakSpeechState, DEFAULT_PLAYER_NAMES,
    DEFAULT_RIVAL_NAMES,
};
use pokered_core::options_menu::{GameOptions, OptionsInput, OptionsMenuResult, OptionsMenuState};
use pokered_core::overworld::{Direction, OverworldInput, OverworldScreen};
use pokered_core::save_menu::{SaveMenuResult, SaveMenuState, SaveScreenInfo, YesNoInput};
use pokered_core::start_menu::{StartMenuAction, StartMenuInput, StartMenuState};
use pokered_core::title_screen::TitleScreenState;
use pokered_renderer::embedded_font::{box_tiles, draw_glyph, draw_text, fill_tile};
use pokered_renderer::input::{GbButton, InputState};
use pokered_renderer::palette::{Palette, GRAYSCALE_PALETTE};
use pokered_renderer::resource::{AssetCategory, AssetRoot, ResourceManager};
use pokered_renderer::tile::TileSet;
use pokered_renderer::window::{run, GameLoop, GameWindowConfig};
use pokered_renderer::{FrameBuffer, Rgba, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

#[derive(Parser)]
#[command(name = "pokered", about = "Pokémon Red/Blue — Rust Rewrite")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the game in windowed mode (default)
    Run,
    /// Capture a screenshot of a specific game screen
    Screenshot {
        /// Which screen to capture
        #[arg(short, long)]
        screen: ScreenTarget,
        /// Output PNG file path
        #[arg(short, long, default_value = "screenshot.png")]
        output: PathBuf,
        /// Number of frames to advance before capturing (for animation)
        #[arg(short, long, default_value_t = 5)]
        frames: u32,
    },
    /// Capture screenshots of all game screens
    ScreenshotAll {
        /// Output directory for PNG files
        #[arg(short, long, default_value = "screenshots")]
        output_dir: PathBuf,
        /// Number of frames to advance before capturing each screen
        #[arg(short, long, default_value_t = 5)]
        frames: u32,
    },
}

#[derive(Clone, ValueEnum)]
enum ScreenTarget {
    Copyright,
    Title,
    MainMenu,
    Oak,
    Overworld,
    Battle,
    StartMenu,
    Options,
    Save,
}

struct PokemonGame {
    state: GameState,
    title_screen: TitleScreenState,
    main_menu: MainMenuState,
    oak_speech: OakSpeechState,
    overworld: OverworldScreen,
    battle: BattleScreen,
    start_menu: StartMenuState,
    options_menu: OptionsMenuState,
    save_menu: SaveMenuState,
    player_name: String,
    rival_name: String,
    frame_count: u64,
    exit_requested: bool,
    resources: Option<ResourceManager>,
}

impl PokemonGame {
    fn new(version: GameVersion) -> Self {
        let state = GameState::new(version);
        let title_screen = TitleScreenState::new(version);
        let main_menu = MainMenuState::new(None);
        let oak_speech = OakSpeechState::new();
        let overworld = OverworldScreen::new(MapId::PalletTown);
        let battle = BattleScreen::new(true);
        let start_menu = StartMenuState::new(false, false);
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

        // Try to auto-detect asset root (walks up from CWD to find gfx/)
        let resources = match AssetRoot::auto_detect() {
            Ok(root) => {
                println!("Asset root found: {:?}", root.gfx_dir());
                Some(ResourceManager::new(root))
            }
            Err(e) => {
                eprintln!("Warning: Could not find gfx/ directory: {}", e);
                eprintln!("Falling back to text-only placeholder rendering.");
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
            player_name: "RED".to_string(),
            rival_name: "BLUE".to_string(),
            frame_count: 0,
            exit_requested: false,
            resources,
        }
    }

    fn handle_transition(&mut self, screen: GameScreen) {
        match screen {
            GameScreen::TitleScreen => {
                self.title_screen.reset();
            }
            GameScreen::MainMenu => {
                self.main_menu = MainMenuState::new(None);
            }
            GameScreen::OakSpeech => {
                self.oak_speech = OakSpeechState::new();
            }
            GameScreen::Overworld => {
                self.overworld = OverworldScreen::new(MapId::PalletTown);
            }
            GameScreen::Battle => {
                self.battle = BattleScreen::new(true);
            }
            GameScreen::StartMenu => {
                self.start_menu.open(false, false);
            }
            GameScreen::OptionsMenu => {
                self.options_menu = OptionsMenuState::new(GameOptions::default());
            }
            GameScreen::SaveMenu => {
                self.save_menu = SaveMenuState::new(
                    SaveScreenInfo {
                        player_name: self.player_name.clone(),
                        num_badges: 0,
                        pokedex_owned: 0,
                        play_time_hours: 0,
                        play_time_minutes: 0,
                    },
                    false,
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

        let action = match self.state.screen {
            GameScreen::CopyrightSplash | GameScreen::TitleScreen => {
                let any_pressed = input.any_just_pressed();
                self.title_screen.update_frame(any_pressed)
            }
            GameScreen::MainMenu => {
                let menu_input = MenuInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    a: input.is_just_pressed(GbButton::A) || input.is_just_pressed(GbButton::Start),
                    b: input.is_just_pressed(GbButton::B),
                };
                self.main_menu.update_frame(menu_input)
            }
            GameScreen::OakSpeech => {
                if self.oak_speech.is_naming_active() {
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
                    match self.oak_speech.update_naming_frame(naming_input) {
                        OakSpeechResult::PlayerNameSet(name) => {
                            self.player_name = name;
                            ScreenAction::Continue
                        }
                        OakSpeechResult::RivalNameSet(name) => {
                            self.rival_name = name;
                            ScreenAction::Continue
                        }
                        OakSpeechResult::Finished => {
                            ScreenAction::Transition(GameScreen::Overworld)
                        }
                        OakSpeechResult::Active => ScreenAction::Continue,
                    }
                } else {
                    let oak_input = OakSpeechInput {
                        up: input.is_just_pressed(GbButton::Up),
                        down: input.is_just_pressed(GbButton::Down),
                        a: input.is_just_pressed(GbButton::A),
                        b: input.is_just_pressed(GbButton::B),
                    };
                    match self.oak_speech.update_frame(oak_input) {
                        OakSpeechResult::PlayerNameSet(name) => {
                            self.player_name = name;
                            ScreenAction::Continue
                        }
                        OakSpeechResult::RivalNameSet(name) => {
                            self.rival_name = name;
                            ScreenAction::Continue
                        }
                        OakSpeechResult::Finished => {
                            ScreenAction::Transition(GameScreen::Overworld)
                        }
                        OakSpeechResult::Active => ScreenAction::Continue,
                    }
                }
            }
            GameScreen::Overworld => {
                let ow_input = OverworldInput {
                    up: input.is_just_pressed(GbButton::Up),
                    down: input.is_just_pressed(GbButton::Down),
                    left: input.is_just_pressed(GbButton::Left),
                    right: input.is_just_pressed(GbButton::Right),
                    a: input.is_just_pressed(GbButton::A),
                    b: input.is_just_pressed(GbButton::B),
                    start: input.is_just_pressed(GbButton::Start),
                    select: input.is_just_pressed(GbButton::Select),
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
                    SaveMenuResult::Saved | SaveMenuResult::Cancelled => {
                        ScreenAction::Transition(GameScreen::StartMenu)
                    }
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
                    self.state.screen == GameScreen::CopyrightSplash,
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
                draw_start_menu(&self.start_menu, &self.player_name, frame_buffer);
            }
            GameScreen::OptionsMenu => {
                draw_options_menu(&self.options_menu, frame_buffer);
            }
            GameScreen::SaveMenu => {
                draw_save_menu(&self.save_menu, frame_buffer);
            }
        }
    }

    fn should_exit(&self) -> bool {
        self.exit_requested
    }
}

fn blit_tileset(
    fb: &mut FrameBuffer,
    tileset: &TileSet,
    x: u32,
    y: u32,
    tiles_per_row: u32,
    palette: &Palette,
) {
    let total = tileset.len();
    for idx in 0..total {
        let tile = tileset.get(idx);
        let tx = (idx as u32) % tiles_per_row;
        let ty = (idx as u32) / tiles_per_row;
        let px = x + tx * TILE_SIZE;
        let py = y + ty * TILE_SIZE;
        for row in 0..TILE_SIZE {
            let rgba_row = tile.render_row(row as usize, palette);
            for col in 0..TILE_SIZE {
                let sx = px + col;
                let sy = py + row;
                if sx < SCREEN_WIDTH && sy < SCREEN_HEIGHT {
                    let c = rgba_row[col as usize];
                    if c != Rgba::TRANSPARENT {
                        fb.set_pixel(sx, sy, c);
                    }
                }
            }
        }
    }
}

fn draw_text_box(fb: &mut FrameBuffer, bx: u32, by: u32, bw: u32, bh: u32, color: Rgba) {
    let bg = Rgba::WHITE;
    let t = TILE_SIZE;

    draw_glyph(&box_tiles::TOP_LEFT, bx, by, color, bg, fb);
    for col in 0..bw {
        draw_glyph(
            &box_tiles::HORIZONTAL,
            bx + (1 + col) * t,
            by,
            color,
            bg,
            fb,
        );
    }
    draw_glyph(&box_tiles::TOP_RIGHT, bx + (1 + bw) * t, by, color, bg, fb);

    for row in 0..bh {
        let y = by + (1 + row) * t;
        draw_glyph(&box_tiles::VERTICAL_LEFT, bx, y, color, bg, fb);
        for col in 0..bw {
            fill_tile(bx + (1 + col) * t, y, bg, fb);
        }
        draw_glyph(
            &box_tiles::VERTICAL_RIGHT,
            bx + (1 + bw) * t,
            y,
            color,
            bg,
            fb,
        );
    }

    let bot_y = by + (1 + bh) * t;
    draw_glyph(&box_tiles::BOTTOM_LEFT, bx, bot_y, color, bg, fb);
    for col in 0..bw {
        draw_glyph(
            &box_tiles::HORIZONTAL_BOTTOM,
            bx + (1 + col) * t,
            bot_y,
            color,
            bg,
            fb,
        );
    }
    draw_glyph(
        &box_tiles::BOTTOM_RIGHT,
        bx + (1 + bw) * t,
        bot_y,
        color,
        bg,
        fb,
    );
}

fn draw_centered_sprite(
    fb: &mut FrameBuffer,
    tileset: &TileSet,
    sprite_w: u32,
    _sprite_h: u32,
    pal: &Palette,
) {
    let tiles_per_row = sprite_w / TILE_SIZE;
    let sx = (SCREEN_WIDTH.saturating_sub(sprite_w)) / 2;
    let sy = 32_u32;
    blit_tileset(fb, tileset, sx, sy, tiles_per_row, pal);
}

fn draw_title_screen(
    state: &TitleScreenState,
    is_copyright: bool,
    res: &mut Option<ResourceManager>,
    fb: &mut FrameBuffer,
) {
    let fade = state.fade_progress();
    let bg_color = if fade > 0.0 {
        let v = (255.0 * (1.0 - fade)) as u8;
        Rgba::rgb(v, v, v)
    } else {
        Rgba::WHITE
    };
    fb.clear(bg_color);

    let pal = &GRAYSCALE_PALETTE;

    if let Some(ref mut rm) = res {
        if is_copyright {
            if let Ok(copyright) = rm.load_splash("copyright") {
                let cw = copyright.source_size.0;
                let ch = copyright.source_size.1;
                let tiles_per_row = cw / TILE_SIZE;
                let cx = (SCREEN_WIDTH.saturating_sub(cw)) / 2;
                let cy = (SCREEN_HEIGHT.saturating_sub(ch)) / 2;
                blit_tileset(fb, &copyright.tileset.clone(), cx, cy, tiles_per_row, pal);
            }
            return;
        }

        if let Ok(logo) = rm.load_title("pokemon_logo") {
            let lw = logo.source_size.0;
            let tiles_per_row = lw / TILE_SIZE;
            let logo_ts = logo.tileset.clone();
            let lx = (SCREEN_WIDTH.saturating_sub(lw)) / 2;
            blit_tileset(fb, &logo_ts, lx, 2, tiles_per_row, pal);
        }

        if let Ok(ver) = rm.load_title("red_version") {
            let vw = ver.source_size.0;
            let tiles_per_row = vw / TILE_SIZE;
            let ver_ts = ver.tileset.clone();
            let vx = (SCREEN_WIDTH.saturating_sub(vw)) / 2;
            blit_tileset(fb, &ver_ts, vx, 58, tiles_per_row, pal);
        }

        if let Ok(player) = rm.load_title("player") {
            let pw = player.source_size.0;
            let tiles_per_row = pw / TILE_SIZE;
            let player_ts = player.tileset.clone();
            let player_x = SCREEN_WIDTH.saturating_sub(pw).saturating_sub(8);
            blit_tileset(fb, &player_ts, player_x, 68, tiles_per_row, pal);
        }
    } else {
        let phase_text = format!("Title Screen: {:?}", state.phase);
        draw_text(&phase_text, 10, 10, Rgba::BLACK, fb);
        draw_text("Press any button to continue", 10, 100, Rgba::BLACK, fb);
    }

    draw_text("Press START", 36, 130, Rgba::BLACK, fb);
}

fn draw_main_menu(state: &MainMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    // Original ASM (main_menu.asm):
    // No save: hlcoord 0,0 / b=4, c=13 / text at hlcoord 2,2 → "NEW GAME" + "OPTION"
    // With save: hlcoord 0,0 / b=6, c=13 / text at hlcoord 2,2 → "CONTINUE" + "NEW GAME" + "OPTION"
    let labels = state.item_labels();
    let num_items = labels.len();
    let box_h = if num_items > 2 { 6_u32 } else { 4_u32 };

    // TextBoxBorder at tile (0, 0) with inner size c=13, b=box_h
    draw_text_box(fb, 0, 0, 13, box_h, Rgba::BLACK);

    // Menu items at tile (2, 2), each 2 tiles apart
    for (i, label) in labels.iter().enumerate() {
        let tile_y = 2 + (i as u32 * 2);
        let px_x = 2 * TILE_SIZE;
        let px_y = tile_y * TILE_SIZE;
        draw_text(label, px_x, px_y, Rgba::BLACK, fb);
    }

    // Cursor arrow at tile (1, 2 + cursor*2)
    let cursor_tile_y = 2 + (state.cursor as u32 * 2);
    draw_text(
        "\x7F",
        1 * TILE_SIZE,
        cursor_tile_y * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
}

fn draw_oak_speech(
    state: &OakSpeechState,
    res: &mut Option<ResourceManager>,
    fb: &mut FrameBuffer,
) {
    fb.clear(Rgba::WHITE);
    let pal = &GRAYSCALE_PALETTE;

    // GB screen is 20×18 tiles. Text box: 18 inner tiles wide, 4 tall, at tile row 12 → pixel 96
    let text_box_x = 0_u32;
    let text_box_y = 12 * TILE_SIZE;
    let text_box_w = 18_u32;
    let text_box_h = 4_u32;

    if let Some(naming) = &state.naming_screen {
        draw_text("NAME ENTRY", 45, 30, Rgba::BLACK, fb);
        let name_text = format!("Name: {}_", naming.name());
        draw_text(&name_text, 30, 50, Rgba::BLACK, fb);
        let alphabet = naming.current_alphabet();
        for (row_i, row) in alphabet.iter().enumerate() {
            let y = 70 + (row_i as u32 * 12);
            let row_str: String = row.iter().map(|c| format!("{} ", c)).collect();
            draw_text(&row_str, 10, y, Rgba::BLACK, fb);
        }
        let cursor_marker = format!(
            "Cursor: row={} col={}",
            naming.cursor_row(),
            naming.cursor_col()
        );
        draw_text(&cursor_marker, 10, 134, Rgba::BLACK, fb);
        return;
    }

    match &state.phase {
        OakSpeechPhase::Intro { .. } => {
            if let Some(ref mut rm) = res {
                if let Ok(cached) = rm.load_trainer("prof.oak") {
                    let ts = cached.tileset.clone();
                    let w = cached.source_size.0;
                    let h = cached.source_size.1;
                    draw_centered_sprite(fb, &ts, w, h, pal);
                }
            }
            draw_text_box(
                fb,
                text_box_x,
                text_box_y,
                text_box_w,
                text_box_h,
                Rgba::BLACK,
            );
            if let Some(text) = state.current_intro_text() {
                draw_text(
                    text,
                    text_box_x + TILE_SIZE,
                    text_box_y + TILE_SIZE,
                    Rgba::BLACK,
                    fb,
                );
            }
        }
        OakSpeechPhase::ShowNidorino { .. } => {
            if let Some(ref mut rm) = res {
                if let Ok(cached) = rm.load_pokemon_front("nidorino") {
                    let ts = cached.tileset.clone();
                    let w = cached.source_size.0;
                    let h = cached.source_size.1;
                    draw_centered_sprite(fb, &ts, w, h, pal);
                }
            }
            draw_text_box(
                fb,
                text_box_x,
                text_box_y,
                text_box_w,
                text_box_h,
                Rgba::BLACK,
            );
            draw_text(
                "This world is",
                text_box_x + TILE_SIZE,
                text_box_y + TILE_SIZE,
                Rgba::BLACK,
                fb,
            );
            draw_text(
                "inhabited by POKeMON!",
                text_box_x + TILE_SIZE,
                text_box_y + TILE_SIZE * 3,
                Rgba::BLACK,
                fb,
            );
        }
        OakSpeechPhase::PlayerNameChoice { cursor } => {
            if let Some(ref mut rm) = res {
                if let Ok(cached) = rm.load(AssetCategory::Player, "red") {
                    let ts = cached.tileset.clone();
                    let w = cached.source_size.0;
                    let h = cached.source_size.1;
                    draw_centered_sprite(fb, &ts, w, h, pal);
                }
            }
            draw_text_box(
                fb,
                text_box_x,
                text_box_y,
                text_box_w,
                text_box_h,
                Rgba::BLACK,
            );
            draw_text(
                "Your name?",
                text_box_x + TILE_SIZE,
                text_box_y + TILE_SIZE,
                Rgba::BLACK,
                fb,
            );
            for (i, name) in DEFAULT_PLAYER_NAMES.iter().enumerate() {
                let prefix = if i == *cursor { ">" } else { " " };
                let label = format!("{}{}", prefix, name);
                draw_text(&label, 8, 8 + (i as u32 * 12), Rgba::BLACK, fb);
            }
        }
        OakSpeechPhase::PlayerNaming => {}
        OakSpeechPhase::RivalNameChoice { cursor } => {
            if let Some(ref mut rm) = res {
                if let Ok(cached) = rm.load_trainer("rival1") {
                    let ts = cached.tileset.clone();
                    let w = cached.source_size.0;
                    let h = cached.source_size.1;
                    draw_centered_sprite(fb, &ts, w, h, pal);
                }
            }
            draw_text_box(
                fb,
                text_box_x,
                text_box_y,
                text_box_w,
                text_box_h,
                Rgba::BLACK,
            );
            draw_text(
                "His name?",
                text_box_x + TILE_SIZE,
                text_box_y + TILE_SIZE,
                Rgba::BLACK,
                fb,
            );
            for (i, name) in DEFAULT_RIVAL_NAMES.iter().enumerate() {
                let prefix = if i == *cursor { ">" } else { " " };
                let label = format!("{}{}", prefix, name);
                draw_text(&label, 8, 8 + (i as u32 * 12), Rgba::BLACK, fb);
            }
        }
        OakSpeechPhase::RivalNaming => {}
        OakSpeechPhase::ShrinkPlayer { wait_frames } => {
            if let Some(ref mut rm) = res {
                let shrink_name = if *wait_frames > 30 {
                    "shrink1"
                } else {
                    "shrink2"
                };
                if let Ok(cached) = rm.load(AssetCategory::Player, shrink_name) {
                    let ts = cached.tileset.clone();
                    let w = cached.source_size.0;
                    let h = cached.source_size.1;
                    draw_centered_sprite(fb, &ts, w, h, pal);
                }
            }
        }
        OakSpeechPhase::Done => {
            draw_text("...", 70, 70, Rgba::BLACK, fb);
        }
    }
}

/// Blit a single tile from a tileset at the given screen position.
fn blit_single_tile(
    fb: &mut FrameBuffer,
    tileset: &TileSet,
    tile_idx: usize,
    px: u32,
    py: u32,
    palette: &Palette,
) {
    if tile_idx >= tileset.len() {
        return;
    }
    let tile = tileset.get(tile_idx);
    for row in 0..TILE_SIZE {
        let rgba_row = tile.render_row(row as usize, palette);
        for col in 0..TILE_SIZE {
            let sx = px + col;
            let sy = py + row;
            if sx < SCREEN_WIDTH && sy < SCREEN_HEIGHT {
                let c = rgba_row[col as usize];
                if c != Rgba::TRANSPARENT {
                    fb.set_pixel(sx, sy, c);
                }
            }
        }
    }
}

fn draw_overworld(
    screen: &OverworldScreen,
    res: &mut Option<ResourceManager>,
    fb: &mut FrameBuffer,
) {
    fb.clear(Rgba::WHITE);
    let pal = &GRAYSCALE_PALETTE;

    let player_tx = screen.state.player.x as i32;
    let player_ty = screen.state.player.y as i32;
    let screen_center_tx = 9_i32;
    let screen_center_ty = 8_i32;

    if let Some(ref mut rm) = res {
        if let Ok(cached) = rm.load_tileset("overworld") {
            let ts = cached.tileset.clone();

            let view_origin_tx = player_tx - screen_center_tx;
            let view_origin_ty = player_ty - screen_center_ty;

            for sy in 0..18_i32 {
                for sx in 0..20_i32 {
                    let world_tx = view_origin_tx + sx;
                    let world_ty = view_origin_ty + sy;
                    let tile_idx = demo_overworld_tile(world_tx, world_ty, ts.len());
                    let px = (sx as u32) * TILE_SIZE;
                    let py = (sy as u32) * TILE_SIZE;
                    blit_single_tile(fb, &ts, tile_idx, px, py, pal);
                }
            }
        }

        // Player sprite: 16×96 sheet = 6 frames of 16×16 (2×2 tiles each)
        // Frame order: down(0), down-walk(1), up(2), up-walk(3), left(4), left-walk(5)
        if let Ok(cached) = rm.load_sprite("red") {
            let ts = cached.tileset.clone();
            let frame = match screen.state.player.facing {
                Direction::Down => 0,
                Direction::Up => 2,
                Direction::Left => 4,
                Direction::Right => 4,
            };
            let base_tile = frame * 4;
            let tpr = cached.source_size.0 / TILE_SIZE;

            let player_px_x = screen_center_tx as u32 * TILE_SIZE;
            let player_px_y = screen_center_ty as u32 * TILE_SIZE;

            for row in 0..2_u32 {
                for col in 0..2_u32 {
                    let tile_idx = base_tile + (row as usize * tpr as usize) + col as usize;
                    if tile_idx < ts.len() {
                        blit_single_tile(
                            fb,
                            &ts,
                            tile_idx,
                            player_px_x + col * TILE_SIZE,
                            player_px_y + row * TILE_SIZE,
                            pal,
                        );
                    }
                }
            }
        }
    }

    let map_name = format!("{:?}", screen.state.current_map);
    let name_len = map_name.len() as u32;
    let box_w = name_len.max(4) + 2;
    let box_x = (SCREEN_WIDTH.saturating_sub((box_w + 2) * TILE_SIZE)) / 2;
    let box_y = 13 * TILE_SIZE;
    draw_text_box(fb, box_x, box_y, box_w, 2, Rgba::BLACK);
    draw_text(
        &map_name,
        box_x + TILE_SIZE,
        box_y + TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
}

fn demo_overworld_tile(world_x: i32, world_y: i32, max_tiles: usize) -> usize {
    let wx = world_x.rem_euclid(32) as u32;
    let wy = world_y.rem_euclid(32) as u32;

    let tile_idx = if wx == 0 || wx == 31 || wy == 0 || wy == 31 {
        51_usize
    } else if (wx == 15 || wx == 16) && wy > 0 && wy < 31 {
        7_usize
    } else if (wy == 15 || wy == 16) && wx > 0 && wx < 31 {
        7_usize
    } else if wx > 10 && wx < 20 && wy > 5 && wy < 10 {
        35_usize
    } else {
        1_usize
    };

    tile_idx.min(max_tiles.saturating_sub(1))
}

fn species_to_sprite_name(species_display: &str) -> String {
    species_display.to_lowercase()
}

fn hp_bar_color(hp: u16, max_hp: u16) -> Rgba {
    if max_hp == 0 {
        return Rgba::new(0, 0, 0, 255);
    }
    let ratio = (hp as u32 * 100) / max_hp as u32;
    if ratio > 50 {
        Rgba::new(0, 200, 0, 255)
    } else if ratio > 25 {
        Rgba::new(200, 200, 0, 255)
    } else {
        Rgba::new(200, 0, 0, 255)
    }
}

fn draw_hp_bar(fb: &mut FrameBuffer, x: u32, y: u32, width: u32, hp: u16, max_hp: u16) {
    let hp_bar_h = 4_u32;
    let filled_w = if max_hp > 0 {
        (hp as u32 * width) / max_hp as u32
    } else {
        0
    };
    let color = hp_bar_color(hp, max_hp);
    let bg = Rgba::new(60, 60, 60, 255);

    for dy in 0..hp_bar_h {
        for dx in 0..width {
            let px = x + dx;
            let py = y + dy;
            if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                let c = if dx < filled_w { color } else { bg };
                fb.set_pixel(px, py, c);
            }
        }
    }
}

fn draw_battle(screen: &BattleScreen, res: &mut Option<ResourceManager>, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);
    let pal = &GRAYSCALE_PALETTE;

    let enemy_name = format!("{}", screen.enemy_species).to_uppercase();
    let player_name = format!("{}", screen.player_species).to_uppercase();
    let enemy_sprite = species_to_sprite_name(&format!("{}", screen.enemy_species));
    let player_sprite = species_to_sprite_name(&format!("{}", screen.player_species));

    if let Some(ref mut rm) = res {
        if let Ok(cached) = rm.load_pokemon_front(&enemy_sprite) {
            let ts = cached.tileset.clone();
            let tpr = cached.source_size.0 / TILE_SIZE;
            blit_tileset(fb, &ts, 12 * TILE_SIZE, 0, tpr, pal);
        }

        if let Ok(cached) = rm.load_pokemon_back(&player_sprite) {
            let ts = cached.tileset.clone();
            let tpr = cached.source_size.0 / TILE_SIZE;
            blit_tileset(fb, &ts, 1 * TILE_SIZE, 5 * TILE_SIZE, tpr, pal);
        }
    }

    draw_text_box(fb, 0, 0, 12, 3, Rgba::BLACK);
    draw_text(&enemy_name, 1 * TILE_SIZE, 1 * TILE_SIZE, Rgba::BLACK, fb);
    let enemy_lv = format!("Lv{}", screen.enemy_level);
    draw_text(&enemy_lv, 8 * TILE_SIZE, 1 * TILE_SIZE, Rgba::BLACK, fb);
    draw_text("HP:", 1 * TILE_SIZE, 2 * TILE_SIZE, Rgba::BLACK, fb);
    draw_hp_bar(
        fb,
        4 * TILE_SIZE,
        2 * TILE_SIZE + 2,
        8 * TILE_SIZE,
        screen.enemy_hp,
        screen.enemy_max_hp,
    );

    draw_text_box(fb, 9 * TILE_SIZE, 7 * TILE_SIZE, 10, 4, Rgba::BLACK);
    draw_text(&player_name, 10 * TILE_SIZE, 8 * TILE_SIZE, Rgba::BLACK, fb);
    let player_lv = format!("Lv{}", screen.player_level);
    draw_text(&player_lv, 17 * TILE_SIZE, 8 * TILE_SIZE, Rgba::BLACK, fb);
    draw_text("HP:", 10 * TILE_SIZE, 9 * TILE_SIZE, Rgba::BLACK, fb);
    draw_hp_bar(
        fb,
        13 * TILE_SIZE,
        9 * TILE_SIZE + 2,
        6 * TILE_SIZE,
        screen.player_hp,
        screen.player_max_hp,
    );
    let hp_text = format!("{:>3}/{:>3}", screen.player_hp, screen.player_max_hp);
    draw_text(&hp_text, 12 * TILE_SIZE, 10 * TILE_SIZE, Rgba::BLACK, fb);

    draw_text_box(fb, 8 * TILE_SIZE, 12 * TILE_SIZE, 10, 4, Rgba::BLACK);

    if matches!(screen.phase, BattlePhase::PlayerMenu) {
        draw_text("FIGHT", 10 * TILE_SIZE, 14 * TILE_SIZE, Rgba::BLACK, fb);
        draw_text("PkMn", 16 * TILE_SIZE, 14 * TILE_SIZE, Rgba::BLACK, fb);
        draw_text("ITEM", 10 * TILE_SIZE, 16 * TILE_SIZE, Rgba::BLACK, fb);
        draw_text("RUN", 16 * TILE_SIZE, 16 * TILE_SIZE, Rgba::BLACK, fb);

        let menu_labels_pos = [(10, 14), (16, 14), (10, 16), (16, 16)];
        let sel = screen.battle_menu.row() * 2 + screen.battle_menu.col();
        if sel < menu_labels_pos.len() {
            let (cx, cy) = menu_labels_pos[sel];
            draw_text(">", (cx - 1) * TILE_SIZE, cy * TILE_SIZE, Rgba::BLACK, fb);
        }
    } else {
        draw_text_box(fb, 0, 12 * TILE_SIZE, 18, 4, Rgba::BLACK);
        let phase_text = match &screen.phase {
            BattlePhase::Intro { .. } => format!("Wild {} appeared!", enemy_name),
            BattlePhase::TurnExecution { .. } => format!("{} used SCRATCH!", player_name),
            BattlePhase::FaintCheck { .. } => format!("Enemy {} fainted!", enemy_name),
            BattlePhase::Finished { won, .. } => {
                if *won {
                    "You won!".to_string()
                } else {
                    "You lost...".to_string()
                }
            }
            _ => String::new(),
        };
        if !phase_text.is_empty() {
            draw_text(&phase_text, 1 * TILE_SIZE, 14 * TILE_SIZE, Rgba::BLACK, fb);
        }
    }
}

fn draw_start_menu(state: &StartMenuState, player_name: &str, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    // draw_start_menu.asm: hlcoord 10,0 / b=14(pokedex)/12(no pokedex), c=8
    // Items at hlcoord 12,2, spaced 2 tile rows. Cursor at X=11.
    let labels = state.item_labels(player_name);
    let box_h = (labels.len() as u32) * 2;

    draw_text_box(fb, 10 * TILE_SIZE, 0, 8, box_h, Rgba::BLACK);

    for (i, label) in labels.iter().enumerate() {
        let tile_y = 2 + (i as u32 * 2);
        draw_text(
            label.as_str(),
            12 * TILE_SIZE,
            tile_y * TILE_SIZE,
            Rgba::BLACK,
            fb,
        );
    }

    let cursor_tile_y = 2 + (state.cursor() as u32 * 2);
    draw_text(
        ">",
        11 * TILE_SIZE,
        cursor_tile_y * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
}

fn draw_options_menu(state: &OptionsMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    // DisplayOptionMenu: 3 boxes at rows 0/5/10, each b=3 c=18. CANCEL at tile (2,16).
    draw_text_box(fb, 0, 0 * TILE_SIZE, 18, 3, Rgba::BLACK);
    draw_text_box(fb, 0, 5 * TILE_SIZE, 18, 3, Rgba::BLACK);
    draw_text_box(fb, 0, 10 * TILE_SIZE, 18, 3, Rgba::BLACK);

    // Section 1: TEXT SPEED at tile (1,1), options at tile (1,2)
    draw_text("TEXT SPEED", 1 * TILE_SIZE, 1 * TILE_SIZE, Rgba::BLACK, fb);
    let speed_str = match state.options.text_speed {
        pokered_core::options_menu::TextSpeed::Fast => " FAST  MEDIUM SLOW",
        pokered_core::options_menu::TextSpeed::Medium => " FAST  MEDIUM SLOW",
        pokered_core::options_menu::TextSpeed::Slow => " FAST  MEDIUM SLOW",
    };
    draw_text(speed_str, 1 * TILE_SIZE, 3 * TILE_SIZE, Rgba::BLACK, fb);

    // Speed cursor: FAST at X=1, MEDIUM at X=7, SLOW at X=14
    let speed_cursor_x = match state.options.text_speed {
        pokered_core::options_menu::TextSpeed::Fast => 1,
        pokered_core::options_menu::TextSpeed::Medium => 7,
        pokered_core::options_menu::TextSpeed::Slow => 14,
    };
    draw_text(
        ">",
        speed_cursor_x * TILE_SIZE,
        3 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    // Section 2: BATTLE ANIMATION at tile (1,6), ON/OFF at tile (1,7)
    draw_text(
        "BATTLE ANIMATION",
        1 * TILE_SIZE,
        6 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    draw_text(
        " ON       OFF",
        1 * TILE_SIZE,
        8 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    let anim_cursor_x = match state.options.battle_animation {
        pokered_core::options_menu::BattleAnimation::On => 1,
        pokered_core::options_menu::BattleAnimation::Off => 10,
    };
    draw_text(
        ">",
        anim_cursor_x * TILE_SIZE,
        8 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    // Section 3: BATTLE STYLE at tile (1,11), SHIFT/SET at tile (1,12)
    draw_text(
        "BATTLE STYLE",
        1 * TILE_SIZE,
        11 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    draw_text(
        " SHIFT    SET",
        1 * TILE_SIZE,
        13 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    let style_cursor_x = match state.options.battle_style {
        pokered_core::options_menu::BattleStyle::Shift => 1,
        pokered_core::options_menu::BattleStyle::Set => 10,
    };
    draw_text(
        ">",
        style_cursor_x * TILE_SIZE,
        13 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    // CANCEL at tile (2, 16)
    draw_text("CANCEL", 2 * TILE_SIZE, 16 * TILE_SIZE, Rgba::BLACK, fb);
}

fn draw_save_menu(state: &SaveMenuState, fb: &mut FrameBuffer) {
    fb.clear(Rgba::WHITE);

    // PrintSaveScreenText: hlcoord 4,0 / b=8, c=14
    // Info at hlcoord 5,2: PLAYER / BADGES / #DEX / TIME
    draw_text_box(fb, 4 * TILE_SIZE, 0, 14, 8, Rgba::BLACK);

    draw_text("PLAYER", 5 * TILE_SIZE, 2 * TILE_SIZE, Rgba::BLACK, fb);
    draw_text(
        &state.info.player_name,
        12 * TILE_SIZE,
        2 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    draw_text("BADGES", 5 * TILE_SIZE, 4 * TILE_SIZE, Rgba::BLACK, fb);
    let badges = format!("{}", state.info.num_badges);
    draw_text(&badges, 17 * TILE_SIZE, 4 * TILE_SIZE, Rgba::BLACK, fb);

    draw_text("#DEX", 5 * TILE_SIZE, 6 * TILE_SIZE, Rgba::BLACK, fb);
    let dex = format!("{}", state.info.pokedex_owned);
    draw_text(&dex, 16 * TILE_SIZE, 6 * TILE_SIZE, Rgba::BLACK, fb);

    draw_text("TIME", 5 * TILE_SIZE, 8 * TILE_SIZE, Rgba::BLACK, fb);
    let time = format!(
        "{:>3}:{:02}",
        state.info.play_time_hours, state.info.play_time_minutes
    );
    draw_text(&time, 13 * TILE_SIZE, 8 * TILE_SIZE, Rgba::BLACK, fb);

    // "Would you like to SAVE?" dialog below
    draw_text_box(fb, 0, 11 * TILE_SIZE, 18, 4, Rgba::BLACK);
    draw_text(
        "Would you like to",
        1 * TILE_SIZE,
        12 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    draw_text(
        "SAVE the game?",
        1 * TILE_SIZE,
        14 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );

    // YES/NO cursor
    let yes_no_x = 15_u32;
    draw_text_box(fb, yes_no_x * TILE_SIZE, 7 * TILE_SIZE, 3, 2, Rgba::BLACK);
    draw_text(
        "YES",
        (yes_no_x + 1) * TILE_SIZE,
        8 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    draw_text(
        "NO",
        (yes_no_x + 1) * TILE_SIZE,
        9 * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
    let cursor_y = if state.cursor == pokered_core::save_menu::YesNoChoice::Yes {
        8
    } else {
        9
    };
    draw_text(
        ">",
        yes_no_x * TILE_SIZE,
        cursor_y * TILE_SIZE,
        Rgba::BLACK,
        fb,
    );
}

fn screen_target_to_game_screen(target: &ScreenTarget) -> GameScreen {
    match target {
        ScreenTarget::Copyright => GameScreen::CopyrightSplash,
        ScreenTarget::Title => GameScreen::TitleScreen,
        ScreenTarget::MainMenu => GameScreen::MainMenu,
        ScreenTarget::Oak => GameScreen::OakSpeech,
        ScreenTarget::Overworld => GameScreen::Overworld,
        ScreenTarget::Battle => GameScreen::Battle,
        ScreenTarget::StartMenu => GameScreen::StartMenu,
        ScreenTarget::Options => GameScreen::OptionsMenu,
        ScreenTarget::Save => GameScreen::SaveMenu,
    }
}

fn screen_name(screen: &GameScreen) -> &'static str {
    match screen {
        GameScreen::CopyrightSplash => "copyright",
        GameScreen::TitleScreen => "title",
        GameScreen::MainMenu => "main_menu",
        GameScreen::OakSpeech => "oak_speech",
        GameScreen::Overworld => "overworld",
        GameScreen::Battle => "battle",
        GameScreen::StartMenu => "start_menu",
        GameScreen::OptionsMenu => "options",
        GameScreen::SaveMenu => "save",
    }
}

const ALL_SCREENS: &[GameScreen] = &[
    GameScreen::CopyrightSplash,
    GameScreen::TitleScreen,
    GameScreen::MainMenu,
    GameScreen::OakSpeech,
    GameScreen::Overworld,
    GameScreen::Battle,
    GameScreen::StartMenu,
    GameScreen::OptionsMenu,
    GameScreen::SaveMenu,
];

fn capture_screen(game: &mut PokemonGame, target: GameScreen, frames: u32) -> FrameBuffer {
    game.handle_transition(target);
    let input = InputState::new();
    for _ in 0..frames {
        game.update(&input);
    }
    let mut fb = FrameBuffer::new(Rgba::WHITE);
    game.draw(&mut fb);
    fb
}

fn cmd_screenshot(target: &ScreenTarget, output: &PathBuf, frames: u32) {
    let version = GameVersion::Red;
    let mut game = PokemonGame::new(version);
    let screen = screen_target_to_game_screen(target);
    println!(
        "Capturing screen: {} ({} frames)...",
        screen_name(&screen),
        frames
    );
    let fb = capture_screen(&mut game, screen, frames);
    fb.save_png(output).expect("Failed to save PNG");
    println!("Saved: {}", output.display());
}

fn cmd_screenshot_all(output_dir: &PathBuf, frames: u32) {
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");
    let version = GameVersion::Red;
    let mut game = PokemonGame::new(version);
    for &screen in ALL_SCREENS {
        let name = screen_name(&screen);
        let path = output_dir.join(format!("{}.png", name));
        println!("Capturing: {}...", name);
        let fb = capture_screen(&mut game, screen, frames);
        fb.save_png(&path).expect("Failed to save PNG");
        println!("  -> {}", path.display());
    }
    println!(
        "Done. {} screenshots saved to {}",
        ALL_SCREENS.len(),
        output_dir.display()
    );
}

fn main() {
    let cli = Cli::parse();
    let version = GameVersion::Red;

    match cli.command {
        None | Some(Commands::Run) => {
            let config = GameWindowConfig {
                title: format!(
                    "Pokémon {} - Rust",
                    match version {
                        GameVersion::Red => "Red",
                        GameVersion::Blue => "Blue",
                    }
                ),
                scale: 3,
                resizable: true,
            };
            let game = PokemonGame::new(version);
            match run(config, game) {
                Ok(()) => println!("Game exited normally"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(Commands::Screenshot {
            ref screen,
            ref output,
            frames,
        }) => {
            cmd_screenshot(screen, output, frames);
        }
        Some(Commands::ScreenshotAll {
            ref output_dir,
            frames,
        }) => {
            cmd_screenshot_all(output_dir, frames);
        }
    }
}
