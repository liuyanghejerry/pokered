use super::event_flags::EventFlags;
use super::map_scripts::*;
use super::script_engine::*;
use super::trainer_engine::*;
use super::Direction;
use pokered_data::event_flags::{EventFlag, EVENT_FLAGS_SIZE};
use pokered_data::maps::MapId;
use pokered_data::trainer_headers::get_trainer_headers;

// ══════════════════════════════════════════════════════════════════════
// EventFlags tests
// ══════════════════════════════════════════════════════════════════════

#[test]
fn event_flags_new_all_cleared() {
    let flags = EventFlags::new();
    assert_eq!(flags.count_set(), 0);
}

#[test]
fn event_flags_default_same_as_new() {
    let a = EventFlags::new();
    let b = EventFlags::default();
    assert_eq!(a.as_bytes(), b.as_bytes());
}

#[test]
fn event_flags_set_and_check() {
    let mut flags = EventFlags::new();
    let flag = EventFlag::EVENT_GOT_STARTER;
    assert!(!flags.check(flag));
    flags.set(flag);
    assert!(flags.check(flag));
}

#[test]
fn event_flags_reset() {
    let mut flags = EventFlags::new();
    let flag = EventFlag::EVENT_GOT_STARTER;
    flags.set(flag);
    assert!(flags.check(flag));
    flags.reset(flag);
    assert!(!flags.check(flag));
}

#[test]
fn event_flags_multiple_independent() {
    let mut flags = EventFlags::new();
    let f1 = EventFlag::EVENT_GOT_STARTER;
    let f2 = EventFlag::EVENT_BEAT_BROCK;
    flags.set(f1);
    assert!(flags.check(f1));
    assert!(!flags.check(f2));
    flags.set(f2);
    assert!(flags.check(f1));
    assert!(flags.check(f2));
    flags.reset(f1);
    assert!(!flags.check(f1));
    assert!(flags.check(f2));
}

#[test]
fn event_flags_count_set() {
    let mut flags = EventFlags::new();
    assert_eq!(flags.count_set(), 0);
    flags.set(EventFlag::EVENT_GOT_STARTER);
    assert_eq!(flags.count_set(), 1);
    flags.set(EventFlag::EVENT_BEAT_BROCK);
    assert_eq!(flags.count_set(), 2);
    flags.set(EventFlag::EVENT_BEAT_MISTY);
    assert_eq!(flags.count_set(), 3);
}

#[test]
fn event_flags_clear_all() {
    let mut flags = EventFlags::new();
    flags.set(EventFlag::EVENT_GOT_STARTER);
    flags.set(EventFlag::EVENT_BEAT_BROCK);
    flags.set(EventFlag::EVENT_BEAT_MISTY);
    assert_eq!(flags.count_set(), 3);
    flags.clear_all();
    assert_eq!(flags.count_set(), 0);
    assert!(!flags.check(EventFlag::EVENT_GOT_STARTER));
}

#[test]
fn event_flags_raw_operations() {
    let mut flags = EventFlags::new();
    // Bit 0
    assert!(!flags.check_raw(0));
    flags.set_raw(0);
    assert!(flags.check_raw(0));
    flags.reset_raw(0);
    assert!(!flags.check_raw(0));
    // High bit
    flags.set_raw(100);
    assert!(flags.check_raw(100));
    assert!(!flags.check_raw(99));
    assert!(!flags.check_raw(101));
}

#[test]
fn event_flags_raw_out_of_bounds_safe() {
    let mut flags = EventFlags::new();
    // Way out of bounds — should not panic
    flags.set_raw(0xFFFF);
    assert!(!flags.check_raw(0xFFFF));
    flags.reset_raw(0xFFFF);
}

#[test]
fn event_flags_save_load_roundtrip() {
    let mut flags = EventFlags::new();
    flags.set(EventFlag::EVENT_GOT_STARTER);
    flags.set(EventFlag::EVENT_BEAT_BROCK);
    flags.set(EventFlag::EVENT_BEAT_MISTY);
    let bytes = *flags.as_bytes();
    let loaded = EventFlags::from_bytes(bytes);
    assert!(loaded.check(EventFlag::EVENT_GOT_STARTER));
    assert!(loaded.check(EventFlag::EVENT_BEAT_BROCK));
    assert!(loaded.check(EventFlag::EVENT_BEAT_MISTY));
    assert!(!loaded.check(EventFlag::EVENT_BEAT_LT_SURGE));
    assert_eq!(loaded.count_set(), 3);
}

