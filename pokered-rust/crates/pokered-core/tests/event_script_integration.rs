//! M9.3 Integration tests — Event flags + Script engine cross-module scenarios.
//!
//! These tests verify that the event flag system and script engine work together
//! correctly across multi-step game scenarios (e.g., setting flags via scripts,
//! checking flags to gate progression, coord event triggers advancing scripts).

use pokered_core::overworld::event_flags::EventFlags;
use pokered_core::overworld::map_scripts::{
    classify_map_script_type, get_map_trainer_count, MapScriptContext, MapScriptType,
};
use pokered_core::overworld::script_engine::{
    check_coord_events, execute_next_action, CoordEvent, MapScriptState, NpcMovementStep,
    ScriptAction, ScriptPhase, ScriptResult,
};
use pokered_data::event_flags::EventFlag;
use pokered_data::maps::MapId;
use pokered_data::species::Species;

// ── Event Flag Lifecycle ─────────────────────────────────────────────

#[test]
fn event_flags_gym_badge_progression() {
    // Simulate beating all 8 gyms in order — flags should be independent
    let mut flags = EventFlags::new();
    let gym_flags = [
        EventFlag::EVENT_BEAT_BROCK,
        EventFlag::EVENT_BEAT_MISTY,
        EventFlag::EVENT_BEAT_LT_SURGE,
        EventFlag::EVENT_BEAT_ERIKA,
        EventFlag::EVENT_BEAT_KOGA,
        EventFlag::EVENT_BEAT_SABRINA,
        EventFlag::EVENT_BEAT_BLAINE,
    ];

    // None set initially
    for flag in &gym_flags {
        assert!(!flags.check(*flag), "flag should not be set initially");
    }

    // Set them one by one and verify only the set ones are active
    for (i, flag) in gym_flags.iter().enumerate() {
        flags.set(*flag);
        for (j, check_flag) in gym_flags.iter().enumerate() {
            if j <= i {
                assert!(
                    flags.check(*check_flag),
                    "flag {j} should be set after setting up to {i}"
                );
            } else {
                assert!(!flags.check(*check_flag), "flag {j} should not be set yet");
            }
        }
    }

    assert!(flags.count_set() >= 7, "at least 7 flags should be set");
}

#[test]
fn event_flags_set_reset_idempotent() {
    let mut flags = EventFlags::new();
    let flag = EventFlag::EVENT_GOT_STARTER;

    // Set twice should be same as set once
    flags.set(flag);
    flags.set(flag);
    assert!(flags.check(flag));

    // Reset twice should be same as reset once
    flags.reset(flag);
    assert!(!flags.check(flag));
    flags.reset(flag);
    assert!(!flags.check(flag));
}

#[test]
fn event_flags_clear_all_resets_everything() {
    let mut flags = EventFlags::new();
    flags.set(EventFlag::EVENT_BEAT_BROCK);
    flags.set(EventFlag::EVENT_GOT_STARTER);
    flags.set(EventFlag::EVENT_BEAT_MISTY);
    assert!(flags.count_set() >= 3);

    flags.clear_all();
    assert_eq!(flags.count_set(), 0);
    assert!(!flags.check(EventFlag::EVENT_BEAT_BROCK));
    assert!(!flags.check(EventFlag::EVENT_GOT_STARTER));
}

#[test]
fn event_flags_serialization_roundtrip() {
    let mut flags = EventFlags::new();
    flags.set(EventFlag::EVENT_BEAT_BROCK);
    flags.set(EventFlag::EVENT_GOT_POKEDEX);
    flags.set(EventFlag::EVENT_BEAT_CHAMPION_RIVAL);

    let bytes = *flags.as_bytes();
    let restored = EventFlags::from_bytes(bytes);

    assert!(restored.check(EventFlag::EVENT_BEAT_BROCK));
    assert!(restored.check(EventFlag::EVENT_GOT_POKEDEX));
    assert!(restored.check(EventFlag::EVENT_BEAT_CHAMPION_RIVAL));
    assert!(!restored.check(EventFlag::EVENT_BEAT_MISTY));
    assert_eq!(flags.count_set(), restored.count_set());
}

