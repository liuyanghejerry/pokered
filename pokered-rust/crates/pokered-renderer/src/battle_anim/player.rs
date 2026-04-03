use crate::sprite::SpriteOamEntry;

use super::*;

// ─── Animation player ────────────────────────────────────────────────

/// State of the subanimation playback within a single AnimCommand::SubAnim.
#[derive(Debug, Clone)]
struct SubAnimState {
    /// The SubAnimation being played.
    subanim_id: u8,
    /// Resolved transform type (after Enemy resolution).
    transform: SubAnimTransform,
    /// Current frame index within the subanimation.
    frame_index: usize,
    /// Total number of frames.
    num_frames: usize,
    /// Per-frame delay in ticks.
    delay: u8,
    /// Current delay countdown.
    delay_counter: u8,
    /// Tileset index (0 or 1) for this subanimation.
    tileset: u8,
}

/// Plays a move animation, stepping through its command sequence.
///
/// The player is a state machine:
///   1. `start(move_id)` loads the move's command list.
///   2. `tick()` advances one step and returns what happened.
///   3. After each tick, `oam_entries()` has the current sprite data.
///
/// The caller is responsible for:
///   - Rendering OAM entries to the screen each frame.
///   - Applying `AnimEffect` actions (palette changes, screen shake, etc.).
///   - Counting down `WaitDelay` frames before calling tick again.
#[derive(Debug, Clone)]
pub struct AnimationPlayer {
    /// The move animation ID (1-based, matching wAnimationID).
    move_id: usize,
    /// Whether the player's mon is the attacker (affects Enemy transform).
    player_is_attacker: bool,
    /// Index into the current move animation's command list.
    command_index: usize,
    /// Total number of commands in this move animation.
    num_commands: usize,
    /// Current subanimation playback state (Some while playing a SubAnim command).
    subanim_state: Option<SubAnimState>,
    /// Accumulated OAM entries for the current frame.
    oam_buffer: Vec<SpriteOamEntry>,
    /// Whether the animation is finished.
    finished: bool,
}

impl AnimationPlayer {
    /// Create a new idle animation player.
    pub fn new() -> Self {
        Self {
            move_id: 0,
            player_is_attacker: true,
            command_index: 0,
            num_commands: 0,
            subanim_state: None,
            oam_buffer: Vec::with_capacity(40),
            finished: true,
        }
    }

    /// Start playing the animation for the given move.
    /// `move_id` is 0-based index into MOVE_ANIM_DATA.
    /// `player_is_attacker` determines Enemy transform resolution.
    pub fn start(&mut self, move_id: usize, player_is_attacker: bool) {
        self.move_id = move_id;
        self.player_is_attacker = player_is_attacker;
        self.command_index = 0;
        self.subanim_state = None;
        self.oam_buffer.clear();
        self.finished = false;

        if move_id < NUM_MOVE_ANIMS {
            self.num_commands = MOVE_ANIM_DATA[move_id].len();
        } else {
            self.num_commands = 0;
            self.finished = true;
        }
    }

    /// Whether the animation has finished playing.
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Get the current OAM buffer for rendering.
    pub fn oam_entries(&self) -> &[SpriteOamEntry] {
        &self.oam_buffer
    }

    /// Current move-animation tileset index (0/1/2) while a subanimation is active.
    pub fn current_tileset(&self) -> Option<u8> {
        self.subanim_state.as_ref().map(|s| s.tileset)
    }

    /// Decode a raw command tuple from MOVE_ANIM_DATA into an AnimCommand.
    pub fn decode_command(raw: &(u8, u8, u8, u8)) -> AnimCommand {
        let (kind, sound_val, id_val, packed) = *raw;
        if kind == 0 {
            // SubAnim command
            AnimCommand::SubAnim {
                sound_id: sound_val,
                subanim_id: id_val,
                tileset: packed >> 6,
                delay: packed & 0x3F,
            }
        } else {
            // Effect command
            AnimCommand::Effect {
                sound_id: sound_val,
                effect: SpecialEffect::from_u8(id_val).unwrap_or(SpecialEffect::WavyScreen),
            }
        }
    }

