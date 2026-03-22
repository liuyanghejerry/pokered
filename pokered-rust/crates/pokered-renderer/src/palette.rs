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
