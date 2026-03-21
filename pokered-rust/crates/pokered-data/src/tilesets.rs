#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TilesetId {
    Overworld = 0,
    RedsHouse1 = 1,
    Mart = 2,
    Forest = 3,
    RedsHouse2 = 4,
    Dojo = 5,
    Pokecenter = 6,
    Gym = 7,
    House = 8,
    ForestGate = 9,
    Museum = 10,
    Underground = 11,
    Gate = 12,
    Ship = 13,
    ShipPort = 14,
    Cemetery = 15,
    Interior = 16,
    Cavern = 17,
    Lobby = 18,
    Mansion = 19,
    Lab = 20,
    Club = 21,
    Facility = 22,
    Plateau = 23,
}

pub const NUM_TILESETS: usize = 24;

impl TilesetId {
    pub fn from_u8(value: u8) -> Option<TilesetId> {
        if (value as usize) < NUM_TILESETS {
            Some(unsafe { core::mem::transmute(value) })
        } else {
            None
        }
    }
}
