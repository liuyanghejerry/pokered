use pokered_audio::music_data::MusicId;
use pokered_audio::sfx_data::SfxId;
use pokered_core::battle::state::{BattleType, Pokemon};
use pokered_core::battle::{BattleInput, BattlePhase, BattleScreen};
use pokered_core::game_state::{GameScreen, ScreenAction};
use pokered_data::species::Species;
use pokered_data::trainer_data::TrainerClass;
use pokered_renderer::input::{GbButton, InputState};
use pokered_renderer::resource::{AssetRoot, ResourceManager};
use pokered_renderer::window::GameLoop;
use pokered_renderer::{FrameBuffer, Rgba};

use crate::audio::{species_to_cry, AudioOutput};
use crate::battle_sfx::move_to_sfx;
use crate::render::{draw_battle, BattleVisualEffects};

fn is_gym_leader_or_elite(tc: Option<TrainerClass>) -> bool {
    matches!(
        tc,
        Some(TrainerClass::Brock)
            | Some(TrainerClass::Misty)
            | Some(TrainerClass::LtSurge)
            | Some(TrainerClass::Erika)
            | Some(TrainerClass::Koga)
            | Some(TrainerClass::Blaine)
            | Some(TrainerClass::Sabrina)
            | Some(TrainerClass::Giovanni)
            | Some(TrainerClass::Lorelei)
            | Some(TrainerClass::Bruno)
            | Some(TrainerClass::Agatha)
            | Some(TrainerClass::Lance)
            | Some(TrainerClass::Rival3)
    )
}

fn phase_tag(phase: &BattlePhase) -> u8 {
    match phase {
        BattlePhase::Intro { .. } => 1,
        BattlePhase::PlayerMenu => 2,
        BattlePhase::MoveSelect => 3,
        BattlePhase::ShowingText { .. } => 4,
        BattlePhase::PartySelect => 5,
        BattlePhase::PartySubMenu { .. } => 9,
        BattlePhase::PartyStats { .. } => 10,
        BattlePhase::EnemySendingNext { .. } => 6,
        BattlePhase::PlayerFaintSwitch => 7,
        BattlePhase::BattleOver { .. } => 8,
    }
}

pub struct DirectBattleGame {
    pub battle: BattleScreen,
    pub resources: Option<ResourceManager>,
    pub battle_vfx: BattleVisualEffects,
    pub exit_requested: bool,

    #[cfg(not(target_arch = "wasm32"))]
    audio: Option<AudioOutput>,
    prev_message: Option<String>,
    prev_phase_tag: u8,
    music_started: bool,
    prev_enemy_species: Species,
    prev_player_species: Species,
    end_music_played: bool,
    cry_played_for_msg: bool,
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

        let player_species = battle.player_species;
        let enemy_species = battle.enemy_species;