    /// Resolve the effective transform for a subanimation, accounting for
    /// Enemy type and whose turn it is.
    pub(crate) fn resolve_transform(&self, raw_transform: SubAnimTransform) -> SubAnimTransform {
        match raw_transform {
            SubAnimTransform::Enemy => {
                if self.player_is_attacker {
                    // Player's turn + Enemy type → HFlip
                    SubAnimTransform::HFlip
                } else {
                    // Enemy's turn + Enemy type → Normal
                    SubAnimTransform::Normal
                }
            }
            other => {
                // For non-Enemy types:
                // If it's the player's turn, use Normal (override).
                // If it's the enemy's turn, use the specified type.
                // (from GetSubanimationTransform1)
                if self.player_is_attacker {
                    SubAnimTransform::Normal
                } else {
                    other
                }
            }
        }
    }
}

// ─── Frame block rendering ──────────────────────────────────────────

impl AnimationPlayer {
    /// Render a frame block into OAM entries, applying the given transform.
    ///
    /// `frame_block_id`: index into FRAME_BLOCK_DATA.
    /// `base_coord_id`: index into BASE_COORDS.
    /// `transform`: the resolved SubAnimTransform to apply.
    /// `dest`: output buffer to append OAM entries to.
    pub fn render_frame_block(
        frame_block_id: usize,
        base_coord_id: usize,
        transform: SubAnimTransform,
        dest: &mut Vec<SpriteOamEntry>,
    ) {
        if frame_block_id >= NUM_FRAMEBLOCKS || base_coord_id >= NUM_BASECOORDS {
            return;
        }

        let fb_data = FRAME_BLOCK_DATA[frame_block_id];
        let (base_y, base_x) = BASE_COORDS[base_coord_id];

        for &(y_off, x_off, raw_tile, flags) in fb_data {
            let (screen_y, screen_x, tile_id, oam_flags) = match transform {
                SubAnimTransform::Normal | SubAnimTransform::Reverse => {
                    // No transformation — direct mapping.
                    // y = base_y + y_offset * 8
                    // x = base_x + x_offset * 8
                    let y = base_y as i32 + (y_off as i32) * 8;
                    let x = base_x as i32 + (x_off as i32) * 8;
                    let tile = raw_tile.wrapping_add(ANIM_BASE_TILE_ID);
                    (y, x, tile, flags)
                }

                SubAnimTransform::HvFlip => {
                    // Flip both H and V: mirror around (136, 168).
                    // y = 136 - (base_y + y_offset * 8)
                    // x = 168 - (base_x + x_offset * 8)
                    let y = 136i32 - (base_y as i32 + (y_off as i32) * 8);
                    let x = 168i32 - (base_x as i32 + (x_off as i32) * 8);
                    let tile = raw_tile.wrapping_add(ANIM_BASE_TILE_ID);
                    // Toggle flip flags: 0x00→0x60, 0x20→0x40, 0x40→0x20, 0x60→0x00
                    let new_flags = match flags & 0x60 {
                        0x00 => (flags & !0x60) | 0x60,
                        0x20 => (flags & !0x60) | 0x40,
                        0x40 => (flags & !0x60) | 0x20,
                        0x60 => flags & !0x60,
                        _ => flags, // unreachable, 0x60 mask covers all
                    };
                    (y, x, tile, new_flags)
                }

                SubAnimTransform::HFlip => {
                    // Flip horizontally + translate 40px down.
                    // y = base_y + y_offset * 8 + 40
                    // x = 168 - (base_x + x_offset * 8)
                    let y = base_y as i32 + (y_off as i32) * 8 + 40;
                    let x = 168i32 - (base_x as i32 + (x_off as i32) * 8);
                    let tile = raw_tile.wrapping_add(ANIM_BASE_TILE_ID);
                    // Toggle X flip bit only
                    let new_flags = flags ^ OAM_XFLIP;
                    (y, x, tile, new_flags)
                }

                SubAnimTransform::CoordFlip => {
                    // Flip base coordinates, keep offsets normal.
                    // y = (136 - base_y) + y_offset * 8
                    // x = (168 - base_x) + x_offset * 8
                    let y = (136i32 - base_y as i32) + (y_off as i32) * 8;
                    let x = (168i32 - base_x as i32) + (x_off as i32) * 8;
                    let tile = raw_tile.wrapping_add(ANIM_BASE_TILE_ID);
                    (y, x, tile, flags)
                }

                SubAnimTransform::Enemy => {
                    // Should have been resolved before calling. Fall back to Normal.
                    let y = base_y as i32 + (y_off as i32) * 8;
                    let x = base_x as i32 + (x_off as i32) * 8;
                    let tile = raw_tile.wrapping_add(ANIM_BASE_TILE_ID);
                    (y, x, tile, flags)
                }
            };

            dest.push(SpriteOamEntry::new(screen_y, screen_x, tile_id, oam_flags));
        }
    }
}

