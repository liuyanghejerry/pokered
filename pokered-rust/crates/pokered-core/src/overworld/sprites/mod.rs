pub mod oam;
pub mod collision;
pub mod update;

#[cfg(test)]
mod tests;

use pokered_data::oam_constants::{
    NUM_SPRITESTATEDATA_STRUCTS, OAM_COUNT, SPRITESTATEDATA_LENGTH,
};
use pokered_data::sprite_state_constants::{
    self as sc, MovementStatus, GRASS_PRIORITY, IMAGE_INDEX_OFFSCREEN,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OamEntry {
    pub y: u8,
    pub x: u8,
    pub tile_id: u8,
    pub attributes: u8,
}

#[derive(Debug, Clone)]
pub struct ShadowOam {
    pub entries: [OamEntry; OAM_COUNT],
}

impl Default for ShadowOam {
    fn default() -> Self {
        Self {
            entries: [OamEntry::default(); OAM_COUNT],
        }
    }
}

impl ShadowOam {
    pub fn clear(&mut self) {
        for entry in &mut self.entries {
            *entry = OamEntry::default();
        }
    }

    pub fn hide_all(&mut self) {
        for entry in &mut self.entries {
            entry.y = 160;
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SpriteStateData1 {
    pub picture_id: u8,
    pub movement_status: u8,
    pub image_index: u8,
    pub y_step_vector: i8,
    pub y_pixels: u8,
    pub x_step_vector: i8,
    pub x_pixels: u8,
    pub intra_anim_frame_counter: u8,
    pub anim_frame_counter: u8,
    pub facing_direction: u8,
    pub y_adjusted: u8,
    pub x_adjusted: u8,
    pub collision_data: u8,
    pub field_0d: u8,
    pub collision_sprite_lo: u8,
    pub collision_sprite_hi: u8,
}

impl SpriteStateData1 {
    pub fn is_active(&self) -> bool {
        self.picture_id != 0
    }

    pub fn is_visible(&self) -> bool {
        self.image_index != IMAGE_INDEX_OFFSCREEN
    }

    pub fn movement_status(&self) -> MovementStatus {
        MovementStatus::from_byte(self.movement_status)
    }

    pub fn faces_player(&self) -> bool {
        MovementStatus::face_player_bit(self.movement_status)
    }

    pub fn clear_collision(&mut self) {
        self.collision_data = 0;
        self.collision_sprite_lo = 0;
        self.collision_sprite_hi = 0;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SpriteStateData2 {
    pub walk_anim_counter: u8,
    pub field_01: u8,
    pub y_displacement: u8,
    pub x_displacement: u8,
    pub map_y: u8,
    pub map_x: u8,
    pub movement_byte1: u8,
    pub grass_priority: u8,
    pub movement_delay: u8,
    pub orig_facing_direction: u8,
    pub field_0a: u8,
    pub field_0b: u8,
    pub field_0c: u8,
    pub picture_id: u8,
    pub image_base_offset: u8,
    pub field_0f: u8,
}

impl SpriteStateData2 {
    pub fn init_displacement(&mut self) {
        self.y_displacement = 8;
        self.x_displacement = 8;
    }

    pub fn has_grass_priority(&self) -> bool {
        self.grass_priority & GRASS_PRIORITY != 0
    }
}

pub struct SpriteTable {
    pub data1: [SpriteStateData1; NUM_SPRITESTATEDATA_STRUCTS],
    pub data2: [SpriteStateData2; NUM_SPRITESTATEDATA_STRUCTS],
    pub shadow_oam: ShadowOam,
    pub oam_count: usize,
    pub ledge_or_fishing: bool,
}

impl Default for SpriteTable {
    fn default() -> Self {
        Self {
            data1: [SpriteStateData1::default(); NUM_SPRITESTATEDATA_STRUCTS],
            data2: [SpriteStateData2::default(); NUM_SPRITESTATEDATA_STRUCTS],
            shadow_oam: ShadowOam::default(),
            oam_count: 0,
            ledge_or_fishing: false,
        }
    }
}

impl SpriteTable {
    pub fn clear_all(&mut self) {
        self.data1 = [SpriteStateData1::default(); NUM_SPRITESTATEDATA_STRUCTS];
        self.data2 = [SpriteStateData2::default(); NUM_SPRITESTATEDATA_STRUCTS];
        self.shadow_oam.clear();
        self.oam_count = 0;
    }

    pub fn player_data1(&self) -> &SpriteStateData1 {
        &self.data1[0]
    }

    pub fn player_data1_mut(&mut self) -> &mut SpriteStateData1 {
        &mut self.data1[0]
    }

    pub fn player_data2(&self) -> &SpriteStateData2 {
        &self.data2[0]
    }

    pub fn player_data2_mut(&mut self) -> &mut SpriteStateData2 {
        &mut self.data2[0]
    }

    pub fn active_sprite_count(&self) -> usize {
        self.data1.iter().filter(|s| s.is_active()).count()
    }
}
