use crate::Rgba;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GbColor {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

impl GbColor {
    pub const ALL: [GbColor; 4] = [
        GbColor::White,
        GbColor::LightGray,
        GbColor::DarkGray,
        GbColor::Black,
    ];

    pub fn from_u8(val: u8) -> Self {
        match val & 0x03 {
            0 => GbColor::White,
            1 => GbColor::LightGray,
            2 => GbColor::DarkGray,
            _ => GbColor::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    pub colors: [Rgba; 4],
}

impl Palette {
    pub fn new(colors: [Rgba; 4]) -> Self {
        Self { colors }
    }

    pub fn color(&self, index: GbColor) -> Rgba {
        self.colors[index as usize]
    }

    pub fn from_bgp_register(bgp: u8, base_palette: &Palette) -> Self {
        let mut colors = [Rgba::WHITE; 4];
        for i in 0..4 {
            let shade = (bgp >> (i * 2)) & 0x03;
            colors[i] = base_palette.colors[shade as usize];
        }
        Self { colors }
    }
}

pub const DMG_PALETTE: Palette = Palette {
    colors: [
        Rgba::rgb(0x9B, 0xBC, 0x0F), // White (lightest green)
        Rgba::rgb(0x8B, 0xAC, 0x0F), // Light gray
        Rgba::rgb(0x30, 0x62, 0x30), // Dark gray
        Rgba::rgb(0x0F, 0x38, 0x0F), // Black (darkest green)
    ],
};

pub const GRAYSCALE_PALETTE: Palette = Palette {
    colors: [
        Rgba::rgb(0xFF, 0xFF, 0xFF),
        Rgba::rgb(0xAA, 0xAA, 0xAA),
        Rgba::rgb(0x55, 0x55, 0x55),
        Rgba::rgb(0x00, 0x00, 0x00),
    ],
};

pub const POCKET_PALETTE: Palette = Palette {
    colors: [
        Rgba::rgb(0xC4, 0xCF, 0xA1), // lightest
        Rgba::rgb(0x8B, 0x95, 0x6D), // light
        Rgba::rgb(0x4D, 0x53, 0x3C), // dark
        Rgba::rgb(0x1F, 0x1F, 0x1F), // darkest
    ],
};

pub const DEFAULT_BGP: u8 = 0b11100100; // shade 3,2,1,0 (normal)
pub const DEFAULT_OBP0: u8 = 0b11010000; // shade 3,1,0,0 (sprite palette 0, color 0 = transparent)
pub const DEFAULT_OBP1: u8 = 0b11100100; // shade 3,2,1,0

#[derive(Debug, Clone)]
pub struct PaletteState {
    pub base: Palette,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
}

impl PaletteState {
    pub fn new(base: Palette) -> Self {
        Self {
            base,
            bgp: DEFAULT_BGP,
            obp0: DEFAULT_OBP0,
            obp1: DEFAULT_OBP1,
        }
    }

    pub fn bg_palette(&self) -> Palette {
        Palette::from_bgp_register(self.bgp, &self.base)
    }

    pub fn obj_palette0(&self) -> Palette {
        Palette::from_bgp_register(self.obp0, &self.base)
    }

    pub fn obj_palette1(&self) -> Palette {
        Palette::from_bgp_register(self.obp1, &self.base)
    }

    pub fn white_out(&mut self) {
        self.bgp = 0x00;
        self.obp0 = 0x00;
        self.obp1 = 0x00;
    }

    pub fn reset_normal(&mut self) {
        self.bgp = DEFAULT_BGP;
        self.obp0 = DEFAULT_OBP0;
    }
}

impl Default for PaletteState {
    fn default() -> Self {
        Self::new(DMG_PALETTE)
    }
}

// ============================================================================
// M5.9: SGB/CGB Palette System
// ============================================================================

/// SGB palette IDs — indexes into SuperPalettes.
/// Matches `PAL_*` constants from constants/palette_constants.asm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SgbPaletteId {
    Route = 0x00,
    Pallet = 0x01,
    Viridian = 0x02,
    Pewter = 0x03,
    Cerulean = 0x04,
    Lavender = 0x05,
    Vermilion = 0x06,
    Celadon = 0x07,
    Fuchsia = 0x08,
    Cinnabar = 0x09,
    Indigo = 0x0A,
    Saffron = 0x0B,
    TownMap = 0x0C,
    Logo1 = 0x0D,
    Logo2 = 0x0E,
    Pal0F = 0x0F,
    MewMon = 0x10,
    BlueMon = 0x11,
    RedMon = 0x12,
    CyanMon = 0x13,
    PurpleMon = 0x14,
    BrownMon = 0x15,
    GreenMon = 0x16,
    PinkMon = 0x17,
    YellowMon = 0x18,
    GrayMon = 0x19,
    Slots1 = 0x1A,
    Slots2 = 0x1B,
    Slots3 = 0x1C,
    Slots4 = 0x1D,
    Black = 0x1E,
    GreenBar = 0x1F,
    YellowBar = 0x20,
    RedBar = 0x21,
    Badge = 0x22,
    Cave = 0x23,
    GameFreak = 0x24,
}

/// Total number of SGB palettes (NUM_SGB_PALS = 0x25 = 37).
pub const NUM_SGB_PALS: usize = 0x25;

impl SgbPaletteId {
    pub fn from_u8(val: u8) -> Option<Self> {
        if val < NUM_SGB_PALS as u8 {
            // SAFETY: all values 0..0x24 are valid enum discriminants
            Some(unsafe { core::mem::transmute(val) })
        } else {
            None
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Palette command IDs dispatched by _RunPaletteCommand.
/// Matches `SET_PAL_*` constants from constants/palette_constants.asm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SetPalCommand {
    BattleBlack = 0x00,
    Battle = 0x01,
    TownMap = 0x02,
    StatusScreen = 0x03,
    Pokedex = 0x04,
    Slots = 0x05,
    TitleScreen = 0x06,
    NidorinoIntro = 0x07,
    Generic = 0x08,
    Overworld = 0x09,
    PartyMenu = 0x0A,
    PokemonWholeScreen = 0x0B,
    GameFreakIntro = 0x0C,
    TrainerCard = 0x0D,
}

/// Special command: update HP bar colors in party menu BLK packet.
pub const SET_PAL_PARTY_MENU_HP_BARS: u8 = 0xFC;
/// Special command: use wDefaultPaletteCommand instead.
pub const SET_PAL_DEFAULT: u8 = 0xFF;

impl SetPalCommand {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x00 => Some(Self::BattleBlack),
            0x01 => Some(Self::Battle),
            0x02 => Some(Self::TownMap),
            0x03 => Some(Self::StatusScreen),
            0x04 => Some(Self::Pokedex),
            0x05 => Some(Self::Slots),
            0x06 => Some(Self::TitleScreen),
            0x07 => Some(Self::NidorinoIntro),
            0x08 => Some(Self::Generic),
            0x09 => Some(Self::Overworld),
            0x0A => Some(Self::PartyMenu),
            0x0B => Some(Self::PokemonWholeScreen),
            0x0C => Some(Self::GameFreakIntro),
            0x0D => Some(Self::TrainerCard),
            _ => None,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// A single SGB color in 5-bit-per-channel RGB format.
/// Stored as the original 5-bit values (0-31 per channel).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl SgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Convert 5-bit SGB color to 8-bit RGBA.
    /// Uses the standard conversion: `(val << 3) | (val >> 2)`.
    pub const fn to_rgba(self) -> crate::Rgba {
        let r8 = (self.r << 3) | (self.r >> 2);
        let g8 = (self.g << 3) | (self.g >> 2);
        let b8 = (self.b << 3) | (self.b >> 2);
        crate::Rgba::rgb(r8, g8, b8)
    }
}

/// An SGB palette entry: 4 colors (color0..color3).
/// color0 is typically the lightest, color3 the darkest.
pub type SgbPaletteEntry = [SgbColor; 4];

/// Helper to create an SGB palette entry from 12 values (r,g,b × 4 colors).
const fn sgb_pal(
    r0: u8,
    g0: u8,
    b0: u8,
    r1: u8,
    g1: u8,
    b1: u8,
    r2: u8,
    g2: u8,
    b2: u8,
    r3: u8,
    g3: u8,
    b3: u8,
) -> SgbPaletteEntry {
    [
        SgbColor::new(r0, g0, b0),
        SgbColor::new(r1, g1, b1),
        SgbColor::new(r2, g2, b2),
        SgbColor::new(r3, g3, b3),
    ]
}

/// SuperPalettes for Pokémon Red version.
/// 37 entries, each with 4 SGB colors (5-bit per channel).
/// Transcribed from data/sgb/sgb_palettes.asm with DEF(_RED).
pub const SUPER_PALETTES_RED: [SgbPaletteEntry; NUM_SGB_PALS] = [
    sgb_pal(31, 29, 31, 21, 28, 11, 20, 26, 31, 03, 02, 02), // PAL_ROUTE     0x00
    sgb_pal(31, 29, 31, 25, 28, 27, 20, 26, 31, 03, 02, 02), // PAL_PALLET    0x01
    sgb_pal(31, 29, 31, 17, 26, 03, 20, 26, 31, 03, 02, 02), // PAL_VIRIDIAN  0x02
    sgb_pal(31, 29, 31, 23, 25, 16, 20, 26, 31, 03, 02, 02), // PAL_PEWTER    0x03
    sgb_pal(31, 29, 31, 17, 20, 30, 20, 26, 31, 03, 02, 02), // PAL_CERULEAN  0x04
    sgb_pal(31, 29, 31, 27, 20, 27, 20, 26, 31, 03, 02, 02), // PAL_LAVENDER  0x05
    sgb_pal(31, 29, 31, 30, 18, 00, 20, 26, 31, 03, 02, 02), // PAL_VERMILION 0x06
    sgb_pal(31, 29, 31, 16, 30, 22, 20, 26, 31, 03, 02, 02), // PAL_CELADON   0x07
    sgb_pal(31, 29, 31, 31, 15, 22, 20, 26, 31, 03, 02, 02), // PAL_FUCHSIA   0x08
    sgb_pal(31, 29, 31, 26, 10, 06, 20, 26, 31, 03, 02, 02), // PAL_CINNABAR  0x09
    sgb_pal(31, 29, 31, 22, 14, 24, 20, 26, 31, 03, 02, 02), // PAL_INDIGO    0x0A
    sgb_pal(31, 29, 31, 27, 27, 03, 20, 26, 31, 03, 02, 02), // PAL_SAFFRON   0x0B
    sgb_pal(31, 29, 31, 20, 26, 31, 17, 23, 10, 03, 02, 02), // PAL_TOWNMAP   0x0C
    sgb_pal(31, 29, 31, 30, 30, 17, 17, 23, 10, 21, 00, 04), // PAL_LOGO1     0x0D (RED)
    sgb_pal(31, 29, 31, 30, 30, 17, 18, 18, 24, 07, 07, 16), // PAL_LOGO2     0x0E
    sgb_pal(31, 29, 31, 24, 20, 30, 11, 20, 30, 03, 02, 02), // PAL_0F        0x0F
    sgb_pal(31, 29, 31, 30, 22, 17, 16, 14, 19, 03, 02, 02), // PAL_MEWMON    0x10
    sgb_pal(31, 29, 31, 18, 20, 27, 11, 15, 23, 03, 02, 02), // PAL_BLUEMON   0x11
    sgb_pal(31, 29, 31, 31, 20, 10, 26, 10, 06, 03, 02, 02), // PAL_REDMON    0x12
    sgb_pal(31, 29, 31, 21, 25, 29, 14, 19, 25, 03, 02, 02), // PAL_CYANMON   0x13
    sgb_pal(31, 29, 31, 27, 22, 24, 21, 15, 23, 03, 02, 02), // PAL_PURPLEMON 0x14
    sgb_pal(31, 29, 31, 28, 20, 15, 21, 14, 09, 03, 02, 02), // PAL_BROWNMON  0x15
    sgb_pal(31, 29, 31, 20, 26, 16, 09, 20, 11, 03, 02, 02), // PAL_GREENMON  0x16
    sgb_pal(31, 29, 31, 30, 22, 24, 28, 15, 21, 03, 02, 02), // PAL_PINKMON   0x17
    sgb_pal(31, 29, 31, 31, 28, 14, 26, 20, 00, 03, 02, 02), // PAL_YELLOWMON 0x18
    sgb_pal(31, 29, 31, 26, 21, 22, 15, 15, 18, 03, 02, 02), // PAL_GRAYMON   0x19
    sgb_pal(31, 29, 31, 26, 21, 22, 27, 20, 06, 03, 02, 02), // PAL_SLOTS1    0x1A
    sgb_pal(31, 29, 31, 31, 31, 17, 25, 17, 21, 03, 02, 02), // PAL_SLOTS2    0x1B (RED)
    sgb_pal(31, 29, 31, 22, 31, 16, 25, 17, 21, 03, 02, 02), // PAL_SLOTS3    0x1C (RED)
    sgb_pal(31, 29, 31, 16, 19, 29, 25, 17, 21, 03, 02, 02), // PAL_SLOTS4    0x1D (RED)
    sgb_pal(31, 29, 31, 07, 07, 07, 02, 03, 03, 03, 02, 02), // PAL_BLACK     0x1E
    sgb_pal(31, 29, 31, 30, 26, 15, 09, 20, 11, 03, 02, 02), // PAL_GREENBAR  0x1F
    sgb_pal(31, 29, 31, 30, 26, 15, 26, 20, 00, 03, 02, 02), // PAL_YELLOWBAR 0x20
    sgb_pal(31, 29, 31, 30, 26, 15, 26, 10, 06, 03, 02, 02), // PAL_REDBAR    0x21
    sgb_pal(31, 29, 31, 30, 22, 17, 11, 15, 23, 03, 02, 02), // PAL_BADGE     0x22
    sgb_pal(31, 29, 31, 21, 14, 09, 18, 24, 22, 03, 02, 02), // PAL_CAVE      0x23
    sgb_pal(31, 29, 31, 31, 28, 14, 24, 20, 10, 03, 02, 02), // PAL_GAMEFREAK 0x24
];

/// SuperPalettes for Pokémon Blue version.
/// Only PAL_LOGO1 (0x0D), PAL_SLOTS2-4 (0x1B-0x1D) differ from Red.
pub const SUPER_PALETTES_BLUE: [SgbPaletteEntry; NUM_SGB_PALS] = [
    sgb_pal(31, 29, 31, 21, 28, 11, 20, 26, 31, 03, 02, 02), // PAL_ROUTE     0x00
    sgb_pal(31, 29, 31, 25, 28, 27, 20, 26, 31, 03, 02, 02), // PAL_PALLET    0x01
    sgb_pal(31, 29, 31, 17, 26, 03, 20, 26, 31, 03, 02, 02), // PAL_VIRIDIAN  0x02
    sgb_pal(31, 29, 31, 23, 25, 16, 20, 26, 31, 03, 02, 02), // PAL_PEWTER    0x03
    sgb_pal(31, 29, 31, 17, 20, 30, 20, 26, 31, 03, 02, 02), // PAL_CERULEAN  0x04
    sgb_pal(31, 29, 31, 27, 20, 27, 20, 26, 31, 03, 02, 02), // PAL_LAVENDER  0x05
    sgb_pal(31, 29, 31, 30, 18, 00, 20, 26, 31, 03, 02, 02), // PAL_VERMILION 0x06
    sgb_pal(31, 29, 31, 16, 30, 22, 20, 26, 31, 03, 02, 02), // PAL_CELADON   0x07
    sgb_pal(31, 29, 31, 31, 15, 22, 20, 26, 31, 03, 02, 02), // PAL_FUCHSIA   0x08
    sgb_pal(31, 29, 31, 26, 10, 06, 20, 26, 31, 03, 02, 02), // PAL_CINNABAR  0x09
    sgb_pal(31, 29, 31, 22, 14, 24, 20, 26, 31, 03, 02, 02), // PAL_INDIGO    0x0A
    sgb_pal(31, 29, 31, 27, 27, 03, 20, 26, 31, 03, 02, 02), // PAL_SAFFRON   0x0B
    sgb_pal(31, 29, 31, 20, 26, 31, 17, 23, 10, 03, 02, 02), // PAL_TOWNMAP   0x0C
    sgb_pal(31, 29, 31, 30, 30, 17, 21, 00, 04, 14, 19, 29), // PAL_LOGO1     0x0D (BLUE)
    sgb_pal(31, 29, 31, 30, 30, 17, 18, 18, 24, 07, 07, 16), // PAL_LOGO2     0x0E
    sgb_pal(31, 29, 31, 24, 20, 30, 11, 20, 30, 03, 02, 02), // PAL_0F        0x0F
    sgb_pal(31, 29, 31, 30, 22, 17, 16, 14, 19, 03, 02, 02), // PAL_MEWMON    0x10
    sgb_pal(31, 29, 31, 18, 20, 27, 11, 15, 23, 03, 02, 02), // PAL_BLUEMON   0x11
    sgb_pal(31, 29, 31, 31, 20, 10, 26, 10, 06, 03, 02, 02), // PAL_REDMON    0x12
    sgb_pal(31, 29, 31, 21, 25, 29, 14, 19, 25, 03, 02, 02), // PAL_CYANMON   0x13
    sgb_pal(31, 29, 31, 27, 22, 24, 21, 15, 23, 03, 02, 02), // PAL_PURPLEMON 0x14
    sgb_pal(31, 29, 31, 28, 20, 15, 21, 14, 09, 03, 02, 02), // PAL_BROWNMON  0x15
    sgb_pal(31, 29, 31, 20, 26, 16, 09, 20, 11, 03, 02, 02), // PAL_GREENMON  0x16
    sgb_pal(31, 29, 31, 30, 22, 24, 28, 15, 21, 03, 02, 02), // PAL_PINKMON   0x17
    sgb_pal(31, 29, 31, 31, 28, 14, 26, 20, 00, 03, 02, 02), // PAL_YELLOWMON 0x18
    sgb_pal(31, 29, 31, 26, 21, 22, 15, 15, 18, 03, 02, 02), // PAL_GRAYMON   0x19
    sgb_pal(31, 29, 31, 26, 21, 22, 27, 20, 06, 03, 02, 02), // PAL_SLOTS1    0x1A
    sgb_pal(31, 29, 31, 31, 31, 17, 16, 19, 29, 03, 02, 02), // PAL_SLOTS2    0x1B (BLUE)
    sgb_pal(31, 29, 31, 22, 31, 16, 16, 19, 29, 03, 02, 02), // PAL_SLOTS3    0x1C (BLUE)
    sgb_pal(31, 29, 31, 25, 17, 21, 16, 19, 29, 03, 02, 02), // PAL_SLOTS4    0x1D (BLUE)
    sgb_pal(31, 29, 31, 07, 07, 07, 02, 03, 03, 03, 02, 02), // PAL_BLACK     0x1E
    sgb_pal(31, 29, 31, 30, 26, 15, 09, 20, 11, 03, 02, 02), // PAL_GREENBAR  0x1F
    sgb_pal(31, 29, 31, 30, 26, 15, 26, 20, 00, 03, 02, 02), // PAL_YELLOWBAR 0x20
    sgb_pal(31, 29, 31, 30, 26, 15, 26, 10, 06, 03, 02, 02), // PAL_REDBAR    0x21
    sgb_pal(31, 29, 31, 30, 22, 17, 11, 15, 23, 03, 02, 02), // PAL_BADGE     0x22
    sgb_pal(31, 29, 31, 21, 14, 09, 18, 24, 22, 03, 02, 02), // PAL_CAVE      0x23
    sgb_pal(31, 29, 31, 31, 28, 14, 24, 20, 10, 03, 02, 02), // PAL_GAMEFREAK 0x24
];

/// Get the SuperPalettes table for a given game version.
pub fn super_palettes(is_red: bool) -> &'static [SgbPaletteEntry; NUM_SGB_PALS] {
    if is_red {
        &SUPER_PALETTES_RED
    } else {
        &SUPER_PALETTES_BLUE
    }
}

/// Look up an SGB palette entry by ID and version.
pub fn lookup_sgb_palette(id: SgbPaletteId, is_red: bool) -> &'static SgbPaletteEntry {
    &super_palettes(is_red)[id as usize]
}

/// Convert an SgbPaletteEntry (4 SgbColors) into a Palette (4 Rgba colors).
pub fn sgb_entry_to_palette(entry: &SgbPaletteEntry) -> Palette {
    Palette::new([
        entry[0].to_rgba(),
        entry[1].to_rgba(),
        entry[2].to_rgba(),
        entry[3].to_rgba(),
    ])
}

/// Number of Pokémon species (including MISSINGNO at index 0).
pub const NUM_POKEMON_PLUS_ONE: usize = 151 + 1; // indices 0..=151

/// Monster palette assignments (internal index order, NOT Pokédex order).
/// Index 0 = MISSINGNO, 1 = BULBASAUR, ..., 151 = MEW.
/// Transcribed from data/pokemon/palettes.asm.
pub const MONSTER_PALETTES: [SgbPaletteId; NUM_POKEMON_PLUS_ONE] = [
    SgbPaletteId::MewMon,    // 0: MISSINGNO
    SgbPaletteId::GreenMon,  // 1: BULBASAUR
    SgbPaletteId::GreenMon,  // 2: IVYSAUR
    SgbPaletteId::GreenMon,  // 3: VENUSAUR
    SgbPaletteId::RedMon,    // 4: CHARMANDER
    SgbPaletteId::RedMon,    // 5: CHARMELEON
    SgbPaletteId::RedMon,    // 6: CHARIZARD
    SgbPaletteId::CyanMon,   // 7: SQUIRTLE
    SgbPaletteId::CyanMon,   // 8: WARTORTLE
    SgbPaletteId::CyanMon,   // 9: BLASTOISE
    SgbPaletteId::GreenMon,  // 10: CATERPIE
    SgbPaletteId::GreenMon,  // 11: METAPOD
    SgbPaletteId::CyanMon,   // 12: BUTTERFREE
    SgbPaletteId::YellowMon, // 13: WEEDLE
    SgbPaletteId::YellowMon, // 14: KAKUNA
    SgbPaletteId::YellowMon, // 15: BEEDRILL
    SgbPaletteId::BrownMon,  // 16: PIDGEY
    SgbPaletteId::BrownMon,  // 17: PIDGEOTTO
    SgbPaletteId::BrownMon,  // 18: PIDGEOT
    SgbPaletteId::GrayMon,   // 19: RATTATA
    SgbPaletteId::GrayMon,   // 20: RATICATE
    SgbPaletteId::BrownMon,  // 21: SPEAROW
    SgbPaletteId::BrownMon,  // 22: FEAROW
    SgbPaletteId::PurpleMon, // 23: EKANS
    SgbPaletteId::PurpleMon, // 24: ARBOK
    SgbPaletteId::YellowMon, // 25: PIKACHU
    SgbPaletteId::YellowMon, // 26: RAICHU
    SgbPaletteId::BrownMon,  // 27: SANDSHREW
    SgbPaletteId::BrownMon,  // 28: SANDSLASH
    SgbPaletteId::BlueMon,   // 29: NIDORAN_F
    SgbPaletteId::BlueMon,   // 30: NIDORINA
    SgbPaletteId::BlueMon,   // 31: NIDOQUEEN
    SgbPaletteId::PurpleMon, // 32: NIDORAN_M
    SgbPaletteId::PurpleMon, // 33: NIDORINO
    SgbPaletteId::PurpleMon, // 34: NIDOKING
    SgbPaletteId::PinkMon,   // 35: CLEFAIRY
    SgbPaletteId::PinkMon,   // 36: CLEFABLE
    SgbPaletteId::RedMon,    // 37: VULPIX
    SgbPaletteId::YellowMon, // 38: NINETALES
    SgbPaletteId::PinkMon,   // 39: JIGGLYPUFF
    SgbPaletteId::PinkMon,   // 40: WIGGLYTUFF
    SgbPaletteId::BlueMon,   // 41: ZUBAT
    SgbPaletteId::BlueMon,   // 42: GOLBAT
    SgbPaletteId::GreenMon,  // 43: ODDISH
    SgbPaletteId::RedMon,    // 44: GLOOM
    SgbPaletteId::RedMon,    // 45: VILEPLUME
    SgbPaletteId::RedMon,    // 46: PARAS
    SgbPaletteId::RedMon,    // 47: PARASECT
    SgbPaletteId::PurpleMon, // 48: VENONAT
    SgbPaletteId::PurpleMon, // 49: VENOMOTH
    SgbPaletteId::BrownMon,  // 50: DIGLETT
    SgbPaletteId::BrownMon,  // 51: DUGTRIO
    SgbPaletteId::YellowMon, // 52: MEOWTH
    SgbPaletteId::YellowMon, // 53: PERSIAN
    SgbPaletteId::YellowMon, // 54: PSYDUCK
    SgbPaletteId::CyanMon,   // 55: GOLDUCK
    SgbPaletteId::BrownMon,  // 56: MANKEY
    SgbPaletteId::BrownMon,  // 57: PRIMEAPE
    SgbPaletteId::BrownMon,  // 58: GROWLITHE
    SgbPaletteId::RedMon,    // 59: ARCANINE
    SgbPaletteId::BlueMon,   // 60: POLIWAG
    SgbPaletteId::BlueMon,   // 61: POLIWHIRL
    SgbPaletteId::BlueMon,   // 62: POLIWRATH
    SgbPaletteId::YellowMon, // 63: ABRA
    SgbPaletteId::YellowMon, // 64: KADABRA
    SgbPaletteId::YellowMon, // 65: ALAKAZAM
    SgbPaletteId::GrayMon,   // 66: MACHOP
    SgbPaletteId::GrayMon,   // 67: MACHOKE
    SgbPaletteId::GrayMon,   // 68: MACHAMP
    SgbPaletteId::GreenMon,  // 69: BELLSPROUT
    SgbPaletteId::GreenMon,  // 70: WEEPINBELL
    SgbPaletteId::GreenMon,  // 71: VICTREEBEL
    SgbPaletteId::CyanMon,   // 72: TENTACOOL
    SgbPaletteId::CyanMon,   // 73: TENTACRUEL
    SgbPaletteId::GrayMon,   // 74: GEODUDE
    SgbPaletteId::GrayMon,   // 75: GRAVELER
    SgbPaletteId::GrayMon,   // 76: GOLEM
    SgbPaletteId::RedMon,    // 77: PONYTA
    SgbPaletteId::RedMon,    // 78: RAPIDASH
    SgbPaletteId::PinkMon,   // 79: SLOWPOKE
    SgbPaletteId::PinkMon,   // 80: SLOWBRO
    SgbPaletteId::GrayMon,   // 81: MAGNEMITE
    SgbPaletteId::GrayMon,   // 82: MAGNETON
    SgbPaletteId::BrownMon,  // 83: FARFETCH'D
    SgbPaletteId::BrownMon,  // 84: DODUO
    SgbPaletteId::BrownMon,  // 85: DODRIO
    SgbPaletteId::BlueMon,   // 86: SEEL
    SgbPaletteId::BlueMon,   // 87: DEWGONG
    SgbPaletteId::PurpleMon, // 88: GRIMER
    SgbPaletteId::PurpleMon, // 89: MUK
    SgbPaletteId::GrayMon,   // 90: SHELLDER
    SgbPaletteId::GrayMon,   // 91: CLOYSTER
    SgbPaletteId::PurpleMon, // 92: GASTLY
    SgbPaletteId::PurpleMon, // 93: HAUNTER
    SgbPaletteId::PurpleMon, // 94: GENGAR
    SgbPaletteId::GrayMon,   // 95: ONIX
    SgbPaletteId::YellowMon, // 96: DROWZEE
    SgbPaletteId::YellowMon, // 97: HYPNO
    SgbPaletteId::RedMon,    // 98: KRABBY
    SgbPaletteId::RedMon,    // 99: KINGLER
    SgbPaletteId::YellowMon, // 100: VOLTORB
    SgbPaletteId::YellowMon, // 101: ELECTRODE
    SgbPaletteId::PinkMon,   // 102: EXEGGCUTE
    SgbPaletteId::GreenMon,  // 103: EXEGGUTOR
    SgbPaletteId::GrayMon,   // 104: CUBONE
    SgbPaletteId::GrayMon,   // 105: MAROWAK
    SgbPaletteId::BrownMon,  // 106: HITMONLEE
    SgbPaletteId::BrownMon,  // 107: HITMONCHAN
    SgbPaletteId::PinkMon,   // 108: LICKITUNG
    SgbPaletteId::PurpleMon, // 109: KOFFING
    SgbPaletteId::PurpleMon, // 110: WEEZING
    SgbPaletteId::GrayMon,   // 111: RHYHORN
    SgbPaletteId::GrayMon,   // 112: RHYDON
    SgbPaletteId::PinkMon,   // 113: CHANSEY
    SgbPaletteId::BlueMon,   // 114: TANGELA
    SgbPaletteId::BrownMon,  // 115: KANGASKHAN
    SgbPaletteId::CyanMon,   // 116: HORSEA
    SgbPaletteId::CyanMon,   // 117: SEADRA
    SgbPaletteId::RedMon,    // 118: GOLDEEN
    SgbPaletteId::RedMon,    // 119: SEAKING
    SgbPaletteId::RedMon,    // 120: STARYU
    SgbPaletteId::GrayMon,   // 121: STARMIE
    SgbPaletteId::PinkMon,   // 122: MR. MIME
    SgbPaletteId::GreenMon,  // 123: SCYTHER
    SgbPaletteId::MewMon,    // 124: JYNX
    SgbPaletteId::YellowMon, // 125: ELECTABUZZ
    SgbPaletteId::RedMon,    // 126: MAGMAR
    SgbPaletteId::BrownMon,  // 127: PINSIR
    SgbPaletteId::GrayMon,   // 128: TAUROS
    SgbPaletteId::RedMon,    // 129: MAGIKARP
    SgbPaletteId::BlueMon,   // 130: GYARADOS
    SgbPaletteId::CyanMon,   // 131: LAPRAS
    SgbPaletteId::GrayMon,   // 132: DITTO
    SgbPaletteId::GrayMon,   // 133: EEVEE
    SgbPaletteId::CyanMon,   // 134: VAPOREON
    SgbPaletteId::YellowMon, // 135: JOLTEON
    SgbPaletteId::RedMon,    // 136: FLAREON
    SgbPaletteId::GrayMon,   // 137: PORYGON
    SgbPaletteId::BlueMon,   // 138: OMANYTE
    SgbPaletteId::BlueMon,   // 139: OMASTAR
    SgbPaletteId::BrownMon,  // 140: KABUTO
    SgbPaletteId::BrownMon,  // 141: KABUTOPS
    SgbPaletteId::GrayMon,   // 142: AERODACTYL
    SgbPaletteId::PinkMon,   // 143: SNORLAX
    SgbPaletteId::BlueMon,   // 144: ARTICUNO
    SgbPaletteId::YellowMon, // 145: ZAPDOS
    SgbPaletteId::RedMon,    // 146: MOLTRES
    SgbPaletteId::GrayMon,   // 147: DRATINI
    SgbPaletteId::BlueMon,   // 148: DRAGONAIR
    SgbPaletteId::BrownMon,  // 149: DRAGONITE
    SgbPaletteId::MewMon,    // 150: MEWTWO
    SgbPaletteId::MewMon,    // 151: MEW
];

/// Lookup the SGB palette for a Pokémon by Pokédex index (1-based).
/// Returns None for invalid indices. Index 0 returns MISSINGNO's palette.
pub fn monster_palette(pokedex_index: u8) -> SgbPaletteId {
    if (pokedex_index as usize) < NUM_POKEMON_PLUS_ONE {
        MONSTER_PALETTES[pokedex_index as usize]
    } else {
        MONSTER_PALETTES[0] // fallback to MISSINGNO
    }
}

/// Determine a Pokémon's palette ID, taking Transform into account.
/// If `is_transformed` is true, uses PAL_GRAYMON (Ditto's palette).
/// Otherwise looks up from MonsterPalettes.
/// Mirrors `DeterminePaletteID` from engine/gfx/palettes.asm.
pub fn determine_palette_id(species_index: u8, is_transformed: bool) -> SgbPaletteId {
    if is_transformed {
        SgbPaletteId::GrayMon
    } else {
        monster_palette(species_index)
    }
}

// ---- Map/tileset constants used by overworld palette selection ----

/// Tileset ID for Pokemon Tower / Agatha's room.
pub const TILESET_CEMETERY: u8 = 15;
/// Tileset ID for caves (Rock Tunnel, Victory Road, etc).
pub const TILESET_CAVERN: u8 = 17;
/// Number of city/town maps (PALLET_TOWN..SAFFRON_CITY = 0x00..0x0A).
pub const NUM_CITY_MAPS: u8 = 0x0B;
/// First indoor map ID (REDS_HOUSE_1F = 0x25).
pub const FIRST_INDOOR_MAP: u8 = 0x25;
/// CERULEAN_CAVE_2F map ID.
pub const MAP_CERULEAN_CAVE_2F: u8 = 0xE2;
/// CERULEAN_CAVE_1F map ID.
pub const MAP_CERULEAN_CAVE_1F: u8 = 0xE4;
/// LORELEI'S_ROOM map ID.
pub const MAP_LORELEIS_ROOM: u8 = 0xF5;
/// BRUNO'S_ROOM map ID.
pub const MAP_BRUNOS_ROOM: u8 = 0xF6;

/// Determine the overworld SGB palette for a given map.
///
/// Mirrors `SetPal_Overworld` from engine/gfx/palettes.asm.
///
/// # Parameters
/// - `tileset`: The current map's tileset ID.
/// - `map_id`: The current map ID.
/// - `last_map`: The town/route the current indoor map belongs to (wLastMap).
///
/// # Returns
/// The SGB palette ID to use for this map location.
pub fn overworld_palette_for_map(tileset: u8, map_id: u8, last_map: u8) -> SgbPaletteId {
    // Cemetery tileset → PAL_GRAYMON (Pokemon Tower, Agatha's room)
    if tileset == TILESET_CEMETERY {
        return SgbPaletteId::GrayMon;
    }
    // Cavern tileset → PAL_CAVE
    if tileset == TILESET_CAVERN {
        return SgbPaletteId::Cave;
    }

    // Determine the effective "town" value
    let town = if map_id < FIRST_INDOOR_MAP {
        // Outdoor town or route — use map_id directly
        map_id
    } else if map_id >= MAP_CERULEAN_CAVE_2F && map_id <= MAP_CERULEAN_CAVE_1F {
        // Cerulean Cave maps → PAL_CAVE
        return SgbPaletteId::Cave;
    } else if map_id == MAP_LORELEIS_ROOM {
        // Lorelei's room → PAL_ROUTE (xor a; inc a → 1 → PAL_ROUTE... actually 0+1=1=PAL_PALLET)
        // ASM: xor a → a=0, then falls through to .town which does inc a → a=1
        // Wait — xor a sets a=0, then jr .town, .town does inc a → a=1 = PAL_PALLET
        // But the comment says PAL_ROUTE. Let's trace: xor a → a=0, then .town: inc a → a=1.
        // PAL_ROUTE=0, PAL_PALLET=1. So Lorelei → PAL_PALLET? No wait.
        // Looking more carefully at the code path:
        // .Lorelei: xor a → a=0; jr .town
        // .town: inc a → a=1; ld [wPalPacket+1], a
        // So the palette index written is 1 = PAL_PALLET.
        // But looking at the original code intent: this should be PAL_ROUTE.
        // xor a = 0. After .town (inc a) = 1. PAL_PALLET = 1.
        // Hmm, actually let's re-examine. The ASM has:
        //   .PokemonTowerOrAgatha: ld a, PAL_GRAYMON - 1; jr .town
        //   .caveOrBruno: ld a, PAL_CAVE - 1; jr .town
        //   .Lorelei: xor a; jr .town
        //   .town: inc a; [stores]
        // So Lorelei: a = 0, inc a = 1 = PAL_PALLET
        // Hmm, but the handoff comment says "PAL_ROUTE". PAL_ROUTE = 0.
        // 0 + 1 = 1 = PAL_PALLET. So it's actually PAL_PALLET.
        // Wait, no: "a town's palette ID is its map ID + 1"
        // For .Lorelei: a=0, then inc a → 1. The palette index stored is 1.
        // The SGB PAL_TRN packet byte at offset 1 is the palette index.
        // In InitCGBPalettes, this byte is multiplied by 8 to index into SuperPalettes.
        // So index 1 = PAL_PALLET.
        return SgbPaletteId::Pallet;
    } else if map_id == MAP_BRUNOS_ROOM {
        // Bruno's room → PAL_CAVE
        return SgbPaletteId::Cave;
    } else {
        // Normal indoor map — use last_map (the town/route the building is in)
        last_map
    };

    // If town < NUM_CITY_MAPS, use (town + 1) as palette index.
    // Town map IDs 0..10 map to PAL_PALLET..PAL_SAFFRON (indices 1..11).
    // Routes (>= NUM_CITY_MAPS) get PAL_ROUTE.
    if town < NUM_CITY_MAPS {
        // town's palette = town + 1 (matches const_def order: PALLET=1, VIRIDIAN=2, etc.)
        SgbPaletteId::from_u8(town + 1).unwrap_or(SgbPaletteId::Route)
    } else {
        // Route or out-of-range → PAL_ROUTE
        // ASM: ld a, PAL_ROUTE - 1; .town: inc a → PAL_ROUTE = 0+1-1+1 = 1? No.
        // PAL_ROUTE - 1 = -1 = 0xFF. inc a → 0x00 = PAL_ROUTE.
        // Wait: PAL_ROUTE = 0. PAL_ROUTE - 1 = 0xFF. inc 0xFF = 0x00 = PAL_ROUTE. Correct.
        SgbPaletteId::Route
    }
}

/// The rendering palette mode.
/// - `Dmg`: Original Game Boy 4-shade grayscale (green-tinted).
/// - `Sgb`: Super Game Boy colorized mode (uses SuperPalettes).
/// - `Grayscale`: Clean grayscale (no green tint).
/// - `Pocket`: Game Boy Pocket palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteMode {
    Dmg,
    Sgb,
    Grayscale,
    Pocket,
}

/// Extended palette state that supports both DMG (4-shade) and SGB (colorized) rendering.
///
/// When in DMG/Grayscale/Pocket mode, uses `PaletteState` (bgp/obp0/obp1 registers).
/// When in SGB mode, overlays SGB color palettes on top of the DMG register state.
#[derive(Debug, Clone)]
pub struct ColorPaletteState {
    /// The base DMG palette register state (always maintained).
    pub dmg: PaletteState,
    /// Current palette mode.
    pub mode: PaletteMode,
    /// Whether this is Pokémon Red (true) or Blue (false).
    pub is_red: bool,
    /// Current SGB palette for background (active when mode == Sgb).
    pub sgb_bg_palette: SgbPaletteId,
    /// Current SGB palette for OBJ0 (sprites).
    pub sgb_obj0_palette: SgbPaletteId,
    /// Current SGB palette for OBJ1 (sprites).
    pub sgb_obj1_palette: SgbPaletteId,
    /// The default palette command (wDefaultPaletteCommand).
    pub default_command: SetPalCommand,
}

impl ColorPaletteState {
    pub fn new(mode: PaletteMode, is_red: bool) -> Self {
        Self {
            dmg: PaletteState::default(),
            mode,
            is_red,
            sgb_bg_palette: SgbPaletteId::Route,
            sgb_obj0_palette: SgbPaletteId::Route,
            sgb_obj1_palette: SgbPaletteId::Route,
            default_command: SetPalCommand::Generic,
        }
    }

    /// Get the effective background palette for rendering.
    pub fn bg_palette(&self) -> Palette {
        match self.mode {
            PaletteMode::Sgb => {
                sgb_entry_to_palette(lookup_sgb_palette(self.sgb_bg_palette, self.is_red))
            }
            PaletteMode::Dmg => self.dmg.bg_palette(),
            PaletteMode::Grayscale => {
                let mut state = self.dmg.clone();
                state.base = GRAYSCALE_PALETTE;
                state.bg_palette()
            }
            PaletteMode::Pocket => {
                let mut state = self.dmg.clone();
                state.base = POCKET_PALETTE;
                state.bg_palette()
            }
        }
    }

    /// Get the effective OBJ palette 0 for rendering.
    pub fn obj_palette0(&self) -> Palette {
        match self.mode {
            PaletteMode::Sgb => {
                sgb_entry_to_palette(lookup_sgb_palette(self.sgb_obj0_palette, self.is_red))
            }
            PaletteMode::Dmg => self.dmg.obj_palette0(),
            PaletteMode::Grayscale => {
                let mut state = self.dmg.clone();
                state.base = GRAYSCALE_PALETTE;
                state.obj_palette0()
            }
            PaletteMode::Pocket => {
                let mut state = self.dmg.clone();
                state.base = POCKET_PALETTE;
                state.obj_palette0()
            }
        }
    }

    /// Get the effective OBJ palette 1 for rendering.
    pub fn obj_palette1(&self) -> Palette {
        match self.mode {
            PaletteMode::Sgb => {
                sgb_entry_to_palette(lookup_sgb_palette(self.sgb_obj1_palette, self.is_red))
            }
            PaletteMode::Dmg => self.dmg.obj_palette1(),
            PaletteMode::Grayscale => {
                let mut state = self.dmg.clone();
                state.base = GRAYSCALE_PALETTE;
                state.obj_palette1()
            }
            PaletteMode::Pocket => {
                let mut state = self.dmg.clone();
                state.base = POCKET_PALETTE;
                state.obj_palette1()
            }
        }
    }

    /// Set the overworld palette based on current map context.
    pub fn set_overworld_palette(&mut self, tileset: u8, map_id: u8, last_map: u8) {
        let pal = overworld_palette_for_map(tileset, map_id, last_map);
        self.sgb_bg_palette = pal;
        self.default_command = SetPalCommand::Overworld;
    }

    /// Set battle palettes from player/enemy species and HP bar colors.
    pub fn set_battle_palette(
        &mut self,
        player_species: u8,
        player_transformed: bool,
        enemy_species: u8,
        enemy_transformed: bool,
        player_hp_bar_color: u8,
        _enemy_hp_bar_color: u8,
    ) {
        let player_pal = determine_palette_id(player_species, player_transformed);
        let enemy_pal = determine_palette_id(enemy_species, enemy_transformed);

        // In battle, BG palette areas are:
        // - Player HP bar → GreenBar/YellowBar/RedBar based on HP
        // - Enemy HP bar → GreenBar/YellowBar/RedBar based on HP
        // - Player mon → species palette
        // - Enemy mon → species palette
        // For simplicity, we store the main bg as player HP bar, obj0 as player mon, obj1 as enemy mon.
        self.sgb_bg_palette = hp_bar_to_sgb_palette(player_hp_bar_color);
        self.sgb_obj0_palette = player_pal;
        self.sgb_obj1_palette = enemy_pal;
        self.default_command = SetPalCommand::Battle;
    }
}

/// Convert HP bar color index (0=green, 1=yellow, 2=red) to SGB palette ID.
pub fn hp_bar_to_sgb_palette(hp_bar_color: u8) -> SgbPaletteId {
    match hp_bar_color {
        0 => SgbPaletteId::GreenBar,
        1 => SgbPaletteId::YellowBar,
        _ => SgbPaletteId::RedBar,
    }
}
