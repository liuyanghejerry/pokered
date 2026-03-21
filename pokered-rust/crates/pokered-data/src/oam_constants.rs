pub const OAM_COUNT: usize = 40;
pub const OBJ_SIZE: usize = 4;
pub const OAM_Y_OFS: u8 = 16;
pub const OAM_X_OFS: u8 = 8;
pub const SCREEN_HEIGHT_PX: u8 = 144;
pub const SCREEN_WIDTH_PX: u8 = 160;

pub const BIT_END_OF_OAM_DATA: u8 = 0;
pub const BIT_SPRITE_UNDER_GRASS: u8 = 1;
pub const FACING_END: u8 = 1 << BIT_END_OF_OAM_DATA;
pub const UNDER_GRASS: u8 = 1 << BIT_SPRITE_UNDER_GRASS;
pub const OAM_XFLIP: u8 = 1 << 5;

pub const NUM_SPRITESTATEDATA_STRUCTS: usize = 16;
pub const SPRITESTATEDATA_LENGTH: usize = 16;