#[test]
fn event_flags_size_is_316() {
    assert_eq!(EVENT_FLAGS_SIZE, 316);
    let flags = EventFlags::new();
    assert_eq!(flags.as_bytes().len(), 316);
}

// ══════════════════════════════════════════════════════════════════════
// ScriptEngine tests
// ══════════════════════════════════════════════════════════════════════

#[test]
fn map_script_state_new_is_idle() {
    let state = MapScriptState::new(MapId::PalletTown);
    assert_eq!(state.map, MapId::PalletTown);
    assert_eq!(state.current_script, 0);
    assert_eq!(state.phase, ScriptPhase::Idle);
    assert!(state.action_queue.is_empty());
    assert!(!state.is_active());
}

#[test]
fn map_script_state_set_script_resets() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![ScriptAction::Heal, ScriptAction::EndScript]);
    assert!(state.is_active());
    state.set_script(3);
    assert_eq!(state.current_script, 3);
    assert_eq!(state.phase, ScriptPhase::Idle);
    assert!(state.action_queue.is_empty());
    assert_eq!(state.action_index, 0);
}

#[test]
fn map_script_state_queue_actions_starts_executing() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![ScriptAction::Heal]);
    assert_eq!(state.phase, ScriptPhase::Executing);
    assert!(state.is_active());
}

#[test]
fn map_script_state_queue_empty_stays_idle() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![]);
    assert_eq!(state.phase, ScriptPhase::Idle);
    assert!(!state.is_active());
}

#[test]
fn map_script_state_next_action_advances() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![
        ScriptAction::Heal,
        ScriptAction::StopMusic,
        ScriptAction::EndScript,
    ]);
    let a1 = state.next_action().cloned();
    assert_eq!(a1, Some(ScriptAction::Heal));
    let a2 = state.next_action().cloned();
    assert_eq!(a2, Some(ScriptAction::StopMusic));
    let a3 = state.next_action().cloned();
    assert_eq!(a3, Some(ScriptAction::EndScript));
    let a4 = state.next_action();
    assert!(a4.is_none());
    assert_eq!(state.phase, ScriptPhase::Idle);
}

#[test]
fn signal_text_done_resumes_execution() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.phase = ScriptPhase::WaitingForText;
    state.signal_text_done();
    assert_eq!(state.phase, ScriptPhase::Executing);
}

#[test]
fn signal_text_done_no_op_when_not_waiting() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.phase = ScriptPhase::Executing;
    state.signal_text_done();
    assert_eq!(state.phase, ScriptPhase::Executing);
}

#[test]
fn signal_movement_done_resumes() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.phase = ScriptPhase::WaitingForMovement;
    state.signal_movement_done();
    assert_eq!(state.phase, ScriptPhase::Executing);
}

#[test]
fn signal_battle_done_resumes() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.phase = ScriptPhase::WaitingForBattle;
    state.signal_battle_done();
    assert_eq!(state.phase, ScriptPhase::Executing);
}

#[test]
fn tick_delay_counts_down() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.phase = ScriptPhase::WaitingForDelay {
        frames_remaining: 3,
    };
    state.tick_delay();
    assert_eq!(
        state.phase,
        ScriptPhase::WaitingForDelay {
            frames_remaining: 2
        }
    );
    state.tick_delay();
    assert_eq!(
        state.phase,
        ScriptPhase::WaitingForDelay {
            frames_remaining: 1
        }
    );
    state.tick_delay();
    assert_eq!(state.phase, ScriptPhase::Executing);
}

#[test]
fn tick_delay_no_op_when_idle() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.tick_delay(); // Should not panic
    assert_eq!(state.phase, ScriptPhase::Idle);
}

#[test]
fn execute_next_action_display_text_waits() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![ScriptAction::DisplayText { text_id: 1 }]);
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::WaitForText);
    assert_eq!(state.phase, ScriptPhase::WaitingForText);
}

