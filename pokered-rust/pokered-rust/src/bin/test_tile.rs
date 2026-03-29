use pokered_data::blockset_data;
use pokered_data::tilesets::TilesetId;
use pokered_data::collision;

fn main() {
    // Test RedsHouse2F tileset (RedsHouse1/2 share the same blockset)
    let tileset = TilesetId::RedsHouse2;
    
    // Get passable tiles
    let passable = collision::get_passable_tiles(tileset);
    println!("Passable tiles for RedsHouse2: {:02X?}", passable);
    
    // Check a few blocks
    for block_id in [0x10, 0x11, 0x05, 0x08, 0x0F, 0x0D, 0x0C, 0x12].iter() {
        if let Some(tiles) = blockset_data::block_tiles(tileset, *block_id) {
            println!("\nBlock 0x{:02X} tiles:", block_id);
            for row in 0..4 {
                let row_tiles: Vec<String> = (0..4)
                    .map(|col| format!("{:02X}", tiles[row * 4 + col]))
                    .collect();
                println!("  Row {}: {}", row, row_tiles.join(" "));
            }
            
            // Check which tiles are passable
            let passable_check: Vec<(usize, bool)> = tiles.iter()
                .enumerate()
                .map(|(i, &t)| (i, passable.contains(&t)))
                .collect();
            println!("  Passable: {:?}", passable_check);
        }
    }
    
    // Now test the tile lookup for player positions
    println!("\n\nSimulating player positions in RedsHouse2F:");
    
    // Map is 4x4 blocks = 16x16 tiles
    // Block data: [16, 17, 5, 8, 15, 15, 15, 15, 15, 13, 15, 15, 12, 15, 15, 18]
    let blocks: [u8; 16] = [0x10, 0x11, 0x05, 0x08, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0D, 0x0F, 0x0F, 0x0C, 0x0F, 0x0F, 0x12];
    let map_width = 4u8;
    
    // Test positions around the map
    for y in 0..16 {
        for x in 0..16 {
            let block_x = (x / 4) as usize;
            let block_y = (y / 4) as usize;
            let sub_x = (x % 4) as usize;
            let sub_y = (y % 4) as usize;
            
            let block_idx = block_y * (map_width as usize) + block_x;
            let block_id = blocks[block_idx];
            
            if let Some(tiles) = blockset_data::block_tiles(tileset, block_id) {
                let tile_id = tiles[sub_y * 4 + sub_x];
                let is_passable = passable.contains(&tile_id);
                
                if !is_passable {
                    print!("X");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}