// ─── Animation tick (state machine) ─────────────────────────────────

impl AnimationPlayer {
    /// Advance the animation by one step.
    ///
    /// Returns what happened this tick. The caller should:
    ///   - `Playing` → render `oam_entries()` to screen.
    ///   - `WaitDelay(n)` → wait n frames, then call tick again.
    ///   - `Effect(e)` → apply the special effect, then call tick again.
    ///   - `Done` → animation is finished.
    pub fn tick(&mut self) -> AnimTickResult {
        if self.finished {
            return AnimTickResult::Done;
        }

        // If we're in the middle of playing a subanimation, advance it.
        if let Some(ref mut state) = self.subanim_state {
            // Check if delay countdown is active.
            if state.delay_counter > 0 {
                state.delay_counter -= 1;
                return AnimTickResult::WaitDelay(state.delay_counter);
            }

            // Render next frame of the subanimation.
            let subanim = get_subanimation(state.subanim_id as usize);

            if state.frame_index < state.num_frames {
                let frame_idx = if state.transform == SubAnimTransform::Reverse {
                    // Reverse: play frames from last to first.
                    state.num_frames - 1 - state.frame_index
                } else {
                    state.frame_index
                };

                let frame = &subanim.frames[frame_idx];
                let mode = frame.mode;

                // Determine whether to clear OAM before drawing.
                if mode.cleans_oam() {
                    self.oam_buffer.clear();
                }

                // Render the frame block tiles.
                Self::render_frame_block(
                    frame.frame_block_id as usize,
                    frame.base_coord_id as usize,
                    state.transform,
                    &mut self.oam_buffer,
                );

                state.frame_index += 1;

                // Apply delay based on mode.
                if mode.has_delay() && state.delay > 0 {
                    state.delay_counter = state.delay - 1;
                    return AnimTickResult::WaitDelay(state.delay);
                }

                // Mode02: no delay, keep OAM, advance — immediately process next frame.
                return AnimTickResult::Playing;
            }

            // Subanimation finished — clear state and advance to next command.
            self.subanim_state = None;
            self.oam_buffer.clear();
        }

        // Process next command in the move animation.
        if self.command_index >= self.num_commands {
            self.finished = true;
            return AnimTickResult::Done;
        }

        let raw = &MOVE_ANIM_DATA[self.move_id][self.command_index];
        self.command_index += 1;
        let cmd = Self::decode_command(raw);

        match cmd {
            AnimCommand::SubAnim {
                sound_id: _,
                subanim_id,
                tileset,
                delay,
            } => {
                let subanim = get_subanimation(subanim_id as usize);
                let raw_transform = subanim.transform;
                let resolved = self.resolve_transform(raw_transform);
                let num_frames = subanim.frames.len();

                self.subanim_state = Some(SubAnimState {
                    subanim_id,
                    transform: resolved,
                    frame_index: 0,
                    num_frames,
                    delay,
                    delay_counter: 0,
                    tileset,
                });

                // Immediately start rendering the first frame.
                self.tick()
            }

            AnimCommand::Effect {
                sound_id: _,
                effect,
            } => AnimTickResult::Effect(effect),
        }
    }
}

// ─── Special effect mapping ─────────────────────────────────────────

