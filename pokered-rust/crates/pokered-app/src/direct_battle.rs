use pokered_core::battle::state::{BattleType, Pokemon};
use pokered_core::battle::{BattleInput, BattleScreen};
use pokered_core::game_state::{GameScreen, ScreenAction};
use pokered_data::trainer_data::TrainerClass;
use pokered_renderer::input::{GbButton, InputState};
use pokered_renderer::resource::{AssetRoot, ResourceManager};
use pokered_renderer::window::GameLoop;
use pokered_renderer::{FrameBuffer, Rgba};

use crate::render::{draw_battle, BattleVisualEffects};

pub struct DirectBattleGame {
    pub battle: BattleScreen,
    pub resources: Option<ResourceManager>,
    pub battle_vfx: BattleVisualEffects,
    pub exit_requested: bool,
}

impl DirectBattleGame {
    pub fn new(
        battle_type: BattleType,
        player_party: Vec<Pokemon>,
        enemy_party: Vec<Pokemon>,
        trainer_class: Option<TrainerClass>,
    ) -> Self {
        let is_wild = battle_type == BattleType::Wild;
        let battle =
            BattleScreen::from_parties(is_wild, &player_party, &enemy_party, trainer_class);

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

        Self {
            battle,
            resources,
            battle_vfx: BattleVisualEffects::default(),
            exit_requested: false,
        }
    }
}

impl GameLoop for DirectBattleGame {
    fn update(&mut self, input: &InputState) {
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
        if let ScreenAction::Transition(GameScreen::Overworld) = action {
            self.exit_requested = true;
        }
    }

    fn draw(&mut self, frame_buffer: &mut FrameBuffer) {
        frame_buffer.clear(Rgba::WHITE);
        draw_battle(
            &self.battle,
            &mut self.resources,
            frame_buffer,
            &mut self.battle_vfx,
        );
    }

    fn should_exit(&self) -> bool {
        self.exit_requested
    }
}
