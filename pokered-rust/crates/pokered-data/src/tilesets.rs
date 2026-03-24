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

    /// Returns the PNG filename (without extension) for this tileset,
    /// matching the files in `gfx/tilesets/`.
    pub fn tileset_name(self) -> &'static str {
        match self {
            TilesetId::Overworld => "overworld",
            TilesetId::RedsHouse1 => "reds_house",
            TilesetId::Mart => "pokecenter", // shares pokecenter.bst
            TilesetId::Forest => "forest",
            TilesetId::RedsHouse2 => "reds_house", // shares reds_house.bst
            TilesetId::Dojo => "gym",              // shares gym.bst
            TilesetId::Pokecenter => "pokecenter",
            TilesetId::Gym => "gym",
            TilesetId::House => "house",
            TilesetId::ForestGate => "gate", // shares gate.bst
            TilesetId::Museum => "gate",     // shares gate.bst
            TilesetId::Underground => "underground",
            TilesetId::Gate => "gate",
            TilesetId::Ship => "ship",
            TilesetId::ShipPort => "ship_port",
            TilesetId::Cemetery => "cemetery",
            TilesetId::Interior => "interior",
            TilesetId::Cavern => "cavern",
            TilesetId::Lobby => "lobby",
            TilesetId::Mansion => "mansion",
            TilesetId::Lab => "lab",
            TilesetId::Club => "club",
            TilesetId::Facility => "facility",
            TilesetId::Plateau => "plateau",
        }
    }
}
