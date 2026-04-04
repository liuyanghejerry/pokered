export const TILE_SIZE = 8
export const BLOCK_TILES = 4

// All 45 music IDs from pokered-data/src/music.rs
export const MUSIC_LIST: string[] = [
  'PalletTown',
  'Pokecenter',
  'Gym',
  'Cities1',
  'Cities2',
  'Celadon',
  'Cinnabar',
  'Vermilion',
  'Lavender',
  'SSAnne',
  'MeetProfOak',
  'MeetRival',
  'MuseumGuy',
  'SafariZone',
  'PkmnHealed',
  'Routes1',
  'Routes2',
  'Routes3',
  'Routes4',
  'IndigoPlateau',
  'GymLeaderBattle',
  'TrainerBattle',
  'WildBattle',
  'FinalBattle',
  'DefeatedTrainer',
  'DefeatedWildMon',
  'DefeatedGymLeader',
  'TitleScreen',
  'Credits',
  'HallOfFame',
  'OaksLab',
  'JigglypuffSong',
  'BikeRiding',
  'Surfing',
  'GameCorner',
  'IntroBattle',
  'Dungeon1',
  'Dungeon2',
  'Dungeon3',
  'CinnabarMansion',
  'PokemonTower',
  'SilphCo',
  'MeetEvilTrainer',
  'MeetFemaleTrainer',
  'MeetMaleTrainer',
]

// Direction names for map connections
export const CONNECTION_DIRECTIONS = ['north', 'south', 'west', 'east'] as const
export type ConnectionDirection = typeof CONNECTION_DIRECTIONS[number]

// Town map coordinates for minimap display (from town_map_data.rs)
// Maps 0x00-0x24 (37 outdoor maps) with x,y coordinates on 16x16 grid
export interface TownMapCoord {
  mapId: number
  mapName: string
  x: number
  y: number
  displayName: string
}

// Outdoor map coordinates from OUTDOOR_TOWN_MAP_ENTRIES
export const TOWN_MAP_COORDS: TownMapCoord[] = [
  { mapId: 0x00, mapName: 'PalletTown', x: 2, y: 11, displayName: 'PALLET TOWN' },
  { mapId: 0x01, mapName: 'ViridianCity', x: 2, y: 8, displayName: 'VIRIDIAN CITY' },
  { mapId: 0x02, mapName: 'PewterCity', x: 2, y: 3, displayName: 'PEWTER CITY' },
  { mapId: 0x03, mapName: 'CeruleanCity', x: 10, y: 2, displayName: 'CERULEAN CITY' },
  { mapId: 0x04, mapName: 'LavenderTown', x: 14, y: 5, displayName: 'LAVENDER TOWN' },
  { mapId: 0x05, mapName: 'VermilionCity', x: 10, y: 9, displayName: 'VERMILION CITY' },
  { mapId: 0x06, mapName: 'CeladonCity', x: 7, y: 5, displayName: 'CELADON CITY' },
  { mapId: 0x07, mapName: 'FuchsiaCity', x: 8, y: 13, displayName: 'FUCHSIA CITY' },
  { mapId: 0x08, mapName: 'CinnabarIsland', x: 2, y: 15, displayName: 'CINNABAR ISLAND' },
  { mapId: 0x09, mapName: 'IndigoPlateau', x: 0, y: 2, displayName: 'INDIGO PLATEAU' },
  { mapId: 0x0A, mapName: 'SaffronCity', x: 10, y: 5, displayName: 'SAFFRON CITY' },
  { mapId: 0x0B, mapName: 'UnusedMap0B', x: 0, y: 0, displayName: 'UNUSED' },
  { mapId: 0x0C, mapName: 'Route1', x: 2, y: 10, displayName: 'ROUTE 1' },
  { mapId: 0x0D, mapName: 'Route2', x: 2, y: 6, displayName: 'ROUTE 2' },
  { mapId: 0x0E, mapName: 'Route3', x: 4, y: 3, displayName: 'ROUTE 3' },
  { mapId: 0x0F, mapName: 'Route4', x: 8, y: 2, displayName: 'ROUTE 4' },
  { mapId: 0x10, mapName: 'Route5', x: 10, y: 3, displayName: 'ROUTE 5' },
  { mapId: 0x11, mapName: 'Route6', x: 10, y: 8, displayName: 'ROUTE 6' },
  { mapId: 0x12, mapName: 'Route7', x: 8, y: 5, displayName: 'ROUTE 7' },
  { mapId: 0x13, mapName: 'Route8', x: 13, y: 5, displayName: 'ROUTE 8' },
  { mapId: 0x14, mapName: 'Route9', x: 13, y: 2, displayName: 'ROUTE 9' },
  { mapId: 0x15, mapName: 'Route10', x: 14, y: 4, displayName: 'ROUTE 10' },
  { mapId: 0x16, mapName: 'Route11', x: 12, y: 9, displayName: 'ROUTE 11' },
  { mapId: 0x17, mapName: 'Route12', x: 14, y: 9, displayName: 'ROUTE 12' },
  { mapId: 0x18, mapName: 'Route13', x: 13, y: 11, displayName: 'ROUTE 13' },
  { mapId: 0x19, mapName: 'Route14', x: 11, y: 12, displayName: 'ROUTE 14' },
  { mapId: 0x1A, mapName: 'Route15', x: 10, y: 13, displayName: 'ROUTE 15' },
  { mapId: 0x1B, mapName: 'Route16', x: 5, y: 5, displayName: 'ROUTE 16' },
  { mapId: 0x1C, mapName: 'Route17', x: 4, y: 8, displayName: 'ROUTE 17' },
  { mapId: 0x1D, mapName: 'Route18', x: 6, y: 13, displayName: 'ROUTE 18' },
  { mapId: 0x1E, mapName: 'Route19', x: 6, y: 15, displayName: 'SEA ROUTE 19' },
  { mapId: 0x1F, mapName: 'Route20', x: 4, y: 15, displayName: 'SEA ROUTE 20' },
  { mapId: 0x20, mapName: 'Route21', x: 2, y: 13, displayName: 'SEA ROUTE 21' },
  { mapId: 0x21, mapName: 'Route22', x: 0, y: 8, displayName: 'ROUTE 22' },
  { mapId: 0x22, mapName: 'Route23', x: 0, y: 6, displayName: 'ROUTE 23' },
  { mapId: 0x23, mapName: 'Route24', x: 10, y: 1, displayName: 'ROUTE 24' },
  { mapId: 0x24, mapName: 'Route25', x: 11, y: 0, displayName: 'ROUTE 25' },
]

export const TILESET_FILES: Record<string, string> = {
  Overworld: 'overworld.png',
  RedsHouse1: 'reds_house.png',
  Mart: 'pokecenter.png',
  Forest: 'forest.png',
  RedsHouse2: 'reds_house.png',
  Dojo: 'gym.png',
  Pokecenter: 'pokecenter.png',
  Gym: 'gym.png',
  House: 'house.png',
  ForestGate: 'gate.png',
  Museum: 'gate.png',
  Underground: 'underground.png',
  Gate: 'gate.png',
  Ship: 'ship.png',
  ShipPort: 'ship_port.png',
  Cemetery: 'cemetery.png',
  Interior: 'interior.png',
  Cavern: 'cavern.png',
  Lobby: 'lobby.png',
  Mansion: 'mansion.png',
  Lab: 'lab.png',
  Club: 'club.png',
  Facility: 'facility.png',
  Plateau: 'plateau.png',
}
