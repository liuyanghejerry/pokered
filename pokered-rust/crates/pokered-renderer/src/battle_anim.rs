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

use crate::sprite::SpriteOamEntry;

// ─── Constants ───────────────────────────────────────────────────────

/// Base tile ID added to frame block tile indices (original: $31).
pub const ANIM_BASE_TILE_ID: u8 = 0x31;

/// Number of subanimations (NUM_SUBANIMS = 86).
pub const NUM_SUBANIMS: usize = 86;

/// Number of frame blocks (NUM_FRAMEBLOCKS = 122, 0x00..0x79).
pub const NUM_FRAMEBLOCKS: usize = 122;

/// Number of base coordinates (NUM_BASECOORDS = 177, 0x00..0xB0).
pub const NUM_BASECOORDS: usize = 177;

/// First special-effect ID in the command stream.
/// IDs < this are subanimation commands; IDs >= this are special effects.
pub const FIRST_SE_ID: u8 = 0xD8;

/// Animation terminator byte.
pub const ANIM_END: u8 = 0xFF;

/// Flip constants matching OAM flags.
pub const OAM_XFLIP: u8 = 0x20;
pub const OAM_YFLIP: u8 = 0x40;

// ─── Subanimation transform types ────────────────────────────────────

/// How a subanimation's frame block tiles are transformed when rendered.
/// Maps to `SUBANIMTYPE_*` from `battle_anim_constants.asm`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SubAnimTransform {
    /// No transformation — use base coords + offsets directly.
    Normal = 0,
    /// Flip both horizontally and vertically (rotate 180°).
    HvFlip = 1,
    /// Flip horizontally and translate 40px downward (enemy perspective).
    HFlip = 2,
    /// Flip base coordinates (mirror around screen center).
    CoordFlip = 3,
    /// Play frames in reverse order.
    Reverse = 4,
    /// Enemy-side variant: choose transform based on wPlayerMonIsAttacker.
    Enemy = 5,
}

impl SubAnimTransform {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Normal,
            1 => Self::HvFlip,
            2 => Self::HFlip,
            3 => Self::CoordFlip,
            4 => Self::Reverse,
            5 => Self::Enemy,
            _ => Self::Normal,
        }
    }
}

// ─── Frame block modes ───────────────────────────────────────────────

/// Controls delay and OAM buffer behavior after rendering a frame block.
/// Maps to `FRAMEBLOCKMODE_*` from `battle_anim_constants.asm`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameBlockMode {
    /// Normal: delay, then clean OAM and reset dest address.
    Mode00 = 0,
    /// Unused in practice — same as Mode00.
    Mode01 = 1,
    /// No delay, keep OAM, advance dest address.
    Mode02 = 2,
    /// Delay, keep OAM, advance dest address.
    Mode03 = 3,
    /// Delay, keep OAM, do NOT advance dest address.
    Mode04 = 4,
}

impl FrameBlockMode {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Mode00,
            1 => Self::Mode01,
            2 => Self::Mode02,
            3 => Self::Mode03,
            4 => Self::Mode04,
            _ => Self::Mode00,
        }
    }

    /// Whether this mode delays (waits) after drawing.
    pub fn has_delay(&self) -> bool {
        matches!(
            self,
            Self::Mode00 | Self::Mode01 | Self::Mode03 | Self::Mode04
        )
    }

    /// Whether this mode cleans OAM after drawing.
    pub fn cleans_oam(&self) -> bool {
        matches!(self, Self::Mode00 | Self::Mode01)
    }

    /// Whether this mode advances the OAM destination pointer.
    pub fn advances_dest(&self) -> bool {
        matches!(self, Self::Mode02 | Self::Mode03)
    }
}

// ─── Animation type (screen-level effects) ───────────────────────────

/// Screen-level animation types from `AnimationTypePointerTable`.
/// These control per-subanimation-frame screen effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AnimationType {
    None = 0,
    ShakeScreenVertically = 1,
    ShakeScreenHorizontallyHeavy = 2,
    ShakeScreenHorizontallySlow = 3,
    BlinkEnemyMonSprite = 4,
    ShakeScreenHorizontallyLight = 5,
    ShakeScreenHorizontallySlow2 = 6,
}

impl AnimationType {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::ShakeScreenVertically,
            2 => Self::ShakeScreenHorizontallyHeavy,
            3 => Self::ShakeScreenHorizontallySlow,
            4 => Self::BlinkEnemyMonSprite,
            5 => Self::ShakeScreenHorizontallyLight,
            6 => Self::ShakeScreenHorizontallySlow2,
            _ => Self::None,
        }
    }
}

// ─── Special effects ─────────────────────────────────────────────────

/// Special effects that can appear in move animation command streams.
/// Values 0xD8..0xFE from `move_animation_constants.asm`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SpecialEffect {
    WavyScreen = 0xD8,
    SubstituteMon = 0xD9,
    ShakeBackAndForth = 0xDA,
    SlideEnemyMonOff = 0xDB,
    ShowEnemyMonPic = 0xDC,
    ShowMonPic = 0xDD,
    BlinkEnemyMon = 0xDE,
    HideEnemyMonPic = 0xDF,
    FlashEnemyMonPic = 0xE0,
    DelayAnimation10 = 0xE1,
    SpiralBallsInward = 0xE2,
    ShakeEnemyHud2 = 0xE3,
    ShakeEnemyHud = 0xE4,
    SlideMonHalfOff = 0xE5,
    PetalsFalling = 0xE6,
    LeavesFalling = 0xE7,
    TransformMon = 0xE8,
    SlideMonDownAndHide = 0xE9,
    MinimizeMon = 0xEA,
    BounceUpAndDown = 0xEB,
    ShootManyBallsUpward = 0xEC,
    ShootBallsUpward = 0xED,
    SquishMonPic = 0xEE,
    HideMonPic = 0xEF,
    LightScreenPalette = 0xF0,
    ResetMonPosition = 0xF1,
    MoveMonHorizontally = 0xF2,
    BlinkMon = 0xF3,
    SlideMonOff = 0xF4,
    FlashMonPic = 0xF5,
    SlideMonDown = 0xF6,
    SlideMonUp = 0xF7,
    FlashScreenLong = 0xF8,
    DarkenMonPalette = 0xF9,
    WaterDropletsEverywhere = 0xFA,
    ShakeScreen = 0xFB,
    ResetScreenPalette = 0xFC,
    DarkScreenPalette = 0xFD,
    DarkScreenFlash = 0xFE,
}

impl SpecialEffect {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0xD8 => Some(Self::WavyScreen),
            0xD9 => Some(Self::SubstituteMon),
            0xDA => Some(Self::ShakeBackAndForth),
            0xDB => Some(Self::SlideEnemyMonOff),
            0xDC => Some(Self::ShowEnemyMonPic),
            0xDD => Some(Self::ShowMonPic),
            0xDE => Some(Self::BlinkEnemyMon),
            0xDF => Some(Self::HideEnemyMonPic),
            0xE0 => Some(Self::FlashEnemyMonPic),
            0xE1 => Some(Self::DelayAnimation10),
            0xE2 => Some(Self::SpiralBallsInward),
            0xE3 => Some(Self::ShakeEnemyHud2),
            0xE4 => Some(Self::ShakeEnemyHud),
            0xE5 => Some(Self::SlideMonHalfOff),
            0xE6 => Some(Self::PetalsFalling),
            0xE7 => Some(Self::LeavesFalling),
            0xE8 => Some(Self::TransformMon),
            0xE9 => Some(Self::SlideMonDownAndHide),
            0xEA => Some(Self::MinimizeMon),
            0xEB => Some(Self::BounceUpAndDown),
            0xEC => Some(Self::ShootManyBallsUpward),
            0xED => Some(Self::ShootBallsUpward),
            0xEE => Some(Self::SquishMonPic),
            0xEF => Some(Self::HideMonPic),
            0xF0 => Some(Self::LightScreenPalette),
            0xF1 => Some(Self::ResetMonPosition),
            0xF2 => Some(Self::MoveMonHorizontally),
            0xF3 => Some(Self::BlinkMon),
            0xF4 => Some(Self::SlideMonOff),
            0xF5 => Some(Self::FlashMonPic),
            0xF6 => Some(Self::SlideMonDown),
            0xF7 => Some(Self::SlideMonUp),
            0xF8 => Some(Self::FlashScreenLong),
            0xF9 => Some(Self::DarkenMonPalette),
            0xFA => Some(Self::WaterDropletsEverywhere),
            0xFB => Some(Self::ShakeScreen),
            0xFC => Some(Self::ResetScreenPalette),
            0xFD => Some(Self::DarkScreenPalette),
            0xFE => Some(Self::DarkScreenFlash),
            _ => None,
        }
    }
}

// ─── Data structures ─────────────────────────────────────────────────

/// A single OAM tile entry within a frame block.
/// Encodes position offset, tile index, and flip/attribute flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameBlockTile {
    /// X position in tile units (multiplied by 8 for pixel offset).
    pub x_tile: u8,
    /// Y position in tile units (multiplied by 8 for pixel offset).
    pub y_tile: u8,
    /// Tile ID (before adding ANIM_BASE_TILE_ID).
    pub tile_id: u8,
    /// OAM attribute flags (OAM_XFLIP, OAM_YFLIP, etc.).
    pub flags: u8,
}

/// A frame block — a group of OAM tiles composing one animation "sprite".
/// Frame blocks are indexed 0x00..0x79 (NUM_FRAMEBLOCKS).
#[derive(Debug, Clone)]
pub struct FrameBlock {
    pub tiles: Vec<FrameBlockTile>,
}

impl FrameBlock {
    pub const fn empty() -> Self {
        Self { tiles: Vec::new() }
    }
}

/// A single frame within a subanimation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubAnimFrame {
    /// Frame block index (0..NUM_FRAMEBLOCKS).
    pub frame_block_id: u8,
    /// Base coordinate index (0..NUM_BASECOORDS).
    pub base_coord_id: u8,
    /// Frame block rendering mode.
    pub mode: FrameBlockMode,
}

/// A subanimation definition — specifies transform type and a sequence of frames.
#[derive(Debug, Clone)]
pub struct SubAnimation {
    pub transform: SubAnimTransform,
    pub frames: Vec<SubAnimFrame>,
}

/// A single command in a move's animation sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimCommand {
    /// Play a subanimation with the given parameters.
    /// `sound_id` is the move sound index (0 = no sound, stored as sound-1 in ROM).
    /// `subanim_id` is the subanimation table index.
    /// `tileset` selects animation tileset 0 or 1.
    /// `delay` is the per-frame delay in vblank frames.
    SubAnim {
        sound_id: u8,
        subanim_id: u8,
        tileset: u8,
        delay: u8,
    },
    /// Execute a special effect (screen flash, mon slide, etc.).
    /// `sound_id` is the move sound index (0 = no sound).
    Effect { sound_id: u8, effect: SpecialEffect },
}

/// A complete move animation — a sequence of commands.
#[derive(Debug, Clone)]
pub struct MoveAnimation {
    pub commands: Vec<AnimCommand>,
}

