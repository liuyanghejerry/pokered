use crate::battle::state::{Pokemon, StatusCondition};
use crate::pokemon::party::Party;
use crate::pokemon::pc_box::PcBox;
use crate::save::game_data::{GameData, PlayTime, GAME_PROGRESS_FLAGS_SIZE, NUM_EVENTS_BYTES};
use crate::save::hall_of_fame::{HallOfFame, HofMon, HofTeam, HOF_TEAM_CAPACITY};
use crate::save::ser_pokemon::*;
use crate::save::serialization::*;
use crate::save::SaveData;
use crate::save_menu::calc_checksum;
use pokered_data::moves::MoveId;
use pokered_data::species::Species;
use pokered_data::types::PokemonType;

fn make_test_pokemon(species: Species, level: u8) -> Pokemon {
    Pokemon {
        species,
        level,
        hp: 100,
        max_hp: 100,
        attack: 50,
        defense: 40,
        speed: 60,
        special: 55,
        type1: PokemonType::Normal,
        type2: PokemonType::Normal,
        moves: [MoveId::Tackle, MoveId::None, MoveId::None, MoveId::None],
        pp: [35, 0, 0, 0],
        pp_ups: [0; 4],
        status: StatusCondition::None,
        dv_bytes: [0xAB, 0xCD],
        stat_exp: [100, 200, 300, 400, 500],
        total_exp: 1000,
        is_traded: false,
    }
}

#[test]
fn test_save_data_new_defaults() {
    let save = SaveData::new();
    assert!(save.player_name.is_empty());
    assert_eq!(save.tile_animations, 0);
    assert_eq!(save.game_data.player_money, 0);
    assert_eq!(save.game_data.obtained_badges, 0);
    assert_eq!(save.party.count(), 0);
    assert_eq!(save.hall_of_fame.team_count(), 0);
}

#[test]
fn test_save_data_clear() {
    let mut save = SaveData::new();
    save.player_name = vec![0x80, 0x81, 0x82];
    save.game_data.player_money = 99999;
    save.game_data.obtained_badges = 0xFF;
    save.tile_animations = 5;
    save.clear();
    assert!(save.player_name.is_empty());
    assert_eq!(save.game_data.player_money, 0);
    assert_eq!(save.game_data.obtained_badges, 0);
    assert_eq!(save.tile_animations, 0);
}

#[test]
fn test_checksum_empty_save() {
    let save = SaveData::new();
    let cksum = save.compute_checksum();
    assert!(save.validate_checksum(cksum));
}

#[test]
fn test_checksum_changes_with_data() {
    let mut save = SaveData::new();
    let cksum1 = save.compute_checksum();
    save.game_data.player_money = 50000;
    let cksum2 = save.compute_checksum();
    assert_ne!(cksum1, cksum2);
}

#[test]
fn test_checksum_validate_bad() {
    let save = SaveData::new();
    let cksum = save.compute_checksum();
    assert!(!save.validate_checksum(cksum.wrapping_add(1)));
}

#[test]
fn test_calc_checksum_matches_asm() {
    let data = [0x01, 0x02, 0x03, 0x04];
    let sum: u8 = data.iter().fold(0u8, |a, &b| a.wrapping_add(b));
    let expected = !sum;
    assert_eq!(calc_checksum(&data), expected);
}

#[test]
fn test_status_to_byte_roundtrip() {
    let cases = vec![
        StatusCondition::None,
        StatusCondition::Sleep(3),
        StatusCondition::Poison,
        StatusCondition::Burn,
        StatusCondition::Freeze,
        StatusCondition::Paralysis,
    ];
    for status in cases {
        let b = status_to_byte(&status);
        let back = byte_to_status(b);
        assert_eq!(status, back);
    }
}

#[test]
fn test_serialize_box_mon_size() {
    let mon = make_test_pokemon(Species::Pikachu, 25);
    let mut buf = Vec::new();
    serialize_box_mon(&mon, &mut buf);
    assert_eq!(buf.len(), BOX_STRUCT_SIZE);
}

#[test]
fn test_serialize_party_mon_size() {
    let mon = make_test_pokemon(Species::Pikachu, 25);
    let mut buf = Vec::new();
    serialize_party_mon(&mon, &mut buf);
    assert_eq!(buf.len(), PARTY_STRUCT_SIZE);
}

#[test]
fn test_deserialize_box_mon_roundtrip() {
    let mon = make_test_pokemon(Species::Charmander, 16);
    let mut buf = Vec::new();
    serialize_box_mon(&mon, &mut buf);
    let restored = deserialize_box_mon(&buf).unwrap();
    assert_eq!(restored.species, mon.species);
    assert_eq!(restored.level, mon.level);
    assert_eq!(restored.hp, mon.hp);
    assert_eq!(restored.total_exp, mon.total_exp);
    assert_eq!(restored.dv_bytes, mon.dv_bytes);
    assert_eq!(restored.moves[0], mon.moves[0]);
}