#[test]
fn execute_next_action_move_npc_waits() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![ScriptAction::MoveNpc {
        npc_index: 0,
        movements: vec![NpcMovementStep::Walk(Direction::Down)],
    }]);
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::WaitForMovement);
    assert_eq!(state.phase, ScriptPhase::WaitingForMovement);
}

#[test]
fn execute_next_action_start_battle_waits() {
    let mut state = MapScriptState::new(MapId::PewterGym);
    state.queue_actions(vec![ScriptAction::StartTrainerBattle { trainer_index: 0 }]);
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::WaitForBattle);
    assert_eq!(state.phase, ScriptPhase::WaitingForBattle);
}

#[test]
fn execute_next_action_delay_sets_frames() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![ScriptAction::Delay { frames: 10 }]);
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::WaitForDelay { frames: 10 });
    assert_eq!(
        state.phase,
        ScriptPhase::WaitingForDelay {
            frames_remaining: 10
        }
    );
}

#[test]
fn execute_next_action_set_map_script_changes_script() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![ScriptAction::SetMapScript { script_index: 5 }]);
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::ScriptChanged { new_index: 5 });
    assert_eq!(state.current_script, 5);
}

#[test]
fn execute_next_action_end_script() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![ScriptAction::EndScript]);
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::ScriptEnded);
    assert_eq!(state.phase, ScriptPhase::Idle);
}

#[test]
fn execute_next_action_heal_continues() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![ScriptAction::Heal, ScriptAction::EndScript]);
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::Continue);
}

#[test]
fn execute_next_action_empty_queue_ends() {
    let mut state = MapScriptState::new(MapId::PalletTown);
    state.queue_actions(vec![]);
    let result = execute_next_action(&mut state);
    assert_eq!(result, ScriptResult::ScriptEnded);
}

// ── CoordEvent tests ──────────────────────────────────────────────

#[test]
fn coord_event_matches_exact_position() {
    let event = CoordEvent::new(5, 10, 3);
    assert!(event.matches(5, 10));
    assert!(!event.matches(5, 11));
    assert!(!event.matches(4, 10));
}

#[test]
fn check_coord_events_finds_first_match() {
    let events = vec![
        CoordEvent::new(0, 0, 1),
        CoordEvent::new(5, 5, 2),
        CoordEvent::new(10, 10, 3),
    ];
    assert_eq!(check_coord_events(&events, 5, 5), Some(2));
    assert_eq!(check_coord_events(&events, 0, 0), Some(1));
    assert_eq!(check_coord_events(&events, 10, 10), Some(3));
}

#[test]
fn check_coord_events_returns_none_when_no_match() {
    let events = vec![CoordEvent::new(5, 5, 1)];
    assert_eq!(check_coord_events(&events, 3, 3), None);
}

#[test]
fn check_coord_events_empty_returns_none() {
    assert_eq!(check_coord_events(&[], 5, 5), None);
}

// ══════════════════════════════════════════════════════════════════════
// TrainerEngine tests
// ══════════════════════════════════════════════════════════════════════

fn make_trainer_header(flag: EventFlag, sight_range: u8) -> TrainerHeader {
    TrainerHeader {
        event_flag: flag,
        sight_range,
        before_battle_text_id: 1,
        end_battle_text_id: 2,
        after_battle_text_id: 3,
    }
}

#[test]
fn trainer_not_defeated_initially() {
    let flags = EventFlags::new();
    let header = make_trainer_header(EventFlag::EVENT_BEAT_PEWTER_GYM_TRAINER_0, 5);
    assert!(!is_trainer_defeated(&flags, &header));
}

#[test]
fn trainer_defeated_after_marking() {
    let mut flags = EventFlags::new();
    let header = make_trainer_header(EventFlag::EVENT_BEAT_PEWTER_GYM_TRAINER_0, 5);
    mark_trainer_defeated(&mut flags, &header);
    assert!(is_trainer_defeated(&flags, &header));
}