        Self {
            battle,
            resources,
            battle_vfx: BattleVisualEffects::default(),
            exit_requested: false,
            #[cfg(not(target_arch = "wasm32"))]
            audio,
            prev_message: None,
            prev_phase_tag: 0,
            music_started: false,
            prev_enemy_species: enemy_species,
            prev_player_species: player_species,
            end_music_played: false,
            cry_played_for_msg: false,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn play_sfx(&self, id: SfxId) {
        if let Some(ref audio) = self.audio {
            audio.play_sfx(id);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn play_music(&self, id: MusicId) {
        if let Some(ref audio) = self.audio {
            audio.play_music(id);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn play_cry(&self, species: Species) {
        if species != Species::None {
            self.play_sfx(species_to_cry(species));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn update_audio(&mut self) {
        if let Some(ref audio) = self.audio {
            audio.update_frame();
        }

        let cur_phase_tag = phase_tag(&self.battle.phase);
        let phase_changed = cur_phase_tag != self.prev_phase_tag;
        let cur_message = self.battle.current_message.clone();
        let message_changed = cur_message != self.prev_message;

        if phase_changed {
            match self.battle.phase {
                BattlePhase::Intro { .. } => {
                    if !self.music_started {
                        if self.battle.is_wild {
                            self.play_music(MusicId::WILD_BATTLE);
                        } else if is_gym_leader_or_elite(self.battle.trainer_class) {
                            self.play_music(MusicId::GYM_LEADER_BATTLE);
                        } else {
                            self.play_music(MusicId::TRAINER_BATTLE);
                        }
                        self.music_started = true;
                    }
                }
                BattlePhase::EnemySendingNext { .. } => {
                    let new_species = self.battle.enemy_species;
                    if new_species != self.prev_enemy_species {
                        self.play_cry(new_species);
                        self.prev_enemy_species = new_species;
                    }
                }
                BattlePhase::BattleOver { won, .. } => {
                    if !self.end_music_played {
                        if won {
                            if self.battle.is_wild {
                                self.play_music(MusicId::DEFEATED_WILD_MON);
                            } else if is_gym_leader_or_elite(self.battle.trainer_class) {
                                self.play_music(MusicId::DEFEATED_GYM_LEADER);
                            } else {
                                self.play_music(MusicId::DEFEATED_TRAINER);
                            }
                        } else {
                            if let Some(ref audio) = self.audio {
                                audio.stop_music();
                            }
                        }
                        self.end_music_played = true;
                    }
                }
                _ => {}
            }
        }

        if message_changed {
            self.cry_played_for_msg = false;

            if let Some(ref msg) = cur_message {
                let msg_lower = msg.to_lowercase();

                if msg_lower.contains(" used ") {
                    if let Some(ref state) = self.battle.battle_state {
                        let player_name =
                            state.player.active_mon().species.to_string().to_lowercase();
                        let enemy_name =
                            state.enemy.active_mon().species.to_string().to_lowercase();

                        let move_id =
                            if !player_name.is_empty() && msg_lower.starts_with(&player_name) {
                                state.player.selected_move
                            } else if !enemy_name.is_empty() && msg_lower.starts_with(&enemy_name) {
                                state.enemy.selected_move
                            } else {
                                state.player.selected_move
                            };

                        self.play_sfx(move_to_sfx(move_id));
                    }
                } else if msg_lower.contains("super effective") {
                    self.play_sfx(SfxId::SuperEffective);
                } else if msg_lower.contains("not very effective") {
                    self.play_sfx(SfxId::NotVeryEffective);
                } else if msg_lower.ends_with("fainted!") {
                    self.play_sfx(SfxId::FaintFall);
                } else if msg.starts_with("Go! ") || msg.starts_with("Go, ") {
                    if !self.cry_played_for_msg {
                        self.play_cry(self.battle.player_species);
                        self.cry_played_for_msg = true;
                    }
                    self.prev_player_species = self.battle.player_species;
                } else if msg_lower.contains("come back") || msg_lower.contains("enough") {
                    self.play_sfx(SfxId::WithdrawDeposit);
                } else if msg_lower.contains("appeared!") {
                    if !self.cry_played_for_msg {
                        self.play_cry(self.battle.enemy_species);
                        self.cry_played_for_msg = true;
                    }
                } else if msg_lower.contains("wants to fight") {
                    if !self.cry_played_for_msg {
                        self.play_cry(self.battle.enemy_species);
                        self.cry_played_for_msg = true;
                    }
                } else if msg_lower.contains("critical hit") {
                    self.play_sfx(SfxId::Damage);
                }
            }
        }

        if self.battle.player_species != self.prev_player_species {
            self.prev_player_species = self.battle.player_species;
        }
        if self.battle.enemy_species != self.prev_enemy_species {
            self.prev_enemy_species = self.battle.enemy_species;
        }

        self.prev_phase_tag = cur_phase_tag;
        self.prev_message = cur_message;
    }
}

impl GameLoop for DirectBattleGame {
    fn update(&mut self, input: &InputState) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let tag = phase_tag(&self.battle.phase);
            if matches!(tag, 2 | 3 | 5 | 7) {
                if input.is_just_pressed(GbButton::A) {
                    self.play_sfx(SfxId::PressAB);
                }
                if input.is_just_pressed(GbButton::B) {
                    self.play_sfx(SfxId::PressAB);
                }
            }
        }

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

        #[cfg(not(target_arch = "wasm32"))]
        self.update_audio();

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