#[test]
fn test_deserialize_party_mon_roundtrip() {
    let mon = make_test_pokemon(Species::Blastoise, 36);
    let mut buf = Vec::new();
    serialize_party_mon(&mon, &mut buf);
    let restored = deserialize_party_mon(&buf).unwrap();
    assert_eq!(restored.species, mon.species);
    assert_eq!(restored.level, mon.level);
    assert_eq!(restored.max_hp, mon.max_hp);
    assert_eq!(restored.attack, mon.attack);
    assert_eq!(restored.defense, mon.defense);
    assert_eq!(restored.speed, mon.speed);
    assert_eq!(restored.special, mon.special);
}

#[test]
fn test_deserialize_box_mon_too_short() {
    let buf = [0u8; BOX_STRUCT_SIZE - 1];
    assert_eq!(deserialize_box_mon(&buf), Err(SaveError::DataTooShort));
}

#[test]
fn test_deserialize_party_mon_too_short() {
    let buf = [0u8; PARTY_STRUCT_SIZE - 1];
    assert_eq!(deserialize_party_mon(&buf), Err(SaveError::DataTooShort));
}

#[test]
fn test_serialize_name_padding() {
    let mut buf = Vec::new();
    let name = vec![0x80, 0x81, 0x82];
    serialize_name(&name, &mut buf);
    assert_eq!(buf.len(), 11);
    assert_eq!(buf[0], 0x80);
    assert_eq!(buf[1], 0x81);
    assert_eq!(buf[2], 0x82);
    assert_eq!(buf[3], 0x50);
    for i in 4..11 {
        assert_eq!(buf[i], 0x50);
    }
}

#[test]
fn test_serialize_name_empty() {
    let mut buf = Vec::new();
    serialize_name(&[], &mut buf);
    assert_eq!(buf.len(), 11);
    assert_eq!(buf[0], 0x50);
}

