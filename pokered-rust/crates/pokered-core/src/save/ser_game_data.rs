use super::game_data::{
    GameData, MapConnection, MAX_BG_EVENTS, MAX_OBJECT_EVENTS, MAX_WARP_EVENTS,
};
use super::ser_pokemon::serialize_name;

fn push_u16_be(buf: &mut Vec<u8>, val: u16) {
    buf.push((val >> 8) as u8);
    buf.push((val & 0xFF) as u8);
}

fn push_bcd_money(buf: &mut Vec<u8>, money: u32) {
    let capped = money.min(999999);
    let d5 = (capped / 100000) as u8;
    let d4 = ((capped / 10000) % 10) as u8;
    let d3 = ((capped / 1000) % 10) as u8;
    let d2 = ((capped / 100) % 10) as u8;
    let d1 = ((capped / 10) % 10) as u8;
    let d0 = (capped % 10) as u8;
    buf.push((d5 << 4) | d4);
    buf.push((d3 << 4) | d2);
    buf.push((d1 << 4) | d0);
}

fn push_bcd_coins(buf: &mut Vec<u8>, coins: u16) {
    let capped = coins.min(9999);
    let d3 = (capped / 1000) as u8;
    let d2 = ((capped / 100) % 10) as u8;
    let d1 = ((capped / 10) % 10) as u8;
    let d0 = (capped % 10) as u8;
    buf.push((d3 << 4) | d2);
    buf.push((d1 << 4) | d0);
}

fn serialize_map_connection(conn: &MapConnection, buf: &mut Vec<u8>) {
    buf.push(conn.connected_map);
    push_u16_be(buf, conn.strip_src);
    push_u16_be(buf, conn.strip_dest);
    buf.push(conn.strip_length);
    buf.push(conn.connected_width);
    buf.push(conn.y_alignment);
    buf.push(conn.x_alignment);
    push_u16_be(buf, conn.view_pointer);
}

fn serialize_inventory(
    inv: &crate::items::inventory::Inventory,
    buf: &mut Vec<u8>,
    capacity: usize,
) {
    let count = inv.count() as u8;
    buf.push(count);
    for i in 0..inv.count() {
        if let Some((item, qty)) = inv.get(i) {
            buf.push(item as u8);
            buf.push(qty);
        }
    }
    buf.push(0xFF);
    let items_written = inv.count() * 2 + 1;
    let total_slots = capacity * 2 + 1;
    for _ in items_written..total_slots {
        buf.push(0);
    }
}