#[test]
fn event_flags_raw_bit_access_matches_named() {
    let mut flags = EventFlags::new();
    let flag = EventFlag::EVENT_GOT_STARTER;
    let raw_bit = flag.bit_index();

    flags.set(flag);
    assert!(
        flags.check_raw(raw_bit),
        "raw check should match named check"
    );

    flags.reset(flag);
    assert!(!flags.check_raw(raw_bit));

    // Set via raw, check via named
    flags.set_raw(raw_bit);
    assert!(flags.check(flag), "named check should match raw set");
}

// ── Script Engine Execution Sequences ────────────────────────────────

#[test]
fn script_multi_action_sequence_executes_in_order() {
    let mut state = MapScriptState::new(MapId::OaksLab);
    state.queue_actions(vec![
        ScriptAction::DisplayText { text_id: 1 },
        ScriptAction::GiveItem {
            item_id: 4,
            quantity: 1,
        },
        ScriptAction::SetEventFlag {
            flag: EventFlag::EVENT_GOT_STARTER,
        },
        ScriptAction::EndScript,
    ]);

    // Action 1: DisplayText → waits
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::WaitForText);
    assert_eq!(state.phase, ScriptPhase::WaitingForText);

    // Signal text done
    state.signal_text_done();
    assert_eq!(state.phase, ScriptPhase::Executing);

    // Action 2: GiveItem → continues
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::Continue);

    // Action 3: SetEventFlag → continues
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::Continue);

    // Action 4: EndScript
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::ScriptEnded);
    assert_eq!(state.phase, ScriptPhase::Idle);
}

#[test]
fn script_npc_movement_waits_for_signal() {
    use pokered_core::overworld::Direction;

    let mut state = MapScriptState::new(MapId::OaksLab);
    state.queue_actions(vec![
        ScriptAction::MoveNpc {
            npc_index: 0,
            movements: vec![
                NpcMovementStep::Walk(Direction::Down),
                NpcMovementStep::Walk(Direction::Right),
            ],
        },
        ScriptAction::DisplayText { text_id: 5 },
    ]);

    // Movement action → waits
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::WaitForMovement);
    assert_eq!(state.phase, ScriptPhase::WaitingForMovement);

    // Signal movement done
    state.signal_movement_done();
    assert_eq!(state.phase, ScriptPhase::Executing);

    // Next action: text
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::WaitForText);
}

#[test]
fn script_trainer_battle_waits_for_signal() {
    let mut state = MapScriptState::new(MapId::PewterGym);
    state.queue_actions(vec![
        ScriptAction::StartTrainerBattle { trainer_index: 0 },
        ScriptAction::SetEventFlag {
            flag: EventFlag::EVENT_BEAT_BROCK,
        },
        ScriptAction::EndScript,
    ]);

    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::WaitForBattle);
    assert_eq!(state.phase, ScriptPhase::WaitingForBattle);

    state.signal_battle_done();
    assert_eq!(state.phase, ScriptPhase::Executing);

    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::Continue);

    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::ScriptEnded);
}

#[test]
fn script_delay_ticks_down_to_zero() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![
        ScriptAction::Delay { frames: 3 },
        ScriptAction::EndScript,
    ]);

    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::WaitForDelay { frames: 3 });
    assert!(matches!(
        state.phase,
        ScriptPhase::WaitingForDelay {
            frames_remaining: 3
        }
    ));

    // Tick 3 times
    state.tick_delay();
    assert!(matches!(
        state.phase,
        ScriptPhase::WaitingForDelay {
            frames_remaining: 2
        }
    ));
    state.tick_delay();
    assert!(matches!(
        state.phase,
        ScriptPhase::WaitingForDelay {
            frames_remaining: 1
        }
    ));
    state.tick_delay();
    // Should resume executing
    assert_eq!(state.phase, ScriptPhase::Executing);

    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::ScriptEnded);
}