#[test]
fn trainer_see_player_facing_right() {
    // Trainer at (5,5) facing right, player at (8,5), range 5
    assert!(can_trainer_see_player(5, 5, 1, 0, 8, 5, 5));
    // Player at (11,5) — out of range
    assert!(!can_trainer_see_player(5, 5, 1, 0, 11, 5, 5));
    // Player at (5,5) — same tile (distance 0, not > 0)
    assert!(!can_trainer_see_player(5, 5, 1, 0, 5, 5, 5));
}

#[test]
fn trainer_see_player_facing_left() {
    // Trainer at (10,5) facing left, player at (7,5), range 5
    assert!(can_trainer_see_player(10, 5, -1, 0, 7, 5, 5));
    // Player at (4,5) — out of range
    assert!(!can_trainer_see_player(10, 5, -1, 0, 4, 5, 5));
}

#[test]
fn trainer_see_player_facing_down() {
    // Trainer at (5,3) facing down, player at (5,7), range 5
    assert!(can_trainer_see_player(5, 3, 0, 1, 5, 7, 5));
    // Player at (5,9) — out of range
    assert!(!can_trainer_see_player(5, 3, 0, 1, 5, 9, 5));
}

#[test]
fn trainer_see_player_facing_up() {
    // Trainer at (5,10) facing up, player at (5,6), range 5
    assert!(can_trainer_see_player(5, 10, 0, -1, 5, 6, 5));
    // Player at (5,4) — out of range
    assert!(!can_trainer_see_player(5, 10, 0, -1, 5, 4, 5));
}

#[test]
fn trainer_cannot_see_off_axis() {
    // Trainer facing right, but player is not on same row
    assert!(!can_trainer_see_player(5, 5, 1, 0, 8, 6, 5));
    // Trainer facing down, but player is not on same column
    assert!(!can_trainer_see_player(5, 5, 0, 1, 6, 8, 5));
}

#[test]
fn trainer_cannot_see_behind() {
    // Trainer facing right, player is to the left
    assert!(!can_trainer_see_player(5, 5, 1, 0, 3, 5, 5));
    // Trainer facing up, player is below
    assert!(!can_trainer_see_player(5, 5, 0, -1, 5, 8, 5));
}

#[test]
fn trainer_see_at_exact_range() {
    // Range 3, facing right. Player exactly 3 tiles away
    assert!(can_trainer_see_player(5, 5, 1, 0, 8, 5, 3));
    // Player at 4 tiles — out of range
    assert!(!can_trainer_see_player(5, 5, 1, 0, 9, 5, 3));
}

#[test]
fn trainer_zero_facing_cannot_see() {
    // Both dx=0 and dy=0 — no facing direction
    assert!(!can_trainer_see_player(5, 5, 0, 0, 6, 5, 5));
}

#[test]
fn check_all_trainers_finds_visible() {
    let headers = vec![make_trainer_header(
        EventFlag::EVENT_BEAT_PEWTER_GYM_TRAINER_0,
        5,
    )];
    let flags = EventFlags::new();
    let positions = vec![TrainerPosition {
        x: 5,
        y: 5,
        facing_dx: 1,
        facing_dy: 0,
    }];
    // Player at (8,5) — in range
    assert_eq!(
        check_all_trainers(&headers, &flags, &positions, 8, 5),
        Some(0)
    );
}

#[test]
fn check_all_trainers_skips_defeated() {
    let mut flags = EventFlags::new();
    let header = make_trainer_header(EventFlag::EVENT_BEAT_PEWTER_GYM_TRAINER_0, 5);
    flags.set(header.event_flag);
    let headers = vec![header];
    let positions = vec![TrainerPosition {
        x: 5,
        y: 5,
        facing_dx: 1,
        facing_dy: 0,
    }];
    assert_eq!(check_all_trainers(&headers, &flags, &positions, 8, 5), None);
}

