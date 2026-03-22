pub mod game_data;
pub mod hall_of_fame;
pub mod ser_game_data;
pub mod ser_pokemon;
pub mod serialization;

#[cfg(test)]
mod save_tests;

use crate::pokemon::party::Party;
use crate::pokemon::pc_box::{PcBox, PcStorage};
use game_data::GameData;
use hall_of_fame::HallOfFame;
use serde::{Deserialize, Serialize};

use crate::save_menu::calc_checksum;

pub use serialization::{SaveError, SRAM_BANK_SIZE};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub player_name: Vec<u8>,
    pub game_data: GameData,
    pub party: Party,
    pub current_box: PcBox,
    pub pc_storage: PcStorage,
    pub hall_of_fame: HallOfFame,
    pub tile_animations: u8,
}

impl SaveData {
    pub fn new() -> Self {
        Self {
            player_name: Vec::new(),
            game_data: GameData::new(),
            party: Party::new(),
            current_box: PcBox::new(),
            pc_storage: PcStorage::new(),
            hall_of_fame: HallOfFame::new(),
            tile_animations: 0,
        }
    }

    pub fn player_id(&self) -> u16 {
        self.game_data.player_id
    }

    pub fn validate_checksum(&self, stored_checksum: u8) -> bool {
        let data = self.serialize_checksummed_region();
        calc_checksum(&data) == stored_checksum
    }

    pub fn compute_checksum(&self) -> u8 {
        let data = self.serialize_checksummed_region();
        calc_checksum(&data)
    }

    fn serialize_checksummed_region(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.player_name);
        ser_game_data::serialize_game_data_into(&self.game_data, &mut buf);
        ser_pokemon::serialize_sprite_data_into(&mut buf);
        ser_pokemon::serialize_party_into(&self.party, &mut buf);
        ser_pokemon::serialize_box_into(&self.current_box, &mut buf);
        buf.push(self.tile_animations);
        buf
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl Default for SaveData {
    fn default() -> Self {
        Self::new()
    }
}