#[test]
fn script_set_map_script_changes_current() {
    let mut state = MapScriptState::new(MapId::CeruleanCity);
    state.queue_actions(vec![ScriptAction::SetMapScript { script_index: 3 }]);

    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::ScriptChanged { new_index: 3 });
    assert_eq!(state.current_script, 3);
    // SetMapScript clears the queue
    assert!(!state.is_active());
}

#[test]
fn script_empty_queue_returns_ended() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![]);

    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::ScriptEnded);
}

#[test]
fn script_set_script_resets_state() {
    let mut state = MapScriptState::new(MapId::OaksLab);
    state.queue_actions(vec![
        ScriptAction::DisplayText { text_id: 1 },
        ScriptAction::DisplayText { text_id: 2 },
    ]);

    // Execute first action
    execute_next_action(&mut state);
    assert!(state.is_active());

    // set_script resets everything
    state.set_script(5);
    assert_eq!(state.current_script, 5);
    assert!(!state.is_active());
    assert_eq!(state.phase, ScriptPhase::Idle);
    assert!(state.action_queue.is_empty());
}

// ── Coord Event Integration ──────────────────────────────────────────

#[test]
fn coord_events_trigger_at_exact_position() {
    let events = vec![
        CoordEvent::new(5, 3, 1),
        CoordEvent::new(10, 7, 2),
        CoordEvent::new(5, 8, 3),
    ];

    // Exact match
    assert_eq!(check_coord_events(&events, 5, 3), Some(1));
    assert_eq!(check_coord_events(&events, 10, 7), Some(2));

    // Off by one — no match
    assert_eq!(check_coord_events(&events, 5, 4), None);
    assert_eq!(check_coord_events(&events, 6, 3), None);
    assert_eq!(check_coord_events(&events, 0, 0), None);
}

#[test]
fn coord_events_first_match_wins() {
    let events = vec![
        CoordEvent::new(5, 5, 10),
        CoordEvent::new(5, 5, 20), // same position, different script
    ];

    // First one wins
    assert_eq!(check_coord_events(&events, 5, 5), Some(10));
}

#[test]
fn map_script_context_coord_trigger_integration() {
    let ctx = MapScriptContext::new(MapId::PalletTown)
        .with_coord_events(vec![CoordEvent::new(3, 10, 1), CoordEvent::new(5, 10, 2)]);

    assert_eq!(ctx.check_coord_trigger(3, 10), Some(1));
    assert_eq!(ctx.check_coord_trigger(5, 10), Some(2));
    assert_eq!(ctx.check_coord_trigger(4, 10), None);
}

// ── Map Script Classification ────────────────────────────────────────

#[test]
fn story_sequence_maps_classified_correctly() {
    // Known story sequence maps
    assert_eq!(
        classify_map_script_type(MapId::OaksLab),
        MapScriptType::StorySequence
    );
    assert_eq!(
        classify_map_script_type(MapId::PalletTown),
        MapScriptType::StorySequence
    );
    assert_eq!(
        classify_map_script_type(MapId::CeruleanCity),
        MapScriptType::StorySequence
    );
}

#[test]
fn trainer_maps_classified_correctly() {
    // Gyms should have trainers
    let pewter_type = classify_map_script_type(MapId::PewterGym);
    assert!(
        pewter_type == MapScriptType::TrainerMap || pewter_type == MapScriptType::StorySequence,
        "Gym should be TrainerMap or StorySequence, got {pewter_type:?}"
    );
}

#[test]
fn simple_text_maps_classified_correctly() {
    assert_eq!(
        classify_map_script_type(MapId::Route1),
        MapScriptType::SimpleTextOnly
    );
    assert_eq!(
        classify_map_script_type(MapId::ViridianPokecenter),
        MapScriptType::SimpleTextOnly
    );
}

#[test]
fn map_trainer_count_matches_headers() {
    // Maps with known trainers
    let pewter_count = get_map_trainer_count(MapId::PewterGym);
    assert!(pewter_count > 0, "Pewter Gym should have trainers");

    // Route 1 has no trainers
    let route1_count = get_map_trainer_count(MapId::Route1);
    assert_eq!(route1_count, 0, "Route 1 should have no trainers");
}