#[test]
fn check_all_trainers_finds_second_when_first_out_of_range() {
    let headers = vec![
        make_trainer_header(EventFlag::EVENT_BEAT_PEWTER_GYM_TRAINER_0, 2),
        make_trainer_header(EventFlag::EVENT_BEAT_CERULEAN_GYM_TRAINER_0, 5),
    ];
    let flags = EventFlags::new();
    let positions = vec![
        TrainerPosition {
            x: 0,
            y: 0,
            facing_dx: 1,
            facing_dy: 0,
        },
        TrainerPosition {
            x: 5,
            y: 10,
            facing_dx: 0,
            facing_dy: -1,
        },
    ];
    // Player at (5,6) — first trainer can't see (wrong row+range), second can (same col, 4 tiles up)
    assert_eq!(
        check_all_trainers(&headers, &flags, &positions, 5, 6),
        Some(1)
    );
}

#[test]
fn check_all_trainers_no_match_returns_none() {
    let headers = vec![make_trainer_header(
        EventFlag::EVENT_BEAT_PEWTER_GYM_TRAINER_0,
        2,
    )];
    let flags = EventFlags::new();
    let positions = vec![TrainerPosition {
        x: 0,
        y: 0,
        facing_dx: 1,
        facing_dy: 0,
    }];
    // Player way out of range
    assert_eq!(
        check_all_trainers(&headers, &flags, &positions, 50, 50),
        None
    );
}

#[test]
fn trainer_battle_state_machine_full_cycle() {
    let mut encounter = TrainerEncounter::new(MapId::PewterGym, 0, 1);
    assert_eq!(encounter.state, TrainerBattleState::NotEngaged);
    assert_eq!(
        advance_trainer_battle(&mut encounter),
        TrainerBattleState::Spotted
    );
    assert_eq!(
        advance_trainer_battle(&mut encounter),
        TrainerBattleState::WalkingToPlayer
    );
    assert_eq!(
        advance_trainer_battle(&mut encounter),
        TrainerBattleState::ShowBeforeBattleText
    );
    assert_eq!(
        advance_trainer_battle(&mut encounter),
        TrainerBattleState::InBattle
    );
    assert_eq!(
        advance_trainer_battle(&mut encounter),
        TrainerBattleState::ShowEndBattleText
    );
    assert_eq!(
        advance_trainer_battle(&mut encounter),
        TrainerBattleState::Defeated
    );
    // Stays defeated
    assert_eq!(
        advance_trainer_battle(&mut encounter),
        TrainerBattleState::Defeated
    );
}

#[test]
fn trainer_encounter_new_defaults() {
    let enc = TrainerEncounter::new(MapId::CeruleanGym, 2, 5);
    assert_eq!(enc.map, MapId::CeruleanGym);
    assert_eq!(enc.trainer_index, 2);
    assert_eq!(enc.npc_index, 5);
    assert_eq!(enc.state, TrainerBattleState::NotEngaged);
}

// ══════════════════════════════════════════════════════════════════════
// MapScripts tests
// ══════════════════════════════════════════════════════════════════════

#[test]
fn map_script_context_new_defaults() {
    let ctx = MapScriptContext::new(MapId::PewterGym);
    assert_eq!(ctx.script_state.map, MapId::PewterGym);
    assert!(ctx.coord_events.is_empty());
    assert!(ctx.active_trainer_encounter.is_none());
    assert!(!ctx.is_encounter_active());
}

#[test]
fn map_script_context_with_coord_events() {
    let ctx = MapScriptContext::new(MapId::CeruleanCity)
        .with_coord_events(vec![CoordEvent::new(5, 5, 1), CoordEvent::new(10, 3, 2)]);
    assert_eq!(ctx.coord_events.len(), 2);
}

#[test]
fn map_script_context_check_coord_trigger() {
    let ctx = MapScriptContext::new(MapId::CeruleanCity)
        .with_coord_events(vec![CoordEvent::new(5, 5, 1), CoordEvent::new(10, 3, 2)]);
    assert_eq!(ctx.check_coord_trigger(5, 5), Some(1));
    assert_eq!(ctx.check_coord_trigger(10, 3), Some(2));
    assert_eq!(ctx.check_coord_trigger(0, 0), None);
}

#[test]
fn map_script_context_start_and_advance_encounter() {
    let mut ctx = MapScriptContext::new(MapId::PewterGym);
    assert!(!ctx.is_encounter_active());
    ctx.start_trainer_battle(0, 1);
    assert!(ctx.is_encounter_active());
    let state = ctx.advance_active_encounter();
    assert_eq!(state, Some(TrainerBattleState::Spotted));
    let state2 = ctx.advance_active_encounter();
    assert_eq!(state2, Some(TrainerBattleState::WalkingToPlayer));
}