impl MoveAnimation {
    pub const fn empty() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

// ─── Base coordinate data ────────────────────────────────────────────
// From data/battle_anims/base_coords.asm — 177 (Y, X) pairs in pixels.

pub static BASE_COORDS: [(u8, u8); NUM_BASECOORDS] = [
    (0x10, 0x68), // 00
    (0x10, 0x70), // 01
    (0x10, 0x78), // 02
    (0x10, 0x80), // 03
    (0x10, 0x88), // 04
    (0x10, 0x90), // 05
    (0x10, 0x98), // 06
    (0x18, 0x68), // 07
    (0x18, 0x70), // 08
    (0x18, 0x78), // 09
    (0x34, 0x28), // 0A
    (0x18, 0x80), // 0B
    (0x18, 0x88), // 0C
    (0x18, 0x98), // 0D
    (0x20, 0x68), // 0E
    (0x20, 0x70), // 0F
    (0x20, 0x78), // 10
    (0x20, 0x80), // 11
    (0x20, 0x88), // 12
    (0x20, 0x90), // 13
    (0x20, 0x98), // 14
    (0x28, 0x68), // 15
    (0x28, 0x70), // 16
    (0x28, 0x78), // 17
    (0x28, 0x80), // 18
    (0x28, 0x88), // 19
    (0x30, 0x68), // 1A
    (0x30, 0x70), // 1B
    (0x30, 0x78), // 1C
    (0x30, 0x80), // 1D
    (0x30, 0x90), // 1E
    (0x30, 0x98), // 1F
    (0x38, 0x68), // 20
    (0x38, 0x78), // 21
    (0x38, 0x80), // 22
    (0x38, 0x88), // 23
    (0x40, 0x68), // 24
    (0x40, 0x70), // 25
    (0x40, 0x78), // 26
    (0x40, 0x80), // 27
    (0x40, 0x88), // 28
    (0x40, 0x98), // 29
    (0x10, 0x60), // 2A
    (0x18, 0x60), // 2B
    (0x20, 0x60), // 2C
    (0x28, 0x60), // 2D
    (0x30, 0x60), // 2E
    (0x40, 0x60), // 2F
    (0x58, 0x28), // 30
    (0x43, 0x38), // 31
    (0x33, 0x48), // 32
    (0x20, 0x58), // 33
    (0x32, 0x78), // 34
    (0x58, 0x58), // 35
    (0x2C, 0x6C), // 36
    (0x34, 0x80), // 37
    (0x48, 0x70), // 38
    (0x42, 0x36), // 39
    (0x38, 0x44), // 3A
    (0x40, 0x52), // 3B
    (0x48, 0x60), // 3C
    (0x3E, 0x6E), // 3D
    (0x28, 0x7C), // 3E
    (0x28, 0x8A), // 3F
    (0x50, 0x3C), // 40
    (0x48, 0x50), // 41
    (0x40, 0x64), // 42
    (0x38, 0x38), // 43
    (0x50, 0x30), // 44
    (0x50, 0x38), // 45
    (0x50, 0x40), // 46
    (0x50, 0x48), // 47
    (0x50, 0x50), // 48
    (0x48, 0x58), // 49
    (0x50, 0x44), // 4A
    (0x48, 0x48), // 4B
    (0x48, 0x4C), // 4C
    (0x40, 0x50), // 4D
    (0x40, 0x54), // 4E
    (0x38, 0x58), // 4F
    (0x38, 0x5C), // 50
    (0x30, 0x64), // 51
    (0x48, 0x40), // 52
    (0x48, 0x39), // 53
    (0x24, 0x88), // 54
    (0x24, 0x70), // 55
    (0x1C, 0x70), // 56
    (0x1C, 0x88), // 57
    (0x34, 0x68), // 58
    (0x34, 0x88), // 59
    (0x68, 0x50), // 5A
    (0x60, 0x50), // 5B
    (0x68, 0x60), // 5C
    (0x58, 0x50), // 5D
    (0x60, 0x60), // 5E
    (0x68, 0x40), // 5F
    (0x40, 0x40), // 60
    (0x38, 0x40), // 61
    (0x0B, 0x60), // 62
    (0x44, 0x48), // 63
    (0x40, 0x14), // 64
    (0x48, 0x1C), // 65
    (0x50, 0x24), // 66
    (0x4C, 0x24), // 67
    (0x10, 0x62), // 68
    (0x12, 0x62), // 69
    (0x12, 0x60), // 6A
    (0x20, 0x72), // 6B
    (0x22, 0x72), // 6C
    (0x22, 0x70), // 6D
    (0x28, 0x62), // 6E
    (0x50, 0x0A), // 6F
    (0x52, 0x0A), // 70
    (0x38, 0x30), // 71
    (0x40, 0x48), // 72
    (0x30, 0x48), // 73
    (0x40, 0x30), // 74
    (0x30, 0x40), // 75
    (0x38, 0x48), // 76
    (0x40, 0x4A), // 77
    (0x48, 0x4B), // 78
    (0x50, 0x4C), // 79
    (0x58, 0x4D), // 7A
    (0x60, 0x4D), // 7B
    (0x68, 0x4D), // 7C
    (0x38, 0x10), // 7D
    (0x50, 0x10), // 7E
    (0x38, 0x28), // 7F
    (0x48, 0x18), // 80
    (0x40, 0x20), // 81
    (0x48, 0x20), // 82
    (0x40, 0x3C), // 83
    (0x38, 0x50), // 84
    (0x28, 0x64), // 85
    (0x1C, 0x90), // 86
    (0x24, 0x80), // 87
    (0x2C, 0x70), // 88
    (0x30, 0x38), // 89
    (0x10, 0x50), // 8A
    (0x3C, 0x40), // 8B
    (0x40, 0x58), // 8C
    (0x30, 0x58), // 8D
    (0x58, 0x48), // 8E
    (0x50, 0x58), // 8F
    (0x48, 0x68), // 90
    (0x40, 0x18), // 91
    (0x28, 0x58), // 92
    (0x40, 0x38), // 93
    (0x48, 0x38), // 94
    (0x08, 0x70), // 95
    (0x44, 0x1C), // 96
    (0x3C, 0x58), // 97
    (0x38, 0x60), // 98
    (0x08, 0x60), // 99
    (0x38, 0x70), // 9A
    (0x38, 0x6C), // 9B
    (0x38, 0x64), // 9C
    (0x1C, 0x74), // 9D
    (0x2E, 0x74), // 9E
    (0x34, 0x50), // 9F
    (0x2F, 0x60), // A0
    (0x31, 0x70), // A1
    (0x4C, 0x30), // A2
    (0x3B, 0x40), // A3
    (0x2D, 0x50), // A4
    (0x26, 0x60), // A5
    (0x2D, 0x70), // A6
    (0x28, 0x50), // A7
    (0x1E, 0x60), // A8
    (0x29, 0x70), // A9
    (0x16, 0x60), // AA
    (0x14, 0x58), // AB
    (0x12, 0x54), // AC
    (0x14, 0x50), // AD
    (0x18, 0x4C), // AE
    (0x1C, 0x48), // AF
    (0x48, 0x28), // B0
];

// ─── Frame block data ────────────────────────────────────────────────

/// Frame block data — generated from data/battle_anims/frame_blocks.asm.
/// Each entry is a slice of (x_tile, y_tile, tile_id, flags).
pub fn get_frame_block(id: usize) -> FrameBlock {
    if id >= NUM_FRAMEBLOCKS {
        return FrameBlock::empty();
    }
    let data: &[(u8, u8, u8, u8)] = FRAME_BLOCK_DATA[id];
    FrameBlock {
        tiles: data
            .iter()
            .map(|&(x, y, t, f)| FrameBlockTile {
                x_tile: x,
                y_tile: y,
                tile_id: t,
                flags: f,
            })
            .collect(),
    }
}

pub(crate) static FRAME_BLOCK_DATA: [&[(u8, u8, u8, u8)]; 122] = [
    &[], // 00 FrameBlock00
    &[
        (0, 0, 0x2C, 0x00),
        (1, 0, 0x2D, 0x00),
        (2, 0, 0x2C, 0x20),
        (0, 1, 0x3C, 0x00),
        (1, 1, 0x3D, 0x00),
        (2, 1, 0x3C, 0x20),
        (0, 2, 0x2C, 0x40),
        (1, 2, 0x2D, 0x40),
        (2, 2, 0x2C, 0x60),
    ], // 01 FrameBlock01
    &[
        (0, 0, 0x20, 0x00),
        (1, 0, 0x21, 0x00),
        (2, 0, 0x21, 0x20),
        (3, 0, 0x20, 0x20),
        (0, 1, 0x30, 0x00),
        (1, 1, 0x31, 0x00),
        (2, 1, 0x31, 0x20),
        (3, 1, 0x30, 0x20),
        (0, 2, 0x30, 0x40),
        (1, 2, 0x31, 0x40),
        (2, 2, 0x31, 0x60),
        (3, 2, 0x30, 0x60),
        (0, 3, 0x20, 0x40),
        (1, 3, 0x21, 0x40),
        (2, 3, 0x21, 0x60),
        (3, 3, 0x20, 0x60),
    ], // 02 FrameBlock02
    &[
        (0, 0, 0x02, 0x00),
        (1, 0, 0x02, 0x20),
        (0, 1, 0x12, 0x00),
        (1, 1, 0x12, 0x20),
    ], // 03 FrameBlock03
    &[
        (0, 0, 0x06, 0x00),
        (1, 0, 0x07, 0x00),
        (0, 1, 0x16, 0x00),
        (1, 1, 0x17, 0x00),
    ], // 04 FrameBlock04
    &[
        (0, 0, 0x07, 0x20),
        (1, 0, 0x06, 0x20),
        (0, 1, 0x17, 0x20),
        (1, 1, 0x16, 0x20),
    ], // 05 FrameBlock05
    &[
        (1, 0, 0x23, 0x00),
        (0, 1, 0x32, 0x00),
        (1, 1, 0x33, 0x00),
        (2, 0, 0x23, 0x20),
        (2, 1, 0x33, 0x20),
        (3, 1, 0x32, 0x20),
        (0, 2, 0x32, 0x40),
        (1, 2, 0x33, 0x40),
        (1, 3, 0x23, 0x40),
        (2, 2, 0x33, 0x60),
        (3, 2, 0x32, 0x60),
        (2, 3, 0x23, 0x60),
    ], // 06 FrameBlock06
    &[
        (0, 0, 0x20, 0x00),
        (1, 0, 0x21, 0x00),
        (0, 1, 0x30, 0x00),
        (1, 1, 0x31, 0x00),
        (2, 0, 0x21, 0x20),
        (3, 0, 0x20, 0x20),
        (2, 1, 0x31, 0x20),
        (3, 1, 0x30, 0x20),
        (0, 2, 0x30, 0x40),
        (1, 2, 0x31, 0x40),
        (0, 3, 0x20, 0x40),
        (1, 3, 0x21, 0x40),
        (2, 2, 0x31, 0x60),
        (3, 2, 0x30, 0x60),
        (2, 3, 0x21, 0x60),
        (3, 3, 0x20, 0x60),
    ], // 07 FrameBlock07
    &[
        (0, 0, 0x20, 0x00),
        (1, 0, 0x21, 0x00),
        (0, 1, 0x30, 0x00),
        (1, 1, 0x31, 0x00),
        (3, 0, 0x21, 0x20),
        (4, 0, 0x20, 0x20),
        (3, 1, 0x31, 0x20),
        (4, 1, 0x30, 0x20),
        (0, 3, 0x30, 0x40),
        (1, 3, 0x31, 0x40),
        (0, 4, 0x20, 0x40),
        (1, 4, 0x21, 0x40),
        (3, 3, 0x31, 0x60),
        (4, 3, 0x30, 0x60),
        (3, 4, 0x21, 0x60),
        (4, 4, 0x20, 0x60),
    ], // 08 FrameBlock08
    &[
        (0, 0, 0x24, 0x00),
        (1, 0, 0x25, 0x00),
        (0, 1, 0x34, 0x00),
        (3, 0, 0x25, 0x20),
        (4, 0, 0x24, 0x20),
        (4, 1, 0x34, 0x20),
        (0, 3, 0x34, 0x40),
        (0, 4, 0x24, 0x40),
        (1, 4, 0x25, 0x40),
        (4, 3, 0x34, 0x60),
        (3, 4, 0x25, 0x60),
        (4, 4, 0x24, 0x60),
    ], // 09 FrameBlock09
    &[
        (0, 0, 0x24, 0x00),
        (1, 0, 0x25, 0x00),
        (0, 1, 0x34, 0x00),
        (4, 0, 0x25, 0x20),
        (5, 0, 0x24, 0x20),
        (5, 1, 0x34, 0x20),
        (0, 4, 0x34, 0x40),
        (0, 5, 0x24, 0x40),
        (1, 5, 0x25, 0x40),
        (5, 4, 0x34, 0x60),
        (4, 5, 0x25, 0x60),
        (5, 5, 0x24, 0x60),
    ], // 0A FrameBlock0a
    &[
        (0, 0, 0x05, 0x00),
        (1, 0, 0x05, 0x20),
        (0, 1, 0x15, 0x00),
        (1, 1, 0x15, 0x20),
    ], // 0B FrameBlock0b
    &[
        (0, 0, 0x04, 0x00),
        (1, 0, 0x04, 0x20),
        (0, 1, 0x14, 0x00),
        (1, 1, 0x14, 0x20),
    ], // 0C FrameBlock0c
    &[
        (0, 0, 0x0C, 0x00),
        (1, 0, 0x0D, 0x00),
        (0, 1, 0x1C, 0x00),
        (1, 1, 0x1D, 0x00),
        (0, 2, 0x1D, 0x60),
        (1, 2, 0x1C, 0x60),
        (0, 3, 0x0D, 0x60),
        (1, 3, 0x0C, 0x60),
    ], // 0D FrameBlock0d
    &[
        (0, 4, 0x0C, 0x00),
        (1, 4, 0x0D, 0x00),
        (0, 5, 0x1C, 0x00),
        (1, 5, 0x1D, 0x00),
    ], // 0E FrameBlock0e
    &[
        (0, 6, 0x1D, 0x60),
        (1, 6, 0x1C, 0x60),
        (0, 7, 0x0D, 0x60),
        (1, 7, 0x0C, 0x60),
    ], // 0F FrameBlock0f
    &[
        (0, 0, 0x0E, 0x00),
        (1, 0, 0x0F, 0x00),
        (0, 1, 0x1E, 0x00),
        (1, 1, 0x1F, 0x00),
        (2, 0, 0x0F, 0x20),
        (3, 0, 0x0E, 0x20),
        (2, 1, 0x1F, 0x20),
        (3, 1, 0x1E, 0x20),
    ], // 10 FrameBlock10
    &[
        (0, 0, 0x0E, 0x00),
        (1, 0, 0x0F, 0x00),
        (0, 1, 0x1E, 0x00),
        (1, 1, 0x1F, 0x00),
        (4, 0, 0x0F, 0x20),
        (5, 0, 0x0E, 0x20),
        (4, 1, 0x1F, 0x20),
        (5, 1, 0x1E, 0x20),
    ], // 11 FrameBlock11
    &[(0, 0, 0x37, 0x00), (2, 1, 0x37, 0x00), (4, 0, 0x37, 0x00)], // 12 FrameBlock12
    &[
        (0, 0, 0x36, 0x00),
        (1, 0, 0x36, 0x20),
        (0, 1, 0x36, 0x40),
        (1, 1, 0x36, 0x60),
    ], // 13 FrameBlock13
    &[
        (2, 0, 0x28, 0x00),
        (3, 0, 0x28, 0x20),
        (2, 1, 0x38, 0x00),
        (3, 1, 0x38, 0x20),
        (4, 0, 0x36, 0x00),
        (5, 0, 0x36, 0x20),
        (4, 1, 0x36, 0x40),
        (5, 1, 0x36, 0x60),
    ], // 14 FrameBlock14
    &[
        (0, 0, 0x28, 0x00),
        (1, 0, 0x28, 0x20),
        (0, 1, 0x38, 0x00),
        (1, 1, 0x38, 0x20),
        (2, 0, 0x29, 0x00),
        (3, 0, 0x29, 0x20),
        (2, 1, 0x39, 0x00),
        (3, 1, 0x39, 0x20),
        (4, 0, 0x28, 0x00),
        (5, 0, 0x28, 0x20),
        (4, 1, 0x38, 0x00),
        (5, 1, 0x38, 0x20),
    ], // 15 FrameBlock15
    &[
        (0, 0, 0x29, 0x00),
        (1, 0, 0x29, 0x20),
        (0, 1, 0x39, 0x00),
        (1, 1, 0x39, 0x20),
        (4, 0, 0x29, 0x00),
        (5, 0, 0x29, 0x20),
        (4, 1, 0x39, 0x00),
        (5, 1, 0x39, 0x20),
    ], // 16 FrameBlock16
    &[
        (0, 0, 0x08, 0x00),
        (1, 0, 0x09, 0x00),
        (0, 1, 0x18, 0x00),
        (1, 1, 0x19, 0x00),
    ], // 17 FrameBlock17
    &[(0, 3, 0x45, 0x60)], // 18 FrameBlock18
    &[(1, 3, 0x45, 0x00), (1, 2, 0x46, 0x60)], // 19 FrameBlock19
    &[(2, 2, 0x45, 0x60), (2, 3, 0x46, 0x00)], // 1A FrameBlock1a
    &[(3, 2, 0x45, 0x00), (3, 1, 0x46, 0x60)], // 1B FrameBlock1b
    &[(4, 1, 0x45, 0x60), (4, 2, 0x46, 0x00)], // 1C FrameBlock1c
    &[(5, 1, 0x45, 0x00), (5, 0, 0x46, 0x60)], // 1D FrameBlock1d
    &[(6, 0, 0x45, 0x60), (6, 1, 0x46, 0x00)], // 1E FrameBlock1e
    &[(0, 0, 0x03, 0x00), (6, 0, 0x03, 0x20)], // 1F FrameBlock1f
    &[
        (0, 0, 0x03, 0x00),
        (6, 0, 0x03, 0x20),
        (1, 1, 0x03, 0x00),
        (5, 1, 0x03, 0x20),
        (0, 1, 0x13, 0x00),
        (6, 1, 0x13, 0x20),
    ], // 20 FrameBlock20
    &[
        (0, 0, 0x03, 0x00),
        (6, 0, 0x03, 0x20),
        (1, 1, 0x03, 0x00),
        (5, 1, 0x03, 0x20),
        (0, 1, 0x13, 0x00),
        (6, 1, 0x13, 0x20),
        (2, 2, 0x03, 0x00),
        (4, 2, 0x03, 0x20),
        (1, 2, 0x13, 0x00),
        (5, 2, 0x13, 0x20),
        (0, 2, 0x03, 0x00),
        (6, 2, 0x03, 0x20),
    ], // 21 FrameBlock21
    &[
        (0, 0, 0x03, 0x00),
        (0, 1, 0x13, 0x00),
        (0, 2, 0x03, 0x00),
        (0, 3, 0x13, 0x00),
        (1, 1, 0x03, 0x00),
        (1, 2, 0x13, 0x00),
        (1, 3, 0x03, 0x00),
        (2, 2, 0x03, 0x00),
        (2, 3, 0x13, 0x00),
        (3, 3, 0x03, 0x00),
        (4, 2, 0x03, 0x20),
        (4, 3, 0x13, 0x20),
        (5, 1, 0x03, 0x20),
        (5, 2, 0x13, 0x20),
        (5, 3, 0x03, 0x20),
        (6, 0, 0x03, 0x20),
        (6, 1, 0x13, 0x20),
        (6, 2, 0x03, 0x20),
        (6, 3, 0x13, 0x20),
    ], // 22 FrameBlock22
    &[
        (0, 0, 0x0A, 0x00),
        (1, 0, 0x0B, 0x00),
        (0, 1, 0x1A, 0x00),
        (1, 1, 0x1B, 0x00),
    ], // 23 FrameBlock23
    &[(0, 1, 0x0A, 0x00), (1, 1, 0x0B, 0x00)], // 24 FrameBlock24
    &[
        (0, 2, 0x0A, 0x00),
        (1, 2, 0x0B, 0x00),
        (0, 3, 0x1A, 0x00),
        (1, 3, 0x1B, 0x00),
        (2, 0, 0x0A, 0x00),
        (3, 0, 0x0B, 0x00),
        (2, 1, 0x1A, 0x00),
        (3, 1, 0x1B, 0x00),
        (4, 1, 0x0A, 0x00),
        (5, 1, 0x0B, 0x00),
        (4, 2, 0x1A, 0x00),
        (5, 2, 0x1B, 0x00),
    ], // 25 FrameBlock25
    &[
        (2, 0, 0x44, 0x00),
        (3, 0, 0x44, 0x20),
        (2, 1, 0x44, 0x40),
        (3, 1, 0x44, 0x60),
    ], // 26 FrameBlock26
    &[
        (1, 1, 0x44, 0x00),
        (2, 1, 0x44, 0x20),
        (1, 2, 0x44, 0x40),
        (2, 2, 0x44, 0x60),
        (3, 0, 0x47, 0x00),
    ], // 27 FrameBlock27
    &[
        (0, 2, 0x44, 0x00),
        (1, 2, 0x44, 0x20),
        (0, 3, 0x44, 0x40),
        (1, 3, 0x44, 0x60),
        (2, 1, 0x47, 0x00),
        (2, 0, 0x47, 0x00),
    ], // 28 FrameBlock28
    &[
        (0, 3, 0x47, 0x00),
        (0, 2, 0x47, 0x00),
        (1, 1, 0x47, 0x00),
        (2, 0, 0x47, 0x00),
    ], // 29 FrameBlock29
    &[
        (0, 0, 0x44, 0x00),
        (1, 0, 0x44, 0x20),
        (0, 1, 0x44, 0x40),
        (1, 1, 0x44, 0x60),
    ], // 2A FrameBlock2a
    &[(0, 0, 0x47, 0x00), (1, 0, 0x47, 0x00)], // 2B FrameBlock2b
    &[(0, 20, 0x4D, 0x00)], // 2C FrameBlock2c
    &[
        (0, 0, 0x26, 0x00),
        (1, 0, 0x27, 0x00),
        (0, 1, 0x36, 0x00),
        (1, 1, 0x37, 0x00),
        (0, 2, 0x28, 0x00),
        (1, 2, 0x29, 0x00),
        (0, 3, 0x38, 0x00),
        (1, 3, 0x39, 0x00),
    ], // 2D FrameBlock2d
    &[
        (0, 0, 0x27, 0x20),
        (1, 0, 0x26, 0x20),
        (0, 1, 0x37, 0x20),
        (1, 1, 0x36, 0x20),
        (0, 2, 0x29, 0x20),
        (1, 2, 0x28, 0x20),
        (0, 3, 0x39, 0x20),
        (1, 3, 0x38, 0x20),
    ], // 2E FrameBlock2e
    &[
        (0, 0, 0x0C, 0x00),
        (1, 0, 0x0D, 0x00),
        (0, 1, 0x0C, 0x40),
        (1, 1, 0x0D, 0x40),
    ], // 2F FrameBlock2f
    &[
        (0, 0, 0x44, 0x00),
        (1, 0, 0x44, 0x20),
        (0, 1, 0x44, 0x40),
        (1, 1, 0x44, 0x60),
    ], // 30 FrameBlock30
    &[(0, 0, 0x45, 0x00)], // 31 FrameBlock31
    &[
        (0, 0, 0x4D, 0x00),
        (1, 0, 0x2F, 0x00),
        (2, 0, 0x4D, 0x20),
        (0, 1, 0x4E, 0x00),
        (1, 1, 0x07, 0x00),
        (2, 1, 0x4E, 0x20),
        (1, 2, 0x3F, 0x00),
    ], // 32 FrameBlock32
    &[
        (1, 0, 0x3F, 0x40),
        (0, 1, 0x4E, 0x40),
        (1, 1, 0x07, 0x40),
        (2, 1, 0x4E, 0x60),
        (0, 2, 0x4D, 0x40),
        (1, 2, 0x2F, 0x40),
        (2, 2, 0x4D, 0x60),
    ], // 33 FrameBlock33
    &[(0, 20, 0x00, 0x00)], // 34 FrameBlock34
    &[
        (0, 0, 0x2A, 0x00),
        (1, 0, 0x2B, 0x00),
        (0, 1, 0x3A, 0x00),
        (0, 2, 0x3A, 0x40),
        (0, 3, 0x2A, 0x40),
        (1, 3, 0x2B, 0x40),
    ], // 35 FrameBlock35
    &[
        (0, 0, 0x00, 0x00),
        (1, 0, 0x01, 0x00),
        (0, 1, 0x10, 0x00),
        (1, 1, 0x11, 0x00),
    ], // 36 FrameBlock36
    &[
        (0, 0, 0x01, 0x20),
        (1, 0, 0x00, 0x20),
        (0, 1, 0x11, 0x20),
        (1, 1, 0x10, 0x20),
    ], // 37 FrameBlock37
    &[
        (0, 0, 0x0A, 0x00),
        (1, 0, 0x0B, 0x00),
        (0, 1, 0x1A, 0x00),
        (1, 1, 0x1B, 0x00),
    ], // 38 FrameBlock38
    &[
        (0, 0, 0x0B, 0x20),
        (1, 0, 0x0A, 0x20),
        (0, 1, 0x1B, 0x20),
        (1, 1, 0x1A, 0x20),
    ], // 39 FrameBlock39
    &[
        (0, 4, 0x05, 0x00),
        (1, 4, 0x05, 0x20),
        (0, 5, 0x15, 0x00),
        (1, 5, 0x15, 0x20),
    ], // 3A FrameBlock3a
    &[
        (0, 3, 0x04, 0x00),
        (1, 3, 0x04, 0x20),
        (0, 4, 0x14, 0x00),
        (1, 4, 0x14, 0x20),
        (0, 5, 0x41, 0x00),
    ], // 3B FrameBlock3b
    &[
        (0, 2, 0x05, 0x00),
        (1, 2, 0x05, 0x20),
        (0, 3, 0x15, 0x00),
        (1, 3, 0x15, 0x20),
        (0, 4, 0x42, 0x00),
        (0, 5, 0x42, 0x00),
    ], // 3C FrameBlock3c
    &[
        (0, 1, 0x04, 0x00),
        (1, 1, 0x04, 0x20),
        (0, 2, 0x14, 0x00),
        (1, 2, 0x14, 0x20),
        (0, 3, 0x41, 0x00),
        (0, 4, 0x41, 0x00),
        (0, 5, 0x41, 0x00),
    ], // 3D FrameBlock3d
    &[
        (0, 0, 0x05, 0x00),
        (1, 0, 0x05, 0x20),
        (0, 1, 0x15, 0x00),
        (1, 1, 0x15, 0x20),
        (0, 2, 0x42, 0x00),
        (0, 3, 0x42, 0x00),
        (0, 4, 0x42, 0x00),
        (0, 5, 0x42, 0x00),
    ], // 3E FrameBlock3e
    &[
        (0, 0, 0x04, 0x00),
        (1, 0, 0x04, 0x20),
        (0, 1, 0x14, 0x00),
        (1, 1, 0x14, 0x20),
        (0, 2, 0x41, 0x00),
        (0, 3, 0x41, 0x00),
        (0, 4, 0x41, 0x00),
        (0, 5, 0x41, 0x00),
    ], // 3F FrameBlock3f
    &[(0, 0, 0x3D, 0x00), (1, 0, 0x3D, 0x00), (1, 1, 0x3D, 0x00)], // 40 FrameBlock40
    &[
        (0, 0, 0x06, 0x00),
        (1, 0, 0x06, 0x20),
        (0, 1, 0x16, 0x00),
        (1, 1, 0x17, 0x00),
    ], // 41 FrameBlock41
    &[
        (2, 0, 0x42, 0x00),
        (0, 1, 0x42, 0x00),
        (1, 1, 0x42, 0x00),
        (2, 1, 0x42, 0x00),
        (3, 1, 0x42, 0x00),
        (4, 1, 0x42, 0x00),
        (2, 2, 0x42, 0x00),
        (1, 3, 0x42, 0x00),
        (3, 3, 0x42, 0x00),
        (0, 4, 0x42, 0x00),
        (4, 4, 0x42, 0x00),
    ], // 42 FrameBlock42
    &[
        (2, 0, 0x41, 0x00),
        (0, 1, 0x41, 0x00),
        (1, 1, 0x41, 0x00),
        (2, 1, 0x41, 0x00),
        (3, 1, 0x41, 0x00),
        (4, 1, 0x41, 0x00),
        (2, 2, 0x41, 0x00),
        (1, 3, 0x41, 0x00),
        (3, 3, 0x41, 0x00),
        (0, 4, 0x41, 0x00),
        (4, 4, 0x41, 0x00),
    ], // 43 FrameBlock43
    &[
        (0, 0, 0x49, 0x00),
        (5, 0, 0x49, 0x00),
        (0, 5, 0x49, 0x00),
        (5, 5, 0x49, 0x00),
    ], // 44 FrameBlock44
    &[
        (0, 0, 0x49, 0x00),
        (3, 0, 0x49, 0x00),
        (0, 3, 0x49, 0x00),
        (3, 3, 0x49, 0x00),
    ], // 45 FrameBlock45
    &[
        (0, 0, 0x49, 0x00),
        (1, 0, 0x49, 0x00),
        (0, 1, 0x49, 0x00),
        (1, 1, 0x49, 0x00),
    ], // 46 FrameBlock46
    &[
        (0, 0, 0x43, 0x00),
        (1, 0, 0x43, 0x20),
        (0, 1, 0x43, 0x40),
        (1, 1, 0x43, 0x60),
    ], // 47 FrameBlock47
    &[
        (1, 1, 0x33, 0x00),
        (2, 1, 0x33, 0x20),
        (1, 2, 0x33, 0x40),
        (2, 2, 0x33, 0x60),
    ], // 48 SmallBlackCircleFrameBlock
    &[
        (0, 0, 0x22, 0x00),
        (1, 0, 0x23, 0x00),
        (2, 0, 0x23, 0x20),
        (3, 0, 0x22, 0x20),
        (0, 1, 0x32, 0x00),
        (1, 1, 0x43, 0x00),
        (2, 1, 0x43, 0x20),
        (3, 1, 0x32, 0x20),
        (0, 2, 0x32, 0x40),
        (1, 2, 0x43, 0x40),
        (2, 2, 0x43, 0x60),
        (3, 2, 0x32, 0x60),
        (0, 3, 0x22, 0x40),
        (1, 3, 0x23, 0x40),
        (2, 3, 0x23, 0x60),
        (3, 3, 0x22, 0x60),
    ], // 49 LargeBlackCircleFrameBlock
    &[
        (3, 1, 0x4C, 0x20),
        (1, 4, 0x4B, 0x00),
        (4, 6, 0x4C, 0x00),
        (6, 3, 0x4B, 0x40),
    ], // 4A FrameBlock4a
    &[
        (3, 0, 0x4C, 0x00),
        (0, 4, 0x4B, 0x40),
        (4, 7, 0x4C, 0x20),
        (7, 3, 0x4B, 0x00),
    ], // 4B FrameBlock4b
    &[
        (1, 2, 0x4A, 0x40),
        (2, 6, 0x4A, 0x00),
        (6, 5, 0x4A, 0x20),
        (5, 1, 0x4A, 0x60),
    ], // 4C FrameBlock4c
    &[
        (0, 1, 0x4A, 0x20),
        (1, 7, 0x4A, 0x60),
        (7, 6, 0x4A, 0x40),
        (6, 0, 0x4A, 0x00),
    ], // 4D FrameBlock4d
    &[
        (6, 0, 0x44, 0x00),
        (7, 0, 0x44, 0x20),
        (6, 1, 0x44, 0x40),
        (7, 1, 0x44, 0x60),
        (1, 4, 0x44, 0x00),
        (2, 4, 0x44, 0x20),
        (1, 5, 0x44, 0x40),
        (2, 5, 0x44, 0x60),
    ], // 4E FrameBlock4e
    &[
        (4, 1, 0x44, 0x00),
        (5, 1, 0x44, 0x20),
        (4, 2, 0x44, 0x40),
        (5, 2, 0x44, 0x60),
        (6, 0, 0x47, 0x00),
        (7, 0, 0x47, 0x00),
        (2, 3, 0x44, 0x00),
        (3, 3, 0x44, 0x20),
        (2, 4, 0x44, 0x40),
        (3, 4, 0x44, 0x60),
        (1, 6, 0x47, 0x00),
        (1, 5, 0x47, 0x00),
    ], // 4F FrameBlock4f
    &[
        (6, 0, 0x47, 0x00),
        (7, 0, 0x47, 0x00),
        (4, 2, 0x47, 0x00),
        (5, 1, 0x47, 0x00),
        (3, 3, 0x47, 0x00),
        (4, 3, 0x47, 0x00),
        (1, 5, 0x47, 0x00),
        (2, 4, 0x47, 0x00),
    ], // 50 FrameBlock50
    &[
        (0, 0, 0x35, 0x20),
        (0, 1, 0x35, 0x40),
        (0, 2, 0x35, 0x00),
        (0, 3, 0x35, 0x60),
        (8, 0, 0x35, 0x00),
        (8, 1, 0x35, 0x60),
        (8, 2, 0x35, 0x20),
        (8, 3, 0x35, 0x40),
    ], // 51 FrameBlock51
    &[
        (0, 0, 0x2A, 0x00),
        (1, 0, 0x2B, 0x00),
        (0, 1, 0x3A, 0x00),
        (1, 1, 0x3B, 0x00),
    ], // 52 FrameBlock52
    &[(0, 0, 0x3F, 0x00), (1, 0, 0x3F, 0x00), (0, 1, 0x3F, 0x00)], // 53 FrameBlock53
    &[
        (0, 0, 0x0E, 0x00),
        (1, 0, 0x0E, 0x20),
        (0, 1, 0x0F, 0x00),
        (1, 1, 0x0F, 0x20),
    ], // 54 FrameBlock54
    &[(0, 2, 0x2C, 0x00), (1, 2, 0x3C, 0x00), (2, 2, 0x2D, 0x00)], // 55 FrameBlock55
    &[
        (2, 2, 0x31, 0x00),
        (3, 2, 0x31, 0x00),
        (2, 1, 0x2C, 0x00),
        (3, 1, 0x3C, 0x00),
        (4, 1, 0x2D, 0x00),
        (4, 2, 0x2D, 0x00),
    ], // 56 FrameBlock56
    &[
        (4, 1, 0x31, 0x00),
        (4, 2, 0x31, 0x00),
        (5, 1, 0x31, 0x00),
        (5, 2, 0x31, 0x00),
        (4, 0, 0x2C, 0x00),
        (5, 0, 0x3C, 0x00),
        (6, 0, 0x2D, 0x00),
        (6, 1, 0x2D, 0x00),
        (6, 2, 0x2D, 0x00),
    ], // 57 FrameBlock57
    &[
        (0, 0, 0x46, 0x00),
        (0, 1, 0x47, 0x00),
        (0, 2, 0x48, 0x00),
        (0, 3, 0x48, 0x00),
        (0, 4, 0x48, 0x00),
        (0, 5, 0x48, 0x00),
        (0, 6, 0x48, 0x00),
    ], // 58 FrameBlock58
    &[(0, 0, 0x42, 0x00)], // 59 FrameBlock59
    &[
        (0, 0, 0x24, 0x00),
        (1, 0, 0x25, 0x00),
        (0, 1, 0x34, 0x00),
        (2, 0, 0x25, 0x20),
        (3, 0, 0x24, 0x20),
        (3, 1, 0x34, 0x20),
        (0, 2, 0x34, 0x40),
        (0, 3, 0x24, 0x40),
        (1, 3, 0x25, 0x40),
        (3, 2, 0x34, 0x60),
        (2, 3, 0x25, 0x60),
        (3, 3, 0x24, 0x60),
    ], // 5A FrameBlock5a
    &[
        (0, 0, 0x43, 0x00),
        (1, 0, 0x43, 0x20),
        (0, 1, 0x43, 0x40),
        (1, 1, 0x43, 0x60),
    ], // 5B FrameBlock5b
    &[
        (0, 0, 0x49, 0x00),
        (1, 0, 0x49, 0x00),
        (0, 3, 0x49, 0x00),
        (2, 2, 0x49, 0x00),
        (0, 1, 0x43, 0x00),
        (1, 1, 0x43, 0x20),
        (0, 2, 0x43, 0x40),
        (1, 2, 0x43, 0x60),
    ], // 5C FrameBlock5c
    &[
        (0, 0, 0x49, 0x00),
        (0, 3, 0x49, 0x00),
        (2, 2, 0x49, 0x00),
        (0, 1, 0x43, 0x00),
        (1, 0, 0x43, 0x20),
        (0, 2, 0x43, 0x40),
        (1, 2, 0x43, 0x60),
        (1, 0, 0x43, 0x00),
        (2, 0, 0x43, 0x20),
        (1, 1, 0x43, 0x40),
        (2, 1, 0x43, 0x60),
    ], // 5D FrameBlock5d
    &[
        (1, 0, 0x49, 0x00),
        (2, 1, 0x49, 0x00),
        (0, 4, 0x49, 0x00),
        (0, 1, 0x43, 0x00),
        (1, 1, 0x43, 0x20),
        (0, 2, 0x43, 0x40),
        (1, 2, 0x43, 0x60),
        (2, 2, 0x43, 0x00),
        (3, 2, 0x43, 0x20),
        (2, 3, 0x43, 0x40),
        (3, 3, 0x43, 0x60),
        (1, 4, 0x43, 0x00),
        (2, 4, 0x43, 0x20),
        (1, 5, 0x43, 0x40),
        (2, 5, 0x43, 0x60),
    ], // 5E FrameBlock5e
    &[
        (0, 0, 0x49, 0x00),
        (2, 0, 0x49, 0x00),
        (4, 0, 0x49, 0x00),
        (6, 0, 0x49, 0x00),
    ], // 5F FrameBlock5f
    &[
        (0, 0, 0x49, 0x00),
        (2, 0, 0x49, 0x00),
        (4, 0, 0x49, 0x00),
        (6, 0, 0x49, 0x00),
        (1, 1, 0x49, 0x00),
        (3, 1, 0x49, 0x00),
        (5, 1, 0x49, 0x00),
        (7, 1, 0x49, 0x00),
    ], // 60 FrameBlock60
    &[
        (0, 0, 0x49, 0x00),
        (2, 0, 0x49, 0x00),
        (4, 0, 0x49, 0x00),
        (6, 0, 0x49, 0x00),
        (1, 1, 0x49, 0x00),
        (3, 1, 0x49, 0x00),
        (5, 1, 0x49, 0x00),
        (7, 1, 0x49, 0x00),
        (0, 2, 0x49, 0x00),
        (2, 2, 0x49, 0x00),
        (4, 2, 0x49, 0x00),
        (6, 2, 0x49, 0x00),
    ], // 61 FrameBlock61
    &[
        (0, 0, 0x49, 0x00),
        (2, 0, 0x49, 0x00),
        (4, 0, 0x49, 0x00),
        (6, 0, 0x49, 0x00),
        (1, 1, 0x49, 0x00),
        (3, 1, 0x49, 0x00),
        (5, 1, 0x49, 0x00),
        (7, 1, 0x49, 0x00),
        (0, 2, 0x49, 0x00),
        (2, 2, 0x49, 0x00),
        (4, 2, 0x49, 0x00),
        (6, 2, 0x49, 0x00),
        (1, 3, 0x49, 0x00),
        (3, 3, 0x49, 0x00),
        (5, 3, 0x49, 0x00),
        (7, 3, 0x49, 0x00),
    ], // 62 FrameBlock62
    &[
        (0, 2, 0x26, 0x00),
        (1, 2, 0x27, 0x00),
        (2, 1, 0x26, 0x00),
        (3, 1, 0x27, 0x00),
        (4, 0, 0x26, 0x00),
        (5, 0, 0x27, 0x00),
    ], // 63 FrameBlock63
    &[
        (0, 3, 0x27, 0x00),
        (1, 2, 0x26, 0x00),
        (2, 2, 0x27, 0x00),
        (3, 1, 0x26, 0x00),
        (4, 1, 0x27, 0x00),
        (5, 0, 0x26, 0x00),
    ], // 64 FrameBlock64
    &[
        (0, 0, 0x1C, 0x00),
        (1, 0, 0x1D, 0x00),
        (0, 2, 0x1C, 0x00),
        (1, 2, 0x1D, 0x00),
        (0, 4, 0x1C, 0x00),
        (1, 4, 0x1D, 0x00),
    ], // 65 FrameBlock65
    &[(0, 0, 0x03, 0x00), (0, 1, 0x13, 0x00)], // 66 FrameBlock66
    &[(0, 0, 0x03, 0x00)], // 67 FrameBlock67
    &[
        (0, 0, 0x03, 0x00),
        (1, 0, 0x03, 0x20),
        (0, 1, 0x13, 0x00),
        (1, 1, 0x13, 0x20),
    ], // 68 FrameBlock68
    &[(0, 0, 0x06, 0x00)], // 69 FrameBlock69
    &[
        (0, 0, 0x2E, 0x00),
        (6, 0, 0x2E, 0x20),
        (0, 6, 0x2E, 0x40),
        (6, 6, 0x2E, 0x60),
        (3, 0, 0x2F, 0x00),
        (3, 6, 0x2F, 0x40),
        (0, 3, 0x3E, 0x00),
        (6, 3, 0x3E, 0x20),
    ], // 6A FrameBlock6a
    &[
        (0, 0, 0x2E, 0x00),
        (4, 0, 0x2E, 0x20),
        (0, 4, 0x2E, 0x40),
        (4, 4, 0x2E, 0x60),
        (2, 0, 0x2F, 0x00),
        (2, 4, 0x2F, 0x40),
        (0, 2, 0x3E, 0x00),
        (4, 2, 0x3E, 0x20),
    ], // 6B FrameBlock6b
    &[
        (0, 0, 0x2E, 0x00),
        (2, 0, 0x2E, 0x20),
        (0, 2, 0x2E, 0x40),
        (2, 2, 0x2E, 0x60),
        (1, 0, 0x2F, 0x00),
        (1, 2, 0x2F, 0x40),
        (0, 1, 0x3E, 0x00),
        (2, 1, 0x3E, 0x20),
    ], // 6C FrameBlock6c
    &[(0, 0, 0x1E, 0x00), (1, 0, 0x1F, 0x00)], // 6D FrameBlock6d
    &[
        (0, 0, 0x48, 0x00),
        (1, 0, 0x48, 0x20),
        (0, 1, 0x12, 0x00),
        (1, 1, 0x12, 0x20),
    ], // 6E FrameBlock6e
    &[
        (0, 0, 0x4A, 0x00),
        (1, 0, 0x07, 0x00),
        (0, 1, 0x16, 0x00),
        (1, 1, 0x17, 0x00),
    ], // 6F FrameBlock6f
    &[
        (0, 0, 0x07, 0x20),
        (1, 0, 0x4A, 0x20),
        (0, 1, 0x17, 0x20),
        (1, 1, 0x16, 0x20),
    ], // 70 FrameBlock70
    &[
        (0, 0, 0x22, 0x00),
        (1, 0, 0x3B, 0x00),
        (2, 0, 0x23, 0x20),
        (3, 0, 0x22, 0x20),
        (0, 1, 0x32, 0x00),
        (1, 1, 0x43, 0x00),
        (2, 1, 0x43, 0x20),
        (3, 1, 0x32, 0x20),
        (0, 2, 0x32, 0x40),
        (1, 2, 0x43, 0x40),
        (2, 2, 0x43, 0x60),
        (3, 2, 0x32, 0x60),
        (0, 3, 0x22, 0x40),
        (1, 3, 0x23, 0x40),
        (2, 3, 0x23, 0x60),
        (3, 3, 0x22, 0x60),
    ], // 71 FrameBlock71
    &[
        (0, 0, 0x32, 0x00),
        (1, 0, 0x43, 0x00),
        (2, 0, 0x43, 0x20),
        (3, 0, 0x32, 0x20),
        (0, 1, 0x32, 0x40),
        (1, 1, 0x43, 0x40),
        (2, 1, 0x43, 0x60),
        (3, 1, 0x32, 0x60),
        (0, 2, 0x22, 0x40),
        (1, 2, 0x23, 0x40),
        (2, 2, 0x23, 0x60),
        (3, 2, 0x22, 0x60),
    ], // 72 FrameBlock72
    &[
        (0, 0, 0x32, 0x40),
        (1, 0, 0x43, 0x40),
        (2, 0, 0x43, 0x60),
        (3, 0, 0x32, 0x60),
        (0, 1, 0x22, 0x40),
        (1, 1, 0x23, 0x40),
        (2, 1, 0x23, 0x60),
        (3, 1, 0x22, 0x60),
    ], // 73 FrameBlock73
    &[
        (0, 0, 0x22, 0x40),
        (1, 0, 0x23, 0x40),
        (2, 0, 0x23, 0x60),
        (3, 0, 0x22, 0x60),
    ], // 74 FrameBlock74
    &[
        (0, 0, 0x43, 0x00),
        (1, 0, 0x43, 0x20),
        (0, 1, 0x22, 0x00),
        (1, 1, 0x43, 0x60),
    ], // 75 FrameBlock75
    &[
        (2, 0, 0x2F, 0x00),
        (1, 0, 0x2F, 0x00),
        (3, 0, 0x2F, 0x00),
        (0, 0, 0x2E, 0x00),
        (4, 0, 0x2E, 0x20),
        (0, 1, 0x3E, 0x00),
        (4, 1, 0x3E, 0x20),
    ], // 76 FrameBlock76
    &[
        (0, 0, 0x4B, 0x00),
        (1, 0, 0x4C, 0x00),
        (0, 1, 0x4C, 0x60),
        (1, 1, 0x4B, 0x60),
    ], // 77 FrameBlock77
    &[(0, 0, 0x4D, 0x00)], // 78 FrameBlock78
    &[(0, 0, 0x4E, 0x00)], // 79 FrameBlock79
];

// ─── Subanimation data ───────────────────────────────────────────────

pub fn get_subanimation(id: usize) -> SubAnimation {
    if id >= NUM_SUBANIMS {
        return SubAnimation {
            transform: SubAnimTransform::Normal,
            frames: Vec::new(),
        };
    }
    let (transform, data) = SUBANIM_DATA[id];
    SubAnimation {
        transform: SubAnimTransform::from_u8(transform),
        frames: data
            .iter()
            .map(|&(fb, bc, m)| SubAnimFrame {
                frame_block_id: fb,
                base_coord_id: bc,
                mode: FrameBlockMode::from_u8(m),
            })
            .collect(),
    }
}

pub(crate) static SUBANIM_DATA: [(u8, &[(u8, u8, u8)]); 86] = [
    (2, &[(1, 23, 0)]),                                     // 00 Subanim_0Star
    (2, &[(1, 15, 0), (1, 29, 0)]),                         // 01 Subanim_0StarTwice
    (2, &[(1, 18, 0), (1, 21, 0), (1, 28, 0)]),             // 02 Subanim_0StarThrice
    (2, &[(1, 11, 0), (1, 17, 0), (1, 24, 0), (1, 29, 0)]), // 03 Subanim_0StarDescending
    (2, &[(2, 26, 0), (2, 16, 0), (2, 3, 0)]),              // 04 Subanim_1StarBigMoving
    (2, &[(2, 16, 0)]),                                     // 05 Subanim_1StarBig
    (
        0,
        &[
            (3, 48, 0),
            (3, 162, 0),
            (3, 147, 0),
            (3, 97, 0),
            (3, 115, 0),
            (3, 167, 0),
            (3, 51, 0),
            (3, 168, 0),
            (3, 14, 0),
            (3, 169, 0),
            (3, 52, 0),
        ],
    ), // 06 Subanim_0BallTossHigh
    (
        0,
        &[
            (3, 48, 0),
            (3, 162, 0),
            (3, 49, 0),
            (3, 163, 0),
            (3, 50, 0),
            (3, 164, 0),
            (3, 146, 0),
            (3, 165, 0),
            (3, 21, 0),
            (3, 166, 0),
            (3, 52, 0),
        ],
    ), // 07 Subanim_0BallTossMiddle
    (
        0,
        &[
            (3, 48, 0),
            (3, 68, 0),
            (3, 148, 0),
            (3, 96, 0),
            (3, 118, 0),
            (3, 159, 0),
            (3, 141, 0),
            (3, 160, 0),
            (3, 26, 0),
            (3, 161, 0),
            (3, 52, 0),
        ],
    ), // 08 Subanim_0BallTossLow
    (0, &[(3, 33, 4), (4, 33, 4), (3, 33, 4), (5, 33, 4)]), // 09 Subanim_0BallShakeEnemy
    (
        2,
        &[
            (6, 27, 0),
            (7, 27, 0),
            (8, 54, 0),
            (9, 54, 0),
            (10, 21, 0),
            (10, 21, 0),
        ],
    ), // 0A Subanim_0BallPoofEnemy
    (0, &[(1, 45, 0), (3, 47, 0), (3, 53, 0), (3, 77, 0)]), // 0B Subanim_0BallBlock
    (2, &[(12, 32, 0), (12, 33, 0), (12, 35, 0)]),          // 0C Subanim_1FlameColumn1
    (
        2,
        &[
            (12, 32, 2),
            (12, 21, 0),
            (12, 33, 2),
            (12, 23, 0),
            (12, 35, 2),
            (12, 25, 0),
        ],
    ), // 0D Subanim_1FlameColumn2
    (
        2,
        &[
            (12, 32, 2),
            (12, 21, 2),
            (12, 7, 0),
            (12, 33, 2),
            (12, 23, 2),
            (12, 9, 0),
            (12, 35, 2),
            (12, 25, 2),
            (12, 12, 0),
        ],
    ), // 0E Subanim_1FlameColumn3
    (
        2,
        &[
            (38, 14, 2),
            (38, 22, 2),
            (38, 28, 0),
            (39, 14, 2),
            (39, 22, 2),
            (39, 28, 0),
            (40, 14, 2),
            (40, 22, 2),
            (40, 28, 0),
            (41, 14, 2),
            (41, 22, 2),
            (41, 28, 0),
        ],
    ), // 0F Subanim_0Scratches
    (
        4,
        &[
            (45, 68, 0),
            (46, 69, 0),
            (45, 70, 0),
            (46, 71, 0),
            (45, 72, 0),
            (46, 73, 0),
            (45, 47, 0),
            (46, 26, 0),
        ],
    ), // 10 Subanim_1Tornado
    (
        2,
        &[
            (11, 38, 0),
            (12, 38, 0),
            (11, 38, 0),
            (12, 38, 0),
            (11, 40, 0),
            (12, 40, 0),
            (11, 40, 0),
            (12, 40, 0),
            (11, 39, 0),
            (12, 39, 0),
            (11, 39, 0),
            (12, 39, 0),
        ],
    ), // 11 Subanim_1Flames
    (
        3,
        &[
            (23, 48, 0),
            (23, 57, 0),
            (23, 58, 0),
            (23, 59, 0),
            (23, 60, 0),
            (23, 61, 0),
            (23, 62, 0),
            (23, 63, 0),
            (23, 31, 0),
        ],
    ), // 12 Subanim_0Heart_1Music
    (
        4,
        &[
            (48, 49, 0),
            (48, 50, 0),
            (48, 146, 0),
            (48, 14, 0),
            (48, 15, 0),
            (48, 16, 0),
        ],
    ), // 13 Subanim_1BlobToss
    (
        2,
        &[
            (48, 16, 0),
            (48, 16, 3),
            (49, 28, 4),
            (49, 33, 4),
            (49, 38, 0),
            (48, 16, 2),
            (49, 29, 4),
            (49, 34, 4),
            (49, 39, 0),
        ],
    ), // 14 Subanim_1BlobDripEnemy
    (1, &[(53, 82, 0), (53, 83, 0)]),                       // 15 Subanim_1Shout
    (
        2,
        &[
            (42, 5, 0),
            (43, 5, 2),
            (43, 12, 2),
            (42, 17, 4),
            (43, 17, 2),
            (43, 23, 2),
            (42, 27, 4),
            (43, 27, 2),
            (43, 32, 2),
            (42, 47, 4),
            (44, 0, 2),
            (44, 0, 0),
        ],
    ), // 16 Subanim_0Slice
    (2, &[(54, 84, 0), (54, 85, 0), (55, 86, 0), (55, 87, 0)]), // 17 Subanim_0BirdiesCirclingEnemy
    (5, &[(54, 84, 0), (54, 85, 0), (55, 86, 0), (55, 87, 0)]), // 18 Subanim_1SwordsCircling
    (
        4,
        &[
            (56, 49, 0),
            (57, 49, 0),
            (56, 50, 0),
            (57, 50, 0),
            (56, 146, 0),
            (57, 146, 0),
            (56, 14, 0),
            (57, 14, 0),
            (56, 15, 0),
            (57, 15, 0),
            (56, 16, 0),
            (57, 16, 0),
        ],
    ), // 19 Subanim_1CloudToss
    (
        2,
        &[
            (58, 8, 0),
            (59, 8, 0),
            (60, 8, 0),
            (61, 8, 0),
            (62, 8, 0),
            (63, 8, 0),
            (62, 8, 0),
            (63, 8, 0),
            (58, 11, 0),
            (59, 11, 0),
            (60, 11, 0),
            (61, 11, 0),
            (62, 11, 0),
            (63, 11, 0),
            (62, 11, 0),
            (63, 11, 0),
        ],
    ), // 1A Subanim_0WaterColumns
    (4, &[(64, 49, 0), (64, 50, 0), (64, 146, 0), (64, 21, 0)]), // 1B Subanim_1SeedToss
    (2, &[(65, 88, 0), (65, 89, 0), (65, 33, 0)]),          // 1C Subanim_1SeedLand
    (
        5,
        &[
            (36, 154, 0),
            (35, 27, 2),
            (36, 34, 0),
            (35, 22, 2),
            (35, 29, 2),
            (36, 152, 0),
            (37, 44, 4),
            (37, 42, 4),
            (37, 153, 4),
            (37, 98, 4),
            (37, 153, 4),
            (37, 98, 4),
            (37, 153, 4),
            (37, 98, 4),
            (37, 153, 3),
        ],
    ), // 1D Subanim_0RocksLift
    (0, &[(37, 117, 0)]),                                   // 1E Subanim_0RocksToss
    (
        4,
        &[
            (12, 48, 3),
            (12, 64, 3),
            (12, 65, 3),
            (12, 66, 3),
            (12, 33, 0),
        ],
    ), // 1F Subanim_1FlameBeam
    (2, &[(66, 7, 0), (67, 7, 0)]),                         // 20 Subanim_1FlameStar
    (2, &[(68, 0, 0), (69, 8, 0), (70, 16, 2)]), // 21 Subanim_0Circles_1Squares_CenteringEnemy
    (
        4,
        &[
            (71, 16, 0),
            (71, 86, 0),
            (71, 7, 0),
            (71, 170, 0),
            (71, 171, 0),
            (71, 172, 0),
            (71, 173, 0),
            (71, 174, 0),
            (71, 175, 0),
            (71, 137, 0),
            (71, 176, 0),
        ],
    ), // 22 Subanim_0Circle_1Square_TossBack
    (2, &[(81, 45, 0), (81, 110, 0)]),           // 23 Subanim_0Bind
    (5, &[(81, 45, 0), (81, 110, 0)]),           // 24 Subanim_0StatusParalyzed
    (3, &[(82, 113, 0), (82, 114, 0)]),          // 25 Subanim_0StatusConfused
    (0, &[(82, 1, 0), (82, 44, 0)]),             // 26 Subanim_0StatusConfusedEnemy
    (5, &[(84, 1, 0), (84, 44, 0)]),             // 27 Subanim_0StatusPoisoned
    (1, &[(85, 115, 3), (86, 115, 3), (87, 115, 0)]), // 28 Subanim_1Sand
    (
        2,
        &[
            (72, 15, 0),
            (74, 104, 3),
            (75, 42, 3),
            (73, 15, 0),
            (74, 104, 3),
            (75, 42, 0),
            (76, 106, 3),
            (77, 105, 3),
            (73, 107, 0),
            (76, 106, 3),
            (77, 105, 0),
            (74, 104, 3),
            (75, 42, 3),
            (73, 108, 0),
            (74, 104, 3),
            (75, 42, 0),
            (76, 106, 3),
            (77, 105, 3),
            (73, 109, 0),
            (76, 106, 3),
            (77, 42, 0),
            (74, 104, 3),
            (75, 42, 3),
            (73, 15, 0),
            (74, 104, 3),
            (75, 42, 0),
            (76, 106, 3),
            (77, 42, 3),
            (73, 107, 0),
        ],
    ), // 29 Subanim_1LightningBall
    (2, &[(78, 43, 0), (79, 43, 0), (80, 43, 0), (80, 43, 0)]), // 2A Subanim_0SliceBothSides
    (
        2,
        &[
            (13, 3, 3),
            (14, 3, 3),
            (15, 3, 0),
            (13, 17, 0),
            (13, 17, 0),
            (13, 55, 0),
            (13, 55, 0),
            (16, 33, 0),
            (16, 33, 0),
            (17, 27, 0),
            (17, 27, 0),
        ],
    ), // 2B Subanim_1Lightning
    (
        2,
        &[
            (18, 1, 0),
            (18, 15, 0),
            (18, 27, 0),
            (18, 37, 0),
            (19, 56, 0),
            (19, 56, 2),
            (20, 56, 0),
            (20, 56, 2),
            (21, 56, 0),
            (21, 56, 0),
            (22, 56, 0),
            (22, 56, 0),
        ],
    ), // 2C Subanim_0WaterDroplets
    (
        3,
        &[
            (68, 100, 0),
            (69, 101, 0),
            (70, 102, 0),
            (71, 102, 0),
            (71, 102, 0),
            (71, 102, 0),
        ],
    ), // 2D Subanim_0CirclesCentering
    (
        1,
        &[
            (24, 67, 2),
            (117, 82, 4),
            (25, 67, 2),
            (117, 99, 4),
            (26, 67, 2),
            (117, 77, 4),
            (27, 67, 2),
            (117, 151, 4),
            (28, 67, 2),
            (117, 152, 4),
            (29, 67, 2),
            (117, 88, 4),
            (30, 67, 2),
            (117, 27, 0),
        ],
    ), // 2E Subanim_0Beam
    (2, &[(31, 36, 0), (32, 32, 0), (33, 26, 0), (34, 21, 0)]), // 2F Subanim_0IceRise
    (
        2,
        &[
            (35, 0, 2),
            (35, 2, 2),
            (35, 4, 0),
            (35, 7, 2),
            (35, 2, 2),
            (35, 4, 0),
            (35, 14, 2),
            (35, 2, 2),
            (35, 12, 0),
            (37, 7, 0),
            (37, 14, 0),
            (37, 21, 0),
            (36, 36, 2),
            (35, 28, 2),
            (35, 35, 0),
            (35, 33, 2),
            (36, 40, 0),
            (36, 40, 0),
        ],
    ), // 30 Subanim_0RocksFallEnemy
    (
        1,
        &[
            (47, 70, 0),
            (47, 74, 0),
            (47, 75, 0),
            (47, 76, 0),
            (47, 77, 0),
            (47, 78, 0),
            (47, 79, 0),
            (47, 80, 0),
            (47, 46, 0),
            (47, 81, 0),
        ],
    ), // 31 Subanim_0SoundWave
    (3, &[(71, 116, 0), (71, 67, 0), (71, 117, 0)]), // 32 Subanim_0Circle_1Square_HalfToss
    (
        1,
        &[
            (88, 118, 0),
            (52, 118, 0),
            (88, 118, 0),
            (52, 118, 0),
            (88, 118, 0),
            (52, 118, 0),
        ],
    ), // 33 Subanim_1Barrier
    (
        1,
        &[
            (72, 125, 0),
            (73, 125, 0),
            (90, 125, 0),
            (72, 48, 0),
            (73, 48, 0),
            (90, 48, 0),
            (72, 126, 0),
            (73, 126, 0),
            (90, 126, 0),
            (72, 127, 0),
            (73, 127, 0),
            (90, 127, 0),
            (72, 128, 0),
            (73, 128, 0),
            (90, 128, 0),
            (72, 129, 0),
            (73, 129, 0),
            (90, 129, 0),
            (72, 130, 0),
            (73, 130, 0),
            (90, 130, 0),
        ],
    ), // 34 Subanim_1Selfdestruct
    (1, &[(91, 131, 3), (92, 132, 3), (93, 133, 3), (94, 9, 0)]), // 35 Subanim_0WaterBubbles
    (
        2,
        &[
            (95, 42, 0),
            (95, 0, 0),
            (96, 42, 0),
            (96, 0, 0),
            (97, 42, 0),
            (97, 0, 0),
            (98, 42, 0),
            (98, 0, 0),
        ],
    ), // 36 Subanim_0CirclesFalling
    (
        1,
        &[
            (99, 137, 0),
            (100, 117, 0),
            (99, 118, 0),
            (101, 13, 0),
            (101, 134, 0),
            (101, 18, 0),
            (101, 135, 0),
            (101, 23, 0),
            (101, 136, 0),
            (101, 26, 0),
        ],
    ), // 37 Subanim_0StringShot
    (
        2,
        &[
            (102, 138, 0),
            (102, 51, 0),
            (102, 46, 0),
            (103, 36, 3),
            (102, 1, 4),
            (102, 16, 4),
            (102, 29, 4),
            (103, 40, 3),
            (102, 42, 4),
            (102, 14, 4),
            (102, 27, 4),
            (103, 38, 3),
            (102, 3, 4),
            (102, 18, 4),
            (102, 30, 4),
            (103, 41, 0),
        ],
    ), // 38 Subanim_0IceFall
    (3, &[(71, 103, 0)]),                        // 39 Subanim_0Circle_1Square_Appears
    (3, &[(83, 113, 0), (83, 127, 0), (83, 129, 0)]), // 3A Subanim_0StatusSleep
    (0, &[(83, 1, 0), (83, 21, 0), (83, 44, 0)]), // 3B Subanim_0StatusSleepEnemy
    (
        3,
        &[
            (89, 121, 3),
            (89, 123, 3),
            (89, 119, 3),
            (89, 122, 3),
            (89, 120, 3),
            (89, 124, 3),
            (89, 118, 0),
        ],
    ), // 3C Subanim_0Water_1Fire_Barrier
    (
        0,
        &[
            (58, 77, 0),
            (59, 77, 0),
            (60, 77, 0),
            (61, 77, 0),
            (62, 77, 0),
            (63, 77, 0),
            (62, 77, 0),
            (63, 77, 0),
        ],
    ), // 3D Subanim_0Water_1Fire_Geyser
    (
        4,
        &[
            (2, 49, 0),
            (52, 49, 0),
            (2, 49, 0),
            (2, 50, 0),
            (52, 50, 0),
            (2, 50, 0),
            (2, 146, 0),
            (52, 146, 0),
            (2, 146, 0),
            (2, 14, 0),
            (52, 14, 0),
            (2, 14, 0),
            (2, 15, 0),
            (52, 15, 0),
            (2, 15, 0),
            (2, 16, 0),
            (52, 16, 0),
            (2, 16, 0),
        ],
    ), // 3E Subanim_1StarBigToss
    (
        3,
        &[
            (104, 75, 0),
            (104, 140, 0),
            (104, 32, 0),
            (104, 28, 0),
            (104, 25, 0),
            (104, 20, 0),
            (104, 118, 0),
            (104, 141, 0),
            (104, 21, 0),
            (104, 16, 0),
            (104, 12, 0),
            (104, 6, 0),
            (104, 142, 0),
            (104, 143, 0),
            (104, 144, 0),
            (104, 38, 0),
            (104, 35, 0),
            (104, 31, 0),
        ],
    ), // 3F Subanim_1StarsSmallToss
    (
        2,
        &[
            (23, 84, 0),
            (23, 85, 0),
            (23, 14, 0),
            (23, 86, 0),
            (23, 87, 0),
            (23, 19, 0),
        ],
    ), // 40 Subanim_1MusicCirclingEnemy
    (
        4,
        &[(3, 49, 0), (3, 50, 0), (3, 146, 0), (3, 14, 0), (3, 16, 0)],
    ), // 41 Subanim_1CircleBlackToss
    (2, &[(72, 8, 0), (73, 8, 0), (90, 8, 0)]),  // 42 Subanim_1ExplosionSmallEnemy
    (5, &[(106, 7, 0), (107, 15, 0), (108, 23, 0)]), // 43 Subanim_0Circle_1Square_Closing
    (
        1,
        &[
            (105, 75, 0),
            (105, 140, 0),
            (105, 32, 0),
            (105, 28, 0),
            (105, 25, 0),
            (105, 20, 0),
            (105, 118, 0),
            (105, 141, 0),
            (105, 21, 0),
            (105, 16, 0),
            (105, 12, 0),
            (105, 6, 0),
        ],
    ), // 44 Subanim_1LeavesToss
    (
        1,
        &[(109, 139, 0), (109, 132, 0), (109, 99, 0), (109, 140, 0)],
    ), // 45 Subanim_0HornJabTwice
    (
        1,
        &[
            (109, 139, 0),
            (109, 132, 0),
            (109, 99, 0),
            (109, 140, 0),
            (109, 10, 0),
            (109, 137, 0),
        ],
    ), // 46 Subanim_0HornJabThrice
    (1, &[(6, 130, 0), (7, 130, 0), (8, 150, 0)]), // 47 Subanim_0BallPoof
    (
        0,
        &[
            (3, 65, 4),
            (3, 72, 4),
            (4, 72, 4),
            (3, 72, 4),
            (5, 72, 4),
            (3, 72, 3),
        ],
    ), // 48 Subanim_2TradeBallDrop
    (0, &[(4, 72, 4), (3, 72, 4), (5, 72, 4), (3, 72, 3)]), // 49 Subanim_2TradeBallShake
    (0, &[(4, 132, 3)]),                         // 4A Subanim_2TradeBallAppear
    (0, &[(6, 114, 0), (7, 114, 0), (8, 114, 0)]), // 4B Subanim_2TradeBallPoof
    (
        3,
        &[
            (111, 48, 0),
            (110, 48, 0),
            (112, 48, 0),
            (110, 48, 0),
            (111, 48, 0),
            (110, 48, 0),
            (112, 48, 0),
            (110, 48, 0),
        ],
    ), // 4C Subanim_0EggShaking
    (
        1,
        &[
            (50, 75, 0),
            (51, 79, 0),
            (50, 32, 0),
            (51, 22, 0),
            (50, 25, 0),
            (51, 13, 0),
        ],
    ), // 4D Subanim_1TriangleToss
    (2, &[(113, 15, 3)]),                        // 4E Subanim_1SphereBig
    (
        2,
        &[
            (113, 15, 0),
            (113, 8, 0),
            (113, 1, 0),
            (113, 149, 0),
            (114, 149, 0),
            (115, 149, 0),
            (116, 149, 0),
        ],
    ), // 4F Subanim_1SphereBigRise
    (
        2,
        &[
            (116, 149, 0),
            (115, 149, 0),
            (114, 149, 0),
            (113, 149, 0),
            (113, 1, 0),
            (113, 8, 0),
            (113, 15, 0),
            (113, 22, 0),
        ],
    ), // 50 Subanim_1SphereBigFall
    (
        5,
        &[
            (118, 27, 0),
            (52, 27, 0),
            (118, 27, 0),
            (52, 27, 0),
            (118, 27, 0),
            (52, 27, 0),
        ],
    ), // 51 Subanim_0Shell
    (
        2,
        &[
            (119, 37, 0),
            (119, 155, 0),
            (119, 26, 0),
            (119, 156, 0),
            (119, 47, 0),
            (119, 80, 0),
            (119, 140, 0),
        ],
    ), // 52 Subanim_0CoinBounce
    (
        0,
        &[
            (120, 48, 0),
            (120, 162, 0),
            (120, 147, 0),
            (120, 97, 0),
            (120, 115, 0),
            (120, 167, 0),
            (120, 51, 0),
            (120, 168, 0),
            (120, 14, 0),
            (120, 169, 0),
            (120, 52, 0),
            (1, 158, 0),
        ],
    ), // 53 Subanim_0SafariRock
    (
        0,
        &[
            (121, 48, 0),
            (121, 162, 0),
            (121, 147, 0),
            (121, 97, 0),
            (121, 115, 0),
            (121, 167, 0),
            (121, 51, 0),
            (121, 168, 0),
            (121, 14, 0),
            (121, 169, 0),
            (121, 52, 0),
        ],
    ), // 54 Subanim_0SafariBait
    (2, &[(1, 157, 0)]),                         // 55 Subanim_0StarHigh
];

// ─── Move animation data ─────────────────────────────────────────────

pub const NUM_MOVE_ANIMS: usize = 203;

pub fn get_move_animation(id: usize) -> MoveAnimation {
    if id >= NUM_MOVE_ANIMS {
        return MoveAnimation::empty();
    }
    let cmds = MOVE_ANIM_DATA[id];
    MoveAnimation {
        commands: cmds
            .iter()
            .map(|&(kind, a, b, c)| {
                if kind == 0 {
                    AnimCommand::SubAnim {
                        sound_id: a,
                        subanim_id: b,
                        tileset: c >> 6,
                        delay: c & 0x3F,
                    }
                } else {
                    AnimCommand::Effect {
                        sound_id: a,
                        effect: SpecialEffect::from_u8(b).unwrap_or(SpecialEffect::WavyScreen),
                    }
                }
            })
            .collect(),
    }
}

pub(crate) static MOVE_ANIM_DATA: [&[(u8, u8, u8, u8)]; 203] = [
    &[(0, 1, 1, 8)],                                                 // 00 Pound
    &[(0, 2, 3, 8)],                                                 // 01 KarateChop
    &[(0, 3, 1, 5), (0, 3, 1, 5)],                                   // 02 DoubleSlap
    &[(0, 4, 2, 4), (0, 4, 2, 4)],                                   // 03 CometPunch
    &[(0, 5, 4, 70)],                                                // 04 MegaPunch
    &[(0, 1, 1, 8), (0, 6, 82, 4)],                                  // 05 PayDay
    &[(0, 7, 2, 6), (0, 0, 17, 70)],                                 // 06 FirePunch
    &[(0, 8, 2, 6), (0, 0, 47, 16)],                                 // 07 IcePunch
    &[(0, 9, 2, 6), (1, 0, 253, 0), (0, 0, 43, 70), (1, 0, 252, 0)], // 08 ThunderPunch
    &[(0, 10, 15, 6)],                                               // 09 Scratch
    &[(0, 11, 42, 8)],                                               // 0A Vicegrip
    &[(0, 12, 42, 6)],                                               // 0B Guillotine
    &[(0, 13, 22, 4)],                                               // 0C RazorWind
    &[(0, 14, 24, 70), (0, 14, 24, 70), (0, 14, 24, 70)],            // 0D SwordsDance
    &[(1, 15, 254, 0), (0, 0, 22, 4)],                               // 0E Cut
    &[(0, 16, 16, 70), (0, 0, 2, 6)],                                // 0F Gust
    &[(0, 17, 4, 70)],                                               // 10 WingAttack
    &[(0, 18, 16, 70), (1, 0, 219, 0)],                              // 11 Whirlwind
    &[(0, 19, 4, 70), (1, 0, 221, 0)],                               // 12 Fly
    &[(0, 20, 35, 4), (0, 20, 35, 4)],                               // 13 Bind
    &[(0, 21, 2, 6)],                                                // 14 Slam
    &[(0, 22, 22, 1), (0, 0, 1, 8)],                                 // 15 VineWhip
    &[(0, 23, 5, 72)],                                               // 16 Stomp
    &[(0, 24, 1, 8), (0, 24, 1, 8)],                                 // 17 DoubleKick
    &[(0, 25, 4, 70)],                                               // 18 MegaKick
    &[(0, 26, 4, 70)],                                               // 19 JumpKick
    &[(1, 27, 254, 0), (0, 0, 4, 70)],                               // 1A RollingKick
    &[(0, 28, 40, 70)],                                              // 1B SandAttack
    &[(0, 29, 5, 70)],                                               // 1C Headbutt
    &[(0, 30, 69, 6), (0, 0, 5, 70)],                                // 1D HornAttack
    &[(0, 31, 70, 2), (0, 0, 70, 2)],                                // 1E FuryAttack
    &[
        (0, 32, 5, 66),
        (0, 0, 5, 66),
        (0, 0, 5, 66),
        (0, 0, 5, 66),
        (0, 0, 5, 66),
    ], // 1F HornDrill
    &[(1, 73, 242, 0), (1, 0, 241, 0)],                              // 20 Tackle
    &[
        (1, 73, 242, 0),
        (1, 0, 254, 0),
        (1, 0, 254, 0),
        (1, 0, 241, 0),
    ], // 21 BodySlam
    &[(0, 35, 35, 4), (0, 35, 35, 4), (0, 35, 35, 4)],               // 22 Wrap
    &[(1, 73, 242, 0), (1, 36, 254, 0), (1, 0, 241, 0)],             // 23 TakeDown
    &[(0, 37, 4, 70)],                                               // 24 Thrash
    &[
        (1, 73, 240, 0),
        (0, 0, 45, 6),
        (1, 0, 252, 0),
        (1, 0, 242, 0),
        (1, 38, 254, 0),
        (1, 0, 241, 0),
    ], // 25 DoubleEdge
    &[
        (1, 133, 242, 0),
        (1, 0, 225, 0),
        (1, 133, 241, 0),
        (1, 0, 225, 0),
        (1, 133, 242, 0),
        (1, 0, 225, 0),
        (1, 133, 241, 0),
    ], // 26 TailWhip
    &[(0, 40, 0, 6)],                                                // 27 PoisonSting
    &[(0, 41, 1, 5), (0, 41, 1, 5)],                                 // 28 Twineedle
    &[(0, 42, 1, 3)],                                                // 29 PinMissile
    &[
        (1, 73, 253, 0),
        (1, 43, 254, 0),
        (1, 43, 254, 0),
        (1, 0, 252, 0),
    ], // 2A Leer
    &[(0, 44, 2, 8)],                                                // 2B Bite
    &[(0, 45, 18, 70)],                                              // 2C Growl
    &[(0, 46, 21, 70), (0, 46, 21, 70), (0, 46, 21, 70)],            // 2D Roar
    &[(0, 47, 18, 70), (0, 0, 64, 80), (0, 0, 64, 80)],              // 2E Sing
    &[(0, 48, 49, 6)],                                               // 2F Supersonic
    &[
        (0, 46, 21, 70),
        (0, 46, 21, 70),
        (0, 16, 16, 70),
        (0, 0, 5, 70),
    ], // 30 SonicBoom
    &[
        (1, 73, 253, 0),
        (1, 43, 254, 0),
        (1, 43, 254, 0),
        (1, 0, 252, 0),
    ], // 31 Disable
    &[(0, 51, 19, 70), (0, 51, 20, 70)],                             // 32 Acid
    &[(0, 52, 17, 70)],                                              // 33 Ember
    &[(0, 53, 31, 70), (0, 53, 12, 70), (0, 53, 13, 70)],            // 34 Flamethrower
    &[(1, 0, 240, 0), (1, 57, 250, 0), (1, 0, 252, 0)],              // 35 Mist
    &[(0, 55, 44, 6)],                                               // 36 WaterGun
    &[(0, 56, 26, 6), (0, 56, 26, 6)],                               // 37 HydroPump
    &[(1, 57, 250, 0), (0, 56, 26, 6)],                              // 38 Surf
    &[(0, 58, 46, 3), (0, 0, 47, 16)],                               // 39 IceBeam
    &[(0, 59, 56, 4), (0, 56, 56, 4)],                               // 3A Blizzard
    &[(0, 60, 46, 3), (1, 0, 248, 0)],                               // 3B PsyBeam
    &[(0, 61, 53, 18)],                                              // 3C BubbleBeam
    &[(0, 62, 46, 3), (1, 0, 225, 0), (1, 0, 225, 0)],               // 3D AuroraBeam
    &[
        (1, 73, 253, 0),
        (1, 0, 226, 0),
        (0, 63, 46, 2),
        (1, 0, 254, 0),
        (1, 0, 254, 0),
        (0, 5, 4, 70),
        (1, 0, 252, 0),
    ], // 3E HyperBeam
    &[(0, 64, 1, 8)],                                                // 3F Peck
    &[(0, 65, 4, 70)],                                               // 40 DrillPeck
    &[(1, 66, 244, 0), (0, 0, 1, 6), (1, 0, 221, 0)],                // 41 Submission
    &[(1, 67, 244, 0), (0, 0, 4, 70), (1, 0, 221, 0)],               // 42 LowKick
    &[(1, 68, 244, 0), (0, 0, 4, 70), (1, 0, 221, 0)],               // 43 Counter
    &[
        (1, 0, 222, 0),
        (0, 140, 78, 65),
        (1, 0, 223, 0),
        (1, 0, 244, 0),
        (0, 69, 79, 66),
        (1, 0, 225, 0),
        (1, 0, 225, 0),
        (1, 0, 221, 0),
        (0, 69, 80, 65),
        (1, 0, 220, 0),
        (1, 0, 251, 0),
    ], // 44 SeismicToss
    &[(1, 73, 242, 0), (1, 0, 241, 0), (0, 7, 4, 70)],               // 45 Strength
    &[
        (1, 71, 240, 0),
        (0, 0, 33, 6),
        (0, 0, 34, 6),
        (1, 0, 252, 0),
    ], // 46 Absorb
    &[
        (1, 72, 240, 0),
        (1, 0, 254, 0),
        (0, 0, 33, 6),
        (0, 0, 34, 6),
        (1, 0, 254, 0),
        (1, 0, 252, 0),
    ], // 47 MegaDrain
    &[(0, 73, 27, 70), (0, 78, 28, 85)],                             // 48 LeechSeed
    &[(1, 74, 240, 0), (1, 0, 226, 0), (1, 0, 252, 0)],              // 49 Growth
    &[(1, 75, 231, 0), (0, 129, 68, 65), (0, 13, 22, 1)],            // 4A RazorLeaf
    &[(0, 76, 46, 6), (0, 0, 1, 6)],                                 // 4B SolarBeam
    &[(0, 77, 54, 6)],                                               // 4C PoisonPowder
    &[(0, 78, 54, 6)],                                               // 4D StunSpore
    &[(0, 79, 54, 6)],                                               // 4E SleepPowder
    &[(1, 80, 240, 0), (1, 0, 230, 0), (1, 0, 252, 0)],              // 4F PetalDance
    &[(0, 81, 55, 8)],                                               // 50 StringShot
    &[
        (0, 82, 31, 70),
        (0, 0, 12, 70),
        (0, 0, 13, 70),
        (0, 0, 14, 70),
    ], // 51 DragonRage
    &[(0, 83, 12, 70), (0, 0, 13, 70), (0, 0, 14, 70)],              // 52 FireSpin
    &[(0, 84, 41, 66)],                                              // 53 ThunderShock
    &[(0, 85, 41, 65), (0, 85, 41, 65)],                             // 54 ThunderBolt
    &[(0, 86, 41, 66), (0, 0, 35, 2), (0, 0, 35, 4)],                // 55 ThunderWave
    &[
        (1, 87, 253, 0),
        (1, 0, 254, 0),
        (0, 0, 43, 70),
        (1, 0, 254, 0),
        (0, 85, 41, 66),
        (1, 0, 252, 0),
    ], // 56 Thunder
    &[(0, 88, 48, 4)],                                               // 57 RockThrow
    &[(1, 89, 251, 0), (1, 89, 251, 0)],                             // 58 Earthquake
    &[
        (1, 90, 254, 0),
        (1, 0, 251, 0),
        (1, 90, 254, 0),
        (1, 0, 251, 0),
    ], // 59 Fissure
    &[(0, 91, 4, 70), (1, 0, 247, 0)],                               // 5A Dig
    &[(1, 57, 250, 0), (0, 92, 20, 70)],                             // 5B Toxic
    &[(1, 93, 248, 0)],                                              // 5C Confusion
    &[(1, 94, 248, 0), (1, 0, 216, 0)],                              // 5D Psychic
    &[(1, 95, 248, 0)],                                              // 5E Hypnosis
    &[
        (1, 96, 240, 0),
        (0, 0, 67, 70),
        (1, 0, 254, 0),
        (1, 0, 252, 0),
    ], // 5F Meditate
    &[(1, 97, 240, 0), (1, 0, 252, 0)],                              // 60 Agility
    &[(1, 98, 244, 0), (0, 0, 4, 70), (1, 0, 221, 0)],               // 61 QuickAttack
    &[(0, 99, 1, 6)],                                                // 62 Rage
    &[(1, 100, 238, 0), (1, 0, 237, 0)],                             // 63 Teleport
    &[(1, 93, 248, 0), (1, 0, 216, 0)],                              // 64 NightShade
    &[(0, 102, 33, 70), (0, 102, 34, 70)],                           // 65 Mimic
    &[(0, 103, 18, 70)],                                             // 66 Screech
    &[
        (1, 0, 253, 0),
        (1, 0, 225, 0),
        (1, 0, 225, 0),
        (1, 0, 254, 0),
        (1, 0, 254, 0),
        (1, 0, 252, 0),
        (1, 104, 218, 0),
        (1, 0, 221, 0),
        (0, 112, 51, 70),
    ], // 67 DoubleTeam
    &[
        (1, 105, 243, 0),
        (1, 0, 240, 0),
        (1, 0, 226, 0),
        (1, 0, 252, 0),
    ], // 68 Recover
    &[
        (1, 106, 240, 0),
        (0, 0, 67, 70),
        (1, 0, 254, 0),
        (1, 0, 252, 0),
    ], // 69 Harden
    &[
        (1, 107, 240, 0),
        (1, 0, 226, 0),
        (1, 0, 234, 0),
        (1, 0, 252, 0),
    ], // 6A Minimize
    &[
        (0, 108, 40, 70),
        (0, 0, 10, 4),
        (1, 0, 249, 0),
        (1, 0, 225, 0),
        (1, 0, 225, 0),
        (1, 0, 253, 0),
        (1, 0, 225, 0),
        (1, 0, 225, 0),
        (1, 0, 225, 0),
        (1, 0, 225, 0),
        (1, 0, 225, 0),
        (1, 0, 225, 0),
        (1, 0, 249, 0),
        (1, 0, 225, 0),
        (1, 0, 252, 0),
    ], // 6B SmokeScreen
    &[(1, 109, 253, 0), (0, 0, 62, 70), (1, 0, 252, 0)],             // 6C ConfuseRay
    &[
        (1, 111, 240, 0),
        (1, 0, 246, 0),
        (0, 0, 81, 6),
        (1, 0, 252, 0),
        (1, 0, 221, 0),
    ], // 6D Withdraw
    &[
        (1, 111, 240, 0),
        (0, 0, 67, 6),
        (1, 0, 254, 0),
        (1, 0, 252, 0),
    ], // 6E DefenseCurl
    &[(0, 112, 51, 70), (0, 112, 51, 70)],                           // 6F Barrier
    &[
        (1, 0, 240, 0),
        (0, 113, 51, 70),
        (0, 113, 51, 70),
        (1, 0, 252, 0),
    ], // 70 LightScreen
    &[(1, 0, 249, 0), (1, 57, 250, 0), (1, 0, 252, 0)],              // 71 Haze
    &[
        (1, 0, 253, 0),
        (0, 115, 51, 70),
        (0, 115, 51, 70),
        (1, 0, 252, 0),
    ], // 72 Reflect
    &[(1, 116, 226, 0)],                                             // 73 FocusEnergy
    &[(0, 117, 4, 70)],                                              // 74 Bide
    &[
        (1, 133, 242, 0),
        (1, 0, 225, 0),
        (1, 133, 241, 0),
        (1, 0, 225, 0),
        (1, 133, 242, 0),
        (1, 0, 225, 0),
        (1, 133, 241, 0),
    ], // 75 Metronome
    &[(0, 119, 1, 8)],                                               // 76 MirrorMove
    &[(0, 120, 52, 67)],                                             // 77 Selfdestruct
    &[(0, 121, 65, 68), (0, 121, 66, 68)],                           // 78 EggBomb
    &[(0, 124, 20, 70)],                                             // 79 Lick
    &[(1, 73, 249, 0), (0, 123, 25, 70), (1, 0, 252, 0)],            // 7A Smog
    &[(0, 124, 19, 70), (0, 124, 20, 70)],                           // 7B Sludge
    &[(0, 125, 2, 8)],                                               // 7C BoneClub
    &[
        (0, 126, 31, 70),
        (0, 0, 32, 70),
        (0, 0, 32, 70),
        (0, 0, 12, 70),
        (0, 0, 13, 70),
    ], // 7D FireBlast
    &[
        (1, 73, 246, 0),
        (0, 56, 26, 6),
        (0, 0, 2, 8),
        (1, 0, 247, 0),
    ], // 7E Waterfall
    &[(0, 128, 42, 8), (0, 132, 35, 6), (0, 132, 35, 6)],            // 7F Clamp
    &[(0, 129, 63, 67)],                                             // 80 Swift
    &[(0, 130, 5, 70)],                                              // 81 SkullBash
    &[(0, 131, 4, 68)],                                              // 82 SpikeCannon
    &[(0, 132, 35, 6), (0, 132, 35, 6), (0, 132, 35, 6)],            // 83 Constrict
    &[(0, 133, 37, 8), (0, 133, 37, 8)],                             // 84 Amnesia
    &[(0, 134, 1, 8)],                                               // 85 Kinesis
    &[
        (1, 73, 229, 0),
        (0, 135, 76, 8),
        (1, 0, 240, 0),
        (1, 0, 226, 0),
        (1, 0, 252, 0),
        (1, 0, 221, 0),
    ], // 86 Softboiled
    &[(0, 136, 4, 70)],                                              // 87 HiJumpKick
    &[
        (1, 73, 253, 0),
        (1, 137, 254, 0),
        (1, 0, 254, 0),
        (1, 0, 252, 0),
    ], // 88 Glare
    &[
        (1, 138, 248, 0),
        (1, 138, 253, 0),
        (0, 138, 2, 8),
        (1, 0, 252, 0),
    ], // 89 DreamEater
    &[(0, 139, 25, 70)],                                             // 8A PoisonGas
    &[(0, 140, 65, 67), (0, 0, 85, 5)],                              // 8B Barrage
    &[
        (0, 141, 2, 8),
        (1, 0, 254, 0),
        (0, 0, 33, 6),
        (0, 0, 34, 6),
        (1, 0, 254, 0),
    ], // 8C LeechLife
    &[(0, 142, 18, 6)],                                              // 8D LovelyKiss
    &[
        (1, 143, 238, 0),
        (1, 0, 237, 0),
        (0, 136, 4, 70),
        (1, 0, 221, 0),
    ], // 8E SkyAttack
    &[
        (0, 144, 33, 70),
        (0, 144, 34, 68),
        (0, 0, 71, 8),
        (1, 0, 232, 0),
    ], // 8F Transform
    &[(0, 145, 53, 22)],                                             // 90 Bubble
    &[
        (0, 146, 23, 6),
        (0, 146, 23, 6),
        (0, 146, 23, 6),
        (0, 3, 2, 6),
    ], // 91 DizzyPunch
    &[(0, 147, 54, 6)],                                              // 92 Spore
    &[
        (1, 73, 240, 0),
        (1, 137, 254, 0),
        (1, 0, 254, 0),
        (1, 0, 252, 0),
    ], // 93 Flash
    &[(0, 48, 49, 6), (1, 93, 216, 0)],                              // 94 Psywave
    &[(1, 150, 235, 0)],                                             // 95 Splash
    &[(1, 151, 233, 0)],                                             // 96 AcidArmor
    &[(0, 152, 5, 70), (0, 0, 42, 6)],                               // 97 CrabHammer
    &[(0, 153, 52, 67)],                                             // 98 Explosion
    &[(0, 154, 15, 4)],                                              // 99 FurySwipes
    &[(0, 155, 2, 6)],                                               // 9A Bonemerang
    &[(0, 156, 58, 16), (0, 156, 58, 16)],                           // 9B Rest
    &[(0, 157, 29, 4), (0, 157, 30, 3), (0, 158, 4, 70)],            // 9C RockSlide
    &[(0, 158, 2, 6)],                                               // 9D HyperFang
    &[
        (1, 159, 240, 0),
        (0, 0, 67, 70),
        (1, 0, 254, 0),
        (1, 0, 252, 0),
    ], // 9E Sharpen
    &[
        (1, 160, 254, 0),
        (0, 0, 33, 70),
        (0, 0, 34, 70),
        (1, 0, 254, 0),
    ], // 9F Conversion
    &[(1, 161, 254, 0), (0, 0, 77, 70), (1, 0, 254, 0)],             // A0 TriAttack
    &[(1, 73, 253, 0), (0, 162, 4, 70), (1, 0, 252, 0)],             // A1 SuperFang
    &[(0, 163, 15, 6)],                                              // A2 Slash
    &[(1, 164, 244, 0), (0, 0, 71, 8), (1, 0, 217, 0)],              // A3 Substitute
    &[(0, 1, 1, 8)],                                                 // A4 Struggle
    &[(1, 0, 220, 0)],                                               // A5 ShowPic
    &[(1, 0, 221, 0)],                                               // A6 EnemyFlash
    &[(1, 0, 245, 0)],                                               // A7 PlayerFlash
    &[(1, 0, 228, 0)],                                               // A8 EnemyHUDShake
    &[(0, 0, 72, 134)],                                              // A9 TradeBallDrop
    &[(0, 0, 73, 132)],                                              // AA TradeBallAppear1
    &[(0, 0, 74, 134)],                                              // AB TradeBallAppear2
    &[(0, 0, 75, 134)],                                              // AC TradeBallPoof
    &[(1, 0, 240, 0), (1, 0, 226, 0), (1, 0, 252, 0)],               // AD XStatItem
    &[(1, 0, 240, 0), (1, 0, 226, 0), (1, 0, 252, 0)],               // AE XStatItem
    &[(1, 0, 240, 0), (0, 0, 67, 70), (1, 0, 252, 0)],               // AF ShrinkingSquare
    &[(1, 0, 240, 0), (0, 0, 67, 70), (1, 0, 252, 0)],               // B0 ShrinkingSquare
    &[(1, 0, 249, 0), (1, 0, 226, 0), (1, 0, 252, 0)],               // B1 XStatItemBlack
    &[(1, 0, 249, 0), (1, 0, 226, 0), (1, 0, 252, 0)],               // B2 XStatItemBlack
    &[(1, 0, 249, 0), (0, 0, 67, 70), (1, 0, 252, 0)],               // B3 ShrinkingSquareBlack
    &[(1, 0, 249, 0), (0, 0, 67, 70), (1, 0, 252, 0)],               // B4 ShrinkingSquareBlack
    &[(1, 0, 240, 0), (1, 0, 236, 0), (1, 0, 252, 0)],               // B5 Unused
    &[(1, 0, 240, 0), (1, 0, 236, 0), (1, 0, 252, 0)],               // B6 Unused
    &[(0, 20, 36, 4), (0, 20, 36, 4)],                               // B7 Paralyze
    &[(0, 20, 36, 4), (0, 20, 36, 4)],                               // B8 Paralyze
    &[(0, 20, 39, 8), (0, 20, 39, 8)],                               // B9 Poison
    &[(0, 20, 39, 8), (0, 20, 39, 8)],                               // BA Poison
    &[(0, 156, 58, 16), (0, 156, 58, 16)],                           // BB SleepPlayer
    &[(0, 156, 59, 16), (0, 156, 59, 16)],                           // BC SleepEnemy
    &[(0, 133, 37, 8), (0, 133, 37, 8)],                             // BD ConfusedPlayer
    &[(0, 133, 38, 8), (0, 133, 38, 8)],                             // BE ConfusedEnemy
    &[(1, 91, 246, 0)],                                              // BF SlideDown
    &[(0, 0, 6, 3)],                                                 // C0 BallToss
    &[(0, 0, 9, 4)],                                                 // C1 BallShake
    &[(0, 0, 10, 4)],                                                // C2 BallPoof
    &[(0, 0, 11, 3)],                                                // C3 BallBlock
    &[(0, 0, 7, 3)],                                                 // C4 GreatToss
    &[(0, 0, 8, 2)],                                                 // C5 UltraToss
    &[(1, 0, 251, 0)],                                               // C6 ShakeScreen
    &[(1, 0, 223, 0)],                                               // C7 HidePic
    &[(0, 140, 83, 3)],                                              // C8 ThrowRock
    &[(0, 140, 84, 3)],                                              // C9 ThrowBait
    &[(1, 0, 216, 0)],                                               // CA ZigZagScreen
];

// ─── Animation tick result ───────────────────────────────────────────

/// Result of a single animation tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimTickResult {
    /// Animation is still playing — OAM entries ready to render.
    Playing,
    /// Waiting for a delay (number of frames remaining).
    WaitDelay(u8),
    /// A special effect needs to be applied by the caller.
    Effect(SpecialEffect),
    /// The entire move animation is complete.
    Done,
}