pub fn serialize_game_data_into(data: &GameData, buf: &mut Vec<u8>) {
    buf.extend_from_slice(data.pokedex.owned_flags());
    buf.extend_from_slice(data.pokedex.seen_flags());

    serialize_inventory(&data.bag, buf, 20);

    push_bcd_money(buf, data.player_money);

    serialize_name(&data.rival_name, buf);

    buf.push(data.options.to_byte());

    buf.push(data.obtained_badges);
    buf.push(0); // unused byte after badges

    buf.push(data.letter_printing_delay_flags);

    push_u16_be(buf, data.player_id);

    buf.push(data.map_music_sound_id);
    buf.push(data.map_music_rom_bank);
    buf.push(data.map_pal_offset);

    buf.push(data.position.map_id);
    buf.push(data.position.y);
    buf.push(data.position.x);
    buf.push(data.position.y_block);
    buf.push(data.position.x_block);

    buf.push(data.last_map);

    buf.push(data.map_header.tileset);
    buf.push(data.map_header.height);
    buf.push(data.map_header.width);
    push_u16_be(buf, data.map_header.data_ptr);
    push_u16_be(buf, data.map_header.text_ptr);
    push_u16_be(buf, data.map_header.script_ptr);
    buf.push(data.map_header.connections);

    let conn_mask = data.map_header.connections;
    if conn_mask & 0x08 != 0 {
        serialize_map_connection(&data.connections[0], buf);
    }
    if conn_mask & 0x04 != 0 {
        serialize_map_connection(&data.connections[1], buf);
    }
    if conn_mask & 0x02 != 0 {
        serialize_map_connection(&data.connections[2], buf);
    }
    if conn_mask & 0x01 != 0 {
        serialize_map_connection(&data.connections[3], buf);
    }

    buf.extend_from_slice(&data.sprite_set);
    buf.push(data.sprite_set_id);

    buf.push(data.map_background_tile);

    buf.push(data.num_warps);
    for i in 0..MAX_WARP_EVENTS {
        let w = &data.warp_entries[i];
        buf.push(w.y);
        buf.push(w.x);
        buf.push(w.warp_id);
        buf.push(w.map_id);
    }
    buf.push(data.destination_warp_id);

    buf.push(data.num_signs);
    for i in 0..MAX_BG_EVENTS {
        buf.push(data.sign_coords[i].0);
        buf.push(data.sign_coords[i].1);
    }
    for i in 0..MAX_BG_EVENTS {
        buf.push(data.sign_text_ids[i]);
    }

    buf.push(data.num_sprites);
    for i in 0..MAX_OBJECT_EVENTS {
        buf.push(data.map_sprite_data[i].0);
        buf.push(data.map_sprite_data[i].1);
    }
    for i in 0..MAX_OBJECT_EVENTS {
        buf.push(data.map_sprite_extra_data[i].0);
        buf.push(data.map_sprite_extra_data[i].1);
    }

    buf.push(data.current_map_height2);
    buf.push(data.current_map_width2);

    buf.push(data.player_moving_direction);
    buf.push(data.player_last_stop_direction);
    buf.push(data.player_direction);

    buf.push(data.tileset_bank);
    push_u16_be(buf, data.tileset_blocks_ptr);
    push_u16_be(buf, data.tileset_gfx_ptr);
    push_u16_be(buf, data.tileset_collision_ptr);
    buf.extend_from_slice(&data.tileset_talking_over_tiles);
    buf.push(data.grass_tile);

    serialize_inventory(&data.pc_items, buf, 50);

    buf.push(data.current_box_num);

    buf.push(data.num_hof_teams);

    push_bcd_coins(buf, data.player_coins);

    buf.extend_from_slice(&data.toggleable_object_flags);
    buf.extend_from_slice(&data.game_progress_flags);
    buf.extend_from_slice(&data.obtained_hidden_items);
    buf.extend_from_slice(&data.obtained_hidden_coins);

    buf.push(data.walk_bike_surf_state);

    buf.extend_from_slice(&data.town_visited_flags);

    push_u16_be(buf, data.safari_steps);

    buf.push(data.fossil_item);
    buf.push(data.fossil_mon);

    buf.push(data.enemy_mon_or_trainer_class);

    buf.push(data.rival_starter);
    buf.push(data.player_starter);

    buf.push(data.last_blackout_map);
    buf.push(data.destination_map);

    buf.push(data.tile_in_front_of_boulder);
    buf.push(data.dungeon_warp_dest_map);
    buf.push(data.which_dungeon_warp);

    buf.extend_from_slice(&data.status_flags);

    push_u16_be(buf, data.completed_in_game_trade_flags);

    buf.push(data.warped_from_warp);
    buf.push(data.warped_from_map);

    buf.push(data.card_key_door_y);
    buf.push(data.card_key_door_x);

    buf.push(data.first_lock_trash_can);
    buf.push(data.second_lock_trash_can);

    buf.extend_from_slice(&data.event_flags);

    buf.push(data.grass_rate);
    buf.extend_from_slice(&data.grass_mons);

    buf.push(data.water_rate);
    buf.extend_from_slice(&data.water_mons);

    push_u16_be(buf, data.trainer_header_ptr);
    buf.push(data.opponent_after_wrong_answer);
    buf.push(data.cur_map_script);

    buf.push(data.play_time.hours);
    buf.push(if data.play_time.maxed { 0xFF } else { 0x00 });
    buf.push(data.play_time.minutes);
    buf.push(data.play_time.seconds);
    buf.push(data.play_time.frames);

    buf.push(data.safari_zone_game_over);
    buf.push(data.num_safari_balls);

    buf.push(if data.daycare.in_use { 1 } else { 0 });
    serialize_daycare_mon(&data.daycare, buf);
    serialize_name(&data.daycare_mon_name, buf);
    serialize_name(&data.daycare_mon_ot, buf);
}

fn serialize_daycare_mon(dc: &super::game_data::DayCareMon, buf: &mut Vec<u8>) {
    buf.push(dc.species);
    push_u16_be(buf, dc.hp);
    buf.push(dc.box_level);
    buf.push(dc.status);
    buf.push(dc.type1);
    buf.push(dc.type2);
    buf.push(dc.catch_rate);
    for &m in &dc.moves {
        buf.push(m);
    }
    push_u16_be(buf, dc.ot_id);
    buf.push(((dc.exp >> 16) & 0xFF) as u8);
    buf.push(((dc.exp >> 8) & 0xFF) as u8);
    buf.push((dc.exp & 0xFF) as u8);
    push_u16_be(buf, dc.hp_exp);
    push_u16_be(buf, dc.attack_exp);
    push_u16_be(buf, dc.defense_exp);
    push_u16_be(buf, dc.speed_exp);
    push_u16_be(buf, dc.special_exp);
    buf.push((dc.dvs >> 8) as u8);
    buf.push((dc.dvs & 0xFF) as u8);
    for &p in &dc.pp {
        buf.push(p);
    }
}