#[test]
fn map_script_context_clear_encounter() {
    let mut ctx = MapScriptContext::new(MapId::PewterGym);
    ctx.start_trainer_battle(0, 1);
    assert!(ctx.is_encounter_active());
    ctx.clear_encounter();
    assert!(!ctx.is_encounter_active());
    assert_eq!(ctx.advance_active_encounter(), None);
}

// ── MapScriptType classification ──────────────────────────────────

#[test]
fn classify_story_maps() {
    assert_eq!(
        classify_map_script_type(MapId::PalletTown),
        MapScriptType::StorySequence
    );
    assert_eq!(
        classify_map_script_type(MapId::OaksLab),
        MapScriptType::StorySequence
    );
    assert_eq!(
        classify_map_script_type(MapId::CeruleanCity),
        MapScriptType::StorySequence
    );
}

#[test]
fn classify_trainer_maps() {
    assert_eq!(
        classify_map_script_type(MapId::PewterGym),
        MapScriptType::TrainerMap
    );
    assert_eq!(
        classify_map_script_type(MapId::CeladonGym),
        MapScriptType::TrainerMap
    );
    assert_eq!(
        classify_map_script_type(MapId::ViridianForest),
        MapScriptType::TrainerMap
    );
}

#[test]
fn classify_simple_text_maps() {
    assert_eq!(
        classify_map_script_type(MapId::Route1),
        MapScriptType::SimpleTextOnly
    );
    assert_eq!(
        classify_map_script_type(MapId::ViridianPokecenter),
        MapScriptType::SimpleTextOnly
    );
}

// ── Trainer header data tests ─────────────────────────────────────

#[test]
fn pewter_gym_has_one_trainer_header() {
    let headers = get_trainer_headers(MapId::PewterGym);
    assert_eq!(headers.len(), 1, "PewterGym has 1 trainer");
}

#[test]
fn celadon_gym_has_seven_trainer_headers() {
    let headers = get_trainer_headers(MapId::CeladonGym);
    assert_eq!(headers.len(), 7, "CeladonGym has 7 trainers");
}

#[test]
fn viridian_gym_has_eight_trainer_headers() {
    let headers = get_trainer_headers(MapId::ViridianGym);
    assert_eq!(headers.len(), 8, "ViridianGym has 8 trainers");
}

#[test]
fn route24_has_six_trainer_headers() {
    let headers = get_trainer_headers(MapId::Route24);
    assert_eq!(headers.len(), 6, "Route24 has 6 trainers");
}

#[test]
fn total_trainer_headers_322() {
    let mut total = 0usize;
    for i in 0..248u8 {
        if let Some(map) = MapId::from_u8(i) {
            total += get_trainer_headers(map).len();
        }
    }
    assert_eq!(
        total, 322,
        "Total trainer headers across all maps should be 322"
    );
}

#[test]
fn get_map_trainer_count_matches_headers() {
    assert_eq!(get_map_trainer_count(MapId::PewterGym), 1);
    assert_eq!(get_map_trainer_count(MapId::CeladonGym), 7);
    assert_eq!(get_map_trainer_count(MapId::PalletTown), 0);
}

#[test]
fn get_map_trainer_header_valid_index() {
    let header = get_map_trainer_header(MapId::PewterGym, 0);
    assert!(header.is_some());
    assert_eq!(header.unwrap().sight_range, 5);
}

#[test]
fn get_map_trainer_header_invalid_index() {
    let header = get_map_trainer_header(MapId::PewterGym, 99);
    assert!(header.is_none());
}

#[test]
fn map_with_no_trainers_returns_empty() {
    let headers = get_trainer_headers(MapId::PalletTown);
    assert!(headers.is_empty());
    assert_eq!(get_map_trainer_count(MapId::PalletTown), 0);
}

#[test]
fn pewter_gym_trainer_sight_range_5() {
    let headers = get_trainer_headers(MapId::PewterGym);
    assert_eq!(headers[0].sight_range, 5);
}