impl AnimationPlayer {
    /// Map a SpecialEffect to a high-level AnimEffect the caller should apply.
    ///
    /// This translates the original ASM effect handlers into abstract operations.
    /// The caller's rendering layer is responsible for executing these.
    pub fn apply_effect(effect: SpecialEffect) -> AnimEffect {
        match effect {
            SpecialEffect::WavyScreen => AnimEffect::WavyScreen,
            SpecialEffect::SubstituteMon => AnimEffect::SubstituteMon,
            SpecialEffect::ShakeBackAndForth => AnimEffect::ShakeBackAndForth,
            SpecialEffect::SlideEnemyMonOff => AnimEffect::SlideEnemyMonOff,
            SpecialEffect::ShowEnemyMonPic => AnimEffect::ShowEnemyMon,
            SpecialEffect::ShowMonPic => AnimEffect::ShowPlayerMon,
            SpecialEffect::BlinkEnemyMon => AnimEffect::BlinkEnemyMon { times: 6 },
            SpecialEffect::HideEnemyMonPic => AnimEffect::HideEnemyMon,
            SpecialEffect::FlashEnemyMonPic => AnimEffect::FlashEnemyMonPic,
            SpecialEffect::DelayAnimation10 => AnimEffect::Delay10,
            SpecialEffect::SpiralBallsInward => AnimEffect::SpiralBallsInward,
            SpecialEffect::ShakeEnemyHud2 => AnimEffect::ShakeEnemyHud { variant: 2 },
            SpecialEffect::ShakeEnemyHud => AnimEffect::ShakeEnemyHud { variant: 1 },
            SpecialEffect::SlideMonHalfOff => AnimEffect::SlidePlayerMonHalfOff,
            SpecialEffect::PetalsFalling => AnimEffect::PetalsFalling,
            SpecialEffect::LeavesFalling => AnimEffect::LeavesFalling,
            SpecialEffect::TransformMon => AnimEffect::TransformMon,
            SpecialEffect::SlideMonDownAndHide => AnimEffect::SlidePlayerMonDown,
            SpecialEffect::MinimizeMon => AnimEffect::MinimizeMon,
            SpecialEffect::BounceUpAndDown => AnimEffect::BounceUpAndDown,
            SpecialEffect::ShootManyBallsUpward => AnimEffect::ShootBallsUpward { many: true },
            SpecialEffect::ShootBallsUpward => AnimEffect::ShootBallsUpward { many: false },
            SpecialEffect::SquishMonPic => AnimEffect::SquishMonPic,
            SpecialEffect::HideMonPic => AnimEffect::HidePlayerMon,
            SpecialEffect::LightScreenPalette => AnimEffect::LightScreenPalette,
            SpecialEffect::ResetMonPosition => AnimEffect::ResetPlayerMonPosition,
            SpecialEffect::MoveMonHorizontally => AnimEffect::MovePlayerMonH,
            SpecialEffect::BlinkMon => AnimEffect::BlinkPlayerMon { times: 6 },
            SpecialEffect::SlideMonOff => AnimEffect::SlidePlayerMonHalfOff,
            SpecialEffect::FlashMonPic => AnimEffect::FlashPlayerMonPic,
            SpecialEffect::SlideMonDown => AnimEffect::SlidePlayerMonDown,
            SpecialEffect::SlideMonUp => AnimEffect::SlidePlayerMonUp,
            SpecialEffect::FlashScreenLong => AnimEffect::FlashScreen { frames: 16 },
            SpecialEffect::DarkenMonPalette => AnimEffect::DarkenMonPalette,
            SpecialEffect::WaterDropletsEverywhere => AnimEffect::WaterDroplets,
            SpecialEffect::ShakeScreen => AnimEffect::ShakeScreenH {
                pixels: 4,
                frames: 8,
            },
            SpecialEffect::ResetScreenPalette => AnimEffect::ResetScreenPalette,
            SpecialEffect::DarkScreenPalette => AnimEffect::DarkScreenPalette,
            SpecialEffect::DarkScreenFlash => AnimEffect::FlashScreen { frames: 4 },
        }
    }
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self::new()
    }
}
