//! Battle animation system — subanimations, frame blocks, and special effects.
//!
//! Faithful port of the original pokered battle animation engine:
//!   - `engine/battle/animations.asm` — PlayAnimation / PlaySubanimation / DrawFrameBlock
//!   - `data/moves/animations.asm` — per-move animation command sequences
//!   - `data/battle_anims/subanimations.asm` — 86 subanimation definitions
//!   - `data/battle_anims/frame_blocks.asm` — 122 frame block OAM tile layouts
//!   - `data/battle_anims/base_coords.asm` — 177 (Y,X) base coordinate pairs
//!
//! This is NOT a hardware OAM emulator. We model the animation data at a
//! higher level: each "OAM tile" becomes a positioned sprite tile in the
//! framebuffer, and special effects are translated to palette/scroll/sprite
//! operations on our `FrameBuffer` / `SpriteLayer` abstractions.

mod data;
mod player;
mod types;

pub use data::*;
pub use player::*;
pub use types::*;
