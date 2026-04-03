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

    /// Convert from the Debug/variant name string (e.g. "Overworld" → TilesetId::Overworld)
    pub fn from_name(name: &str) -> Option<TilesetId> {
        match name {
            "Overworld" => Some(TilesetId::Overworld),
            "RedsHouse1" => Some(TilesetId::RedsHouse1),
            "Mart" => Some(TilesetId::Mart),
            "Forest" => Some(TilesetId::Forest),
            "RedsHouse2" => Some(TilesetId::RedsHouse2),
            "Dojo" => Some(TilesetId::Dojo),
            "Pokecenter" => Some(TilesetId::Pokecenter),
            "Gym" => Some(TilesetId::Gym),
            "House" => Some(TilesetId::House),
            "ForestGate" => Some(TilesetId::ForestGate),
            "Museum" => Some(TilesetId::Museum),
            "Underground" => Some(TilesetId::Underground),
            "Gate" => Some(TilesetId::Gate),
            "Ship" => Some(TilesetId::Ship),
            "ShipPort" => Some(TilesetId::ShipPort),
            "Cemetery" => Some(TilesetId::Cemetery),
            "Interior" => Some(TilesetId::Interior),
            "Cavern" => Some(TilesetId::Cavern),
            "Lobby" => Some(TilesetId::Lobby),
            "Mansion" => Some(TilesetId::Mansion),
            "Lab" => Some(TilesetId::Lab),
            "Club" => Some(TilesetId::Club),
            "Facility" => Some(TilesetId::Facility),
            "Plateau" => Some(TilesetId::Plateau),
            _ => None,
        }
    }

    /// Returns the variant name as a string (e.g. TilesetId::Overworld → "Overworld")
    pub fn variant_name(self) -> &'static str {
        match self {
            TilesetId::Overworld => "Overworld",
            TilesetId::RedsHouse1 => "RedsHouse1",
            TilesetId::Mart => "Mart",
            TilesetId::Forest => "Forest",
            TilesetId::RedsHouse2 => "RedsHouse2",
            TilesetId::Dojo => "Dojo",
            TilesetId::Pokecenter => "Pokecenter",
            TilesetId::Gym => "Gym",
            TilesetId::House => "House",
            TilesetId::ForestGate => "ForestGate",
            TilesetId::Museum => "Museum",
            TilesetId::Underground => "Underground",
            TilesetId::Gate => "Gate",
            TilesetId::Ship => "Ship",
            TilesetId::ShipPort => "ShipPort",
            TilesetId::Cemetery => "Cemetery",
            TilesetId::Interior => "Interior",
            TilesetId::Cavern => "Cavern",
            TilesetId::Lobby => "Lobby",
            TilesetId::Mansion => "Mansion",
            TilesetId::Lab => "Lab",
            TilesetId::Club => "Club",
            TilesetId::Facility => "Facility",
            TilesetId::Plateau => "Plateau",
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