#[test]
fn test_deserialize_name_with_terminator() {
    let data = [
        0x80, 0x81, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let name = deserialize_name(&data);
    assert_eq!(name, vec![0x80, 0x81]);
}

#[test]
fn test_serialize_sprite_data_size() {
    let mut buf = Vec::new();
    serialize_sprite_data_into(&mut buf);
    assert_eq!(buf.len(), SPRITE_DATA_SIZE);
    assert!(buf.iter().all(|&b| b == 0));
}

#[test]
fn test_serialize_party_empty() {
    let party = Party::new();
    let mut buf = Vec::new();
    serialize_party_into(&party, &mut buf);
    assert_eq!(buf[0], 0);
    assert_eq!(buf[1], 0xFF);
}

#[test]
fn test_serialize_box_empty() {
    let box_data = PcBox::new();
    let mut buf = Vec::new();
    serialize_box_into(&box_data, &mut buf);
    assert_eq!(buf[0], 0);
    assert_eq!(buf[1], 0xFF);
}

#[test]
fn test_game_data_badge_operations() {
    let mut gd = GameData::new();
    assert_eq!(gd.badge_count(), 0);
    assert!(!gd.has_badge(0));
    gd.set_badge(0);
    assert!(gd.has_badge(0));
    assert_eq!(gd.badge_count(), 1);
    gd.set_badge(7);
    assert!(gd.has_badge(7));
    assert_eq!(gd.badge_count(), 2);
}

#[test]
fn test_game_data_badge_out_of_range() {
    let mut gd = GameData::new();
    gd.set_badge(8);
    assert!(!gd.has_badge(8));
    assert_eq!(gd.badge_count(), 0);
}

#[test]
fn test_play_time_total_seconds() {
    let pt = PlayTime {
        hours: 1,
        maxed: false,
        minutes: 30,
        seconds: 45,
        frames: 0,
    };
    assert_eq!(pt.total_seconds(), 3600 + 1800 + 45);
}

#[test]
fn test_hof_push_team() {
    let mut hof = HallOfFame::new();
    assert_eq!(hof.team_count(), 0);
    let mut team = HofTeam::new();
    team.add_mon(HofMon::new(25, 50, vec![0x80, 0x81]));
    hof.push_team(team);
    assert_eq!(hof.team_count(), 1);
    assert_eq!(hof.get_team(0).unwrap().mons.len(), 1);
}

#[test]
fn test_hof_capacity_evicts_oldest() {
    let mut hof = HallOfFame::new();
    for i in 0..HOF_TEAM_CAPACITY + 5 {
        let mut team = HofTeam::new();
        team.add_mon(HofMon::new(i as u8, 50, vec![]));
        hof.push_team(team);
    }
    assert_eq!(hof.team_count(), HOF_TEAM_CAPACITY);
    let first = hof.get_team(0).unwrap();
    assert_eq!(first.mons[0].species, 5);
}

#[test]
fn test_hof_clear() {
    let mut hof = HallOfFame::new();
    let team = HofTeam::new();
    hof.push_team(team);
    hof.clear();
    assert_eq!(hof.team_count(), 0);
}

#[test]
fn test_hof_team_max_mons() {
    let mut team = HofTeam::new();
    for i in 0..10 {
        team.add_mon(HofMon::new(i, 50, vec![]));
    }
    assert_eq!(team.mons.len(), 6);
}

#[test]
fn test_save_error_display() {
    assert_eq!(
        format!("{}", SaveError::DataTooShort),
        "save data too short"
    );
    assert_eq!(format!("{}", SaveError::BadChecksum), "bad checksum");
    assert_eq!(format!("{}", SaveError::InvalidData), "invalid save data");
}

#[test]
fn test_species_from_index_id() {
    assert_eq!(Species::from_index_id(25), Species::Pikachu);
    assert_eq!(Species::from_index_id(0), Species::None);
    assert_eq!(Species::from_index_id(255), Species::None);
}

#[test]
fn test_pokemon_type_from_id() {
    assert_eq!(PokemonType::from_id(0x00), PokemonType::Normal);
    assert_eq!(PokemonType::from_id(0x14), PokemonType::Fire);
    assert_eq!(PokemonType::from_id(0xFF), PokemonType::Normal);
}

#[test]
fn test_move_id_from_id() {
    assert_eq!(MoveId::from_id(0x01), MoveId::Pound);
    assert_eq!(MoveId::from_id(0x00), MoveId::None);
    assert_eq!(MoveId::from_id(0xFF), MoveId::None);
}

#[test]
fn test_game_data_serialization_deterministic() {
    let gd = GameData::new();
    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();
    gd.serialize_into(&mut buf1);
    gd.serialize_into(&mut buf2);
    assert_eq!(buf1, buf2);
}

#[test]
fn test_save_data_default_eq_new() {
    let a = SaveData::new();
    let b = SaveData::default();
    assert_eq!(a.compute_checksum(), b.compute_checksum());
}

#[test]
fn test_game_data_event_flags_size() {
    let gd = GameData::new();
    assert_eq!(gd.event_flags.len(), NUM_EVENTS_BYTES);
    assert_eq!(gd.game_progress_flags.len(), GAME_PROGRESS_FLAGS_SIZE);
}

#[test]
fn test_position_sram_roundtrip() {
    use crate::save::game_data::MapPosition;
    use crate::save::sram_export::export_sram;
    use crate::save::sram_import::import_sram;

    let mut save = SaveData::new();
    save.player_name = vec![0x80, 0x81, 0x82, 0x50];

    save.game_data.position = MapPosition {
        map_id: 0,
        y: 5,
        x: 3,
        y_block: 1,
        x_block: 1,
    };
    save.game_data.player_direction = 4;
    save.game_data.player_last_stop_direction = 4;
    save.game_data.player_moving_direction = 4;
    save.game_data.current_map_height2 = 18;
    save.game_data.current_map_width2 = 20;

    let sram = export_sram(&save);
    let restored = import_sram(&sram).expect("roundtrip import should succeed");

    assert_eq!(restored.game_data.position.map_id, 0, "map_id mismatch");
    assert_eq!(restored.game_data.position.x, 3, "x mismatch");
    assert_eq!(restored.game_data.position.y, 5, "y mismatch");
    assert_eq!(restored.game_data.position.x_block, 1, "x_block mismatch");
    assert_eq!(restored.game_data.position.y_block, 1, "y_block mismatch");
    assert_eq!(restored.game_data.player_direction, 4, "direction mismatch");
}

#[test]
fn test_position_sram_roundtrip_route1() {
    use crate::save::game_data::MapPosition;
    use crate::save::sram_export::export_sram;
    use crate::save::sram_import::import_sram;

    let mut save = SaveData::new();
    save.player_name = vec![0x87, 0x84, 0x83, 0x50];

    save.game_data.position = MapPosition {
        map_id: 12,
        y: 27,
        x: 4,
        y_block: 1,
        x_block: 0,
    };
    save.game_data.player_direction = 0;
    save.game_data.current_map_height2 = 36;
    save.game_data.current_map_width2 = 20;

    let sram = export_sram(&save);
    let restored = import_sram(&sram).expect("roundtrip import should succeed");

    assert_eq!(restored.game_data.position.map_id, 12);
    assert_eq!(restored.game_data.position.x, 4);
    assert_eq!(restored.game_data.position.y, 27);
    assert_eq!(restored.game_data.player_direction, 0);
}