/// High-level effect actions the caller should perform.
/// These map special effects to concrete rendering operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimEffect {
    /// No action needed (effect handled internally or is a no-op).
    None,
    /// Flash/invert the screen palette for N frames.
    FlashScreen { frames: u8 },
    /// Shake the screen horizontally by N pixels for M frames.
    ShakeScreenH { pixels: i8, frames: u8 },
    /// Shake the screen vertically by N pixels for M frames.
    ShakeScreenV { pixels: i8, frames: u8 },
    /// Hide the player's mon sprite.
    HidePlayerMon,
    /// Show the player's mon sprite.
    ShowPlayerMon,
    /// Hide the enemy's mon sprite.
    HideEnemyMon,
    /// Show the enemy's mon sprite.
    ShowEnemyMon,
    /// Slide the enemy mon off screen.
    SlideEnemyMonOff,
    /// Slide the player mon partially off.
    SlidePlayerMonHalfOff,
    /// Slide the player mon down and hide.
    SlidePlayerMonDown,
    /// Slide the player mon up to original position.
    SlidePlayerMonUp,
    /// Reset the player mon to original position.
    ResetPlayerMonPosition,
    /// Move the player mon horizontally.
    MovePlayerMonH,
    /// Blink the enemy mon sprite.
    BlinkEnemyMon { times: u8 },
    /// Blink the player mon sprite.
    BlinkPlayerMon { times: u8 },
    /// Flash the enemy mon picture.
    FlashEnemyMonPic,
    /// Flash the player mon picture.
    FlashPlayerMonPic,
    /// Set screen to dark palette.
    DarkScreenPalette,
    /// Set screen to light palette.
    LightScreenPalette,
    /// Reset screen palette to normal.
    ResetScreenPalette,
    /// Darken the active mon's palette.
    DarkenMonPalette,
    /// Wavy screen distortion effect.
    WavyScreen,
    /// Substitute doll animation.
    SubstituteMon,
    /// Transform mon into another.
    TransformMon,
    /// Minimize mon (shrink).
    MinimizeMon,
    /// Bounce up and down.
    BounceUpAndDown,
    /// Squish the mon picture.
    SquishMonPic,
    /// Spiral balls inward.
    SpiralBallsInward,
    /// Shoot balls upward.
    ShootBallsUpward { many: bool },
    /// Petals falling.
    PetalsFalling,
    /// Leaves falling.
    LeavesFalling,
    /// Water droplets everywhere.
    WaterDroplets,
    /// Shake back and forth.
    ShakeBackAndForth,
    /// Shake enemy HUD.
    ShakeEnemyHud { variant: u8 },
    /// Delay for 10 frames.
    Delay10,
}

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