// ── Script + Event Flag Combined Scenario ────────────────────────────

#[test]
fn oak_lab_starter_scenario_flag_gating() {
    // Simulate: Player hasn't gotten starter → script runs → gets starter → flag set
    let mut flags = EventFlags::new();
    let mut script = MapScriptState::new(MapId::OaksLab);

    // Before getting starter
    assert!(!flags.check(EventFlag::EVENT_GOT_STARTER));

    // Script queues: talk to Oak, give Charmander, set flag
    script.queue_actions(vec![
        ScriptAction::DisplayText { text_id: 1 },
        ScriptAction::GivePokemon {
            species: Species::Charmander,
            level: 5,
        },
        ScriptAction::SetEventFlag {
            flag: EventFlag::EVENT_GOT_STARTER,
        },
        ScriptAction::EndScript,
    ]);

    // Execute all actions
    let r1 = execute_next_action(&mut script);
    assert_eq!(r1, ScriptResult::WaitForText);
    script.signal_text_done();

    let r2 = execute_next_action(&mut script);
    assert_eq!(r2, ScriptResult::Continue); // GivePokemon

    let r3 = execute_next_action(&mut script);
    assert_eq!(r3, ScriptResult::Continue); // SetEventFlag

    // The script engine returns Continue for SetEventFlag — the caller
    // must actually apply the flag. Simulate that:
    flags.set(EventFlag::EVENT_GOT_STARTER);

    let r4 = execute_next_action(&mut script);
    assert_eq!(r4, ScriptResult::ScriptEnded);

    // Now flag is set
    assert!(flags.check(EventFlag::EVENT_GOT_STARTER));
}

#[test]
fn rival_battle_then_pokedex_scenario() {
    let mut flags = EventFlags::new();

    // Step 1: Got starter
    flags.set(EventFlag::EVENT_GOT_STARTER);

    // Step 2: Rival battle in Oak's lab
    let mut script = MapScriptState::new(MapId::OaksLab);
    script.queue_actions(vec![
        ScriptAction::StartTrainerBattle { trainer_index: 0 },
        ScriptAction::SetEventFlag {
            flag: EventFlag::EVENT_BATTLED_RIVAL_IN_OAKS_LAB,
        },
        ScriptAction::EndScript,
    ]);

    let r1 = execute_next_action(&mut script);
    assert_eq!(r1, ScriptResult::WaitForBattle);
    script.signal_battle_done();

    let r2 = execute_next_action(&mut script);
    assert_eq!(r2, ScriptResult::Continue);
    flags.set(EventFlag::EVENT_BATTLED_RIVAL_IN_OAKS_LAB);

    let r3 = execute_next_action(&mut script);
    assert_eq!(r3, ScriptResult::ScriptEnded);

    // Step 3: Get Pokédex
    flags.set(EventFlag::EVENT_GOT_POKEDEX);

    // All three progression flags set
    assert!(flags.check(EventFlag::EVENT_GOT_STARTER));
    assert!(flags.check(EventFlag::EVENT_BATTLED_RIVAL_IN_OAKS_LAB));
    assert!(flags.check(EventFlag::EVENT_GOT_POKEDEX));
}

#[test]
fn script_face_player_and_joy_ignore_are_instant() {
    use pokered_core::overworld::Direction;

    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![
        ScriptAction::FacePlayer {
            direction: Direction::Up,
        },
        ScriptAction::SetJoyIgnore { mask: 0xFF },
        ScriptAction::PlayMusic { music_id: 1 },
        ScriptAction::ClearJoyIgnore,
        ScriptAction::EndScript,
    ]);

    // All should return Continue (instant)
    assert_eq!(execute_next_action(&mut state), ScriptResult::Continue);
    assert_eq!(execute_next_action(&mut state), ScriptResult::Continue);
    assert_eq!(execute_next_action(&mut state), ScriptResult::Continue);
    assert_eq!(execute_next_action(&mut state), ScriptResult::Continue);
    assert_eq!(execute_next_action(&mut state), ScriptResult::ScriptEnded);
}
