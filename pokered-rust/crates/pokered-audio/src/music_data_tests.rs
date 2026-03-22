use crate::music_data::*;

#[test]
fn test_music_id_count() {
    assert_eq!(NUM_MUSIC_TRACKS, 45);
}

#[test]
fn test_music_id_from_u8_valid() {
    assert_eq!(MusicId::from_u8(0), Some(MusicId::PALLET_TOWN));
    assert_eq!(MusicId::from_u8(23), Some(MusicId::FINAL_BATTLE));
    assert_eq!(MusicId::from_u8(31), Some(MusicId::JIGGLYPUFF_SONG));
    assert_eq!(MusicId::from_u8(44), Some(MusicId::MEET_MALE_TRAINER));
}

#[test]
fn test_music_id_from_u8_invalid() {
    assert_eq!(MusicId::from_u8(45), None);
    assert_eq!(MusicId::from_u8(255), None);
}

#[test]
fn test_all_tracks_have_at_least_one_channel() {
    for track in MUSIC_TRACKS.iter() {
        let has_channel = track.channels.iter().any(|ch| ch.is_some());
        assert!(has_channel, "Track {:?} has no channel data", track.id);
    }
}

#[test]
fn test_all_channel_data_non_empty() {
    for track in MUSIC_TRACKS.iter() {
        for (i, ch) in track.channels.iter().enumerate() {
            if let Some(data) = ch {
                assert!(
                    !data.is_empty(),
                    "Track {:?} ch{} is empty",
                    track.id,
                    i + 1
                );
            }
        }
    }
}

#[test]
fn test_all_channel_data_ends_with_loop_or_ret() {
    for track in MUSIC_TRACKS.iter() {
        for (i, ch) in track.channels.iter().enumerate() {
            if let Some(data) = ch {
                let last = data[data.len() - 1];
                let valid_end = last == 0xFF  // sound_ret
                    || last == 0xFE           // part of sound_loop (rare, last byte is hi addr)
                    || data.len() >= 4 && data[data.len() - 4] == 0xFE; // sound_loop: FE cnt lo hi
                assert!(
                    valid_end || (data.len() >= 3 && data[data.len() - 3] == 0xFD), // sound_call as tail
                    "Track {:?} ch{} ends with 0x{:02X}, expected sound_ret/sound_loop",
                    track.id,
                    i + 1,
                    last
                );
            }
        }
    }
}

#[test]
fn test_pallet_town_structure() {
    let track = get_music_track(MusicId::PALLET_TOWN);
    assert_eq!(track.num_channels, 3);
    assert!(track.channels[0].is_some());
    assert!(track.channels[1].is_some());
    assert!(track.channels[2].is_some());
    assert!(track.channels[3].is_none());
    assert!(track.tempo > 0);
}

#[test]
fn test_jigglypuff_two_channels() {
    let track = get_music_track(MusicId::JIGGLYPUFF_SONG);
    assert_eq!(track.num_channels, 2);
    assert!(track.channels[0].is_some());
    assert!(track.channels[1].is_some());
    assert!(track.channels[2].is_none());
    assert!(track.channels[3].is_none());
}

#[test]
fn test_final_battle_has_all_channels_parsed() {
    let track = get_music_track(MusicId::FINAL_BATTLE);
    assert!(
        track.channels[0].is_some(),
        "Ch1 should be present (cross-channel ref test)"
    );
    assert!(track.channels[1].is_some());
    assert!(track.channels[2].is_some());
    let ch1_data = track.channels[0].unwrap();
    assert!(
        ch1_data.len() > 100,
        "Ch1 should have substantial data, got {}",
        ch1_data.len()
    );
}

#[test]
fn test_battle_tracks_have_four_channels() {
    for id in [
        MusicId::TITLE_SCREEN,
        MusicId::INTRO_BATTLE,
        MusicId::DUNGEON1,
    ] {
        let track = get_music_track(id);
        assert_eq!(
            track.num_channels, 4,
            "Track {:?} should have 4 channels",
            id
        );
        for (i, ch) in track.channels.iter().enumerate() {
            assert!(ch.is_some(), "Track {:?} ch{} should be present", id, i + 1);
        }
    }
}

#[test]
fn test_tempos_are_reasonable() {
    for track in MUSIC_TRACKS.iter() {
        assert!(track.tempo > 0, "Track {:?} has zero tempo", track.id);
    }
}

#[test]
fn test_pallet_town_ch1_starts_with_tempo_command() {
    let track = get_music_track(MusicId::PALLET_TOWN);
    let ch1 = track.channels[0].unwrap();
    assert_eq!(ch1[0], 0xED, "First byte should be tempo command (0xED)");
}

#[test]
fn test_channel_data_contains_notes() {
    let track = get_music_track(MusicId::PALLET_TOWN);
    let ch1 = track.channels[0].unwrap();
    let has_notes = ch1.iter().any(|&b| b < 0xB0);
    assert!(has_notes, "Channel data should contain note bytes (< 0xB0)");
}

#[test]
fn test_get_music_track_all_ids() {
    for i in 0..NUM_MUSIC_TRACKS {
        let id = MusicId::from_u8(i as u8).unwrap();
        let track = get_music_track(id);
        assert_eq!(track.id, id);
    }
}

#[test]
fn test_sequencer_can_load_pallet_town() {
    use crate::apu::Apu;
    use crate::sequencer::Sequencer;

    let track = get_music_track(MusicId::PALLET_TOWN);
    let channel_data: Vec<Vec<u8>> = track
        .channels
        .iter()
        .filter_map(|ch| ch.map(|d| d.to_vec()))
        .collect();

    let mut seq = Sequencer::new();
    let mut apu = Apu::new();
    seq.play_music(0, &channel_data, track.tempo);

    for _ in 0..100 {
        seq.update_frame(&mut apu);
    }
}

#[test]
fn test_sequencer_can_load_final_battle() {
    use crate::apu::Apu;
    use crate::sequencer::Sequencer;

    let track = get_music_track(MusicId::FINAL_BATTLE);
    let channel_data: Vec<Vec<u8>> = track
        .channels
        .iter()
        .filter_map(|ch| ch.map(|d| d.to_vec()))
        .collect();

    let mut seq = Sequencer::new();
    let mut apu = Apu::new();
    seq.play_music(0, &channel_data, track.tempo);

    for _ in 0..200 {
        seq.update_frame(&mut apu);
    }
}
