use super::collision::*;
use super::player_movement::*;
use super::*;
use pokered_data::collision;
use pokered_data::maps::MapId;
use pokered_data::music::MusicId;
use pokered_data::tilesets::TilesetId;

fn make_state() -> OverworldState {
    let mut s = OverworldState::new(MapId::PalletTown);
    s.player.x = 5;
    s.player.y = 5;
    s
}

fn make_map() -> MapData {
    MapData {
        id: MapId::PalletTown,
        width: 10,
        height: 9,
        tileset: TilesetId::Overworld,
        music: MusicId::PalletTown,
        blocks: vec![0; 90],
        warps: vec![],
        npcs: vec![],
        signs: vec![],
        connections: MapConnections::default(),
    }
}

#[test]
fn test_try_move_passable() {
    let mut state = make_state();
    let result = try_move(
        &mut state,
        Direction::Down,
        TilesetId::Overworld,
        10,
        9,
        0x00,
        0x00,
        &[],
        0,
    );
    assert_eq!(result, MoveResult::Walking);
    assert_eq!(state.player.movement_state, MovementState::Walking);
    assert_eq!(state.walk_counter, WALK_COUNTER_INIT);
    assert_eq!(state.player.facing, Direction::Down);
}

#[test]
fn test_try_move_blocked() {
    let mut state = make_state();
    let result = try_move(
        &mut state,
        Direction::Down,
        TilesetId::Overworld,
        10,
        9,
        0x00,
        0xFF,
        &[],
        0,
    );
    assert!(matches!(
        result,
        MoveResult::Blocked(CollisionResult::TileBlocked)
    ));
    assert_eq!(state.player.movement_state, MovementState::Idle);
}

#[test]
fn test_try_move_turn_only() {
    let mut state = make_state();
    state.player.facing = Direction::Up;
    let result = try_move(
        &mut state,
        Direction::Down,
        TilesetId::Overworld,
        10,
        9,
        0x00,
        0xFF,
        &[],
        0,
    );
    assert_eq!(result, MoveResult::TurnedOnly);
    assert_eq!(state.player.facing, Direction::Down);
}

#[test]
fn test_try_move_map_edge() {
    let mut state = make_state();
    state.player.x = 0;
    state.player.y = 0;
    let result = try_move(
        &mut state,
        Direction::Up,
        TilesetId::Overworld,
        10,
        9,
        0x00,
        0x00,
        &[],
        0,
    );
    assert_eq!(result, MoveResult::ReachedMapEdge);
}

#[test]
fn test_try_move_ledge_jump() {
    let mut state = make_state();
    let result = try_move(
        &mut state,
        Direction::Down,
        TilesetId::Overworld,
        10,
        9,
        0x2C,
        0x37,
        &[],
        collision::PAD_DOWN,
    );
    assert_eq!(result, MoveResult::LedgeJump);
    assert_eq!(state.player.movement_state, MovementState::Jumping);
    assert_eq!(state.walk_counter, WALK_COUNTER_INIT * 2);
}

#[test]
fn test_try_move_still_moving() {
    let mut state = make_state();
    state.player.movement_state = MovementState::Walking;
    let result = try_move(
        &mut state,
        Direction::Down,
        TilesetId::Overworld,
        10,
        9,
        0x00,
        0x00,
        &[],
        0,
    );
    assert_eq!(result, MoveResult::StillMoving);
}

#[test]
fn test_advance_step_walking() {
    let mut state = make_state();
    state.player.movement_state = MovementState::Walking;
    state.player.facing = Direction::Down;
    state.walk_counter = WALK_COUNTER_INIT;

    for _ in 0..7 {
        assert!(!advance_step(&mut state));
    }
    assert!(advance_step(&mut state));
    assert_eq!(state.player.y, 6);
    assert_eq!(state.player.x, 5);
    assert_eq!(state.player.movement_state, MovementState::Idle);
}

#[test]
fn test_advance_step_biking_double_speed() {
    let mut state = make_state();
    state.player.movement_state = MovementState::Walking;
    state.player.facing = Direction::Right;
    state.player.transport = TransportMode::Biking;
    state.walk_counter = WALK_COUNTER_INIT;

    for _ in 0..3 {
        assert!(!advance_step(&mut state));
    }
    assert!(advance_step(&mut state));
    assert_eq!(state.player.x, 6);
    assert_eq!(state.player.movement_state, MovementState::Idle);
}

#[test]
fn test_advance_step_ledge_jump_moves_two_tiles() {
    let mut state = make_state();
    state.player.movement_state = MovementState::Jumping;
    state.player.facing = Direction::Down;
    state.walk_counter = WALK_COUNTER_INIT * 2;

    while !advance_step(&mut state) {}
    assert_eq!(state.player.y, 7);
    assert_eq!(state.player.movement_state, MovementState::Idle);
}

#[test]
fn test_advance_step_decrements_repel() {
    let mut state = make_state();
    state.player.movement_state = MovementState::Walking;
    state.player.facing = Direction::Down;
    state.walk_counter = 1;
    state.repel_steps = 10;

    advance_step(&mut state);
    assert_eq!(state.repel_steps, 9);
}

#[test]
fn test_advance_step_decrements_encounter_cooldown() {
    let mut state = make_state();
    state.player.movement_state = MovementState::Walking;
    state.player.facing = Direction::Down;
    state.walk_counter = 1;
    state.encounter_cooldown = 5;

    advance_step(&mut state);
    assert_eq!(state.encounter_cooldown, 4);
}

#[test]
fn test_advance_step_zero_counter_noop() {
    let mut state = make_state();
    state.walk_counter = 0;
    assert!(advance_step(&mut state));
    assert_eq!(state.player.x, 5);
    assert_eq!(state.player.y, 5);
}

#[test]
fn test_direction_delta() {
    assert_eq!(direction_delta(Direction::Down), (0, 1));
    assert_eq!(direction_delta(Direction::Up), (0, -1));
    assert_eq!(direction_delta(Direction::Left), (-1, 0));
    assert_eq!(direction_delta(Direction::Right), (1, 0));
}

#[test]
fn test_opposite_direction() {
    assert_eq!(opposite_direction(Direction::Down), Direction::Up);
    assert_eq!(opposite_direction(Direction::Up), Direction::Down);
    assert_eq!(opposite_direction(Direction::Left), Direction::Right);
    assert_eq!(opposite_direction(Direction::Right), Direction::Left);
}

#[test]
fn test_input_direction_priority() {
    let input = InputState {
        down: true,
        up: true,
        ..Default::default()
    };
    assert_eq!(input.direction_pressed(), Some(Direction::Down));

    let input2 = InputState {
        up: true,
        left: true,
        ..Default::default()
    };
    assert_eq!(input2.direction_pressed(), Some(Direction::Up));

    let input3 = InputState {
        left: true,
        right: true,
        ..Default::default()
    };
    assert_eq!(input3.direction_pressed(), Some(Direction::Left));
}

#[test]
fn test_input_no_direction() {
    let input = InputState::default();
    assert_eq!(input.direction_pressed(), None);
}

#[test]
fn test_input_to_pad_bits() {
    let input = InputState {
        down: true,
        ..Default::default()
    };
    assert_eq!(input.to_pad_bits(), collision::PAD_DOWN);

    let input2 = InputState {
        up: true,
        right: true,
        ..Default::default()
    };
    assert_eq!(
        input2.to_pad_bits(),
        collision::PAD_UP | collision::PAD_RIGHT
    );
}

#[test]
fn test_is_on_grass() {
    assert!(is_on_grass(0x52, TilesetId::Overworld));
    assert!(is_on_grass(0x20, TilesetId::Forest));
    assert!(!is_on_grass(0x00, TilesetId::Overworld));
    assert!(!is_on_grass(0x52, TilesetId::Mart));
}

#[test]
fn test_frames_per_step() {
    assert_eq!(frames_per_step(TransportMode::Walking), 8);
    assert_eq!(frames_per_step(TransportMode::Biking), 4);
    assert_eq!(frames_per_step(TransportMode::Surfing), 8);
}

#[test]
fn test_process_frame_no_input() {
    let mut state = make_state();
    let map = make_map();
    let input = InputState::default();
    let result = process_frame(&mut state, &input, &map, 0x00, 0x00, &[]);
    assert_eq!(result, MoveResult::NoInput);
}

#[test]
fn test_process_frame_start_walk() {
    let mut state = make_state();
    let map = make_map();
    let input = InputState {
        down: true,
        ..Default::default()
    };
    let result = process_frame(&mut state, &input, &map, 0x00, 0x00, &[]);
    assert_eq!(result, MoveResult::Walking);
    assert_eq!(state.player.movement_state, MovementState::Walking);
}

#[test]
fn test_process_frame_continue_walk() {
    let mut state = make_state();
    state.player.movement_state = MovementState::Walking;
    state.player.facing = Direction::Down;
    state.walk_counter = 5;
    let map = make_map();
    let input = InputState {
        down: true,
        ..Default::default()
    };
    let result = process_frame(&mut state, &input, &map, 0x00, 0x00, &[]);
    assert_eq!(result, MoveResult::StillMoving);
    assert_eq!(state.walk_counter, 4);
}

#[test]
fn test_process_frame_warp_on_step_complete() {
    let mut state = make_state();
    state.player.movement_state = MovementState::Walking;
    state.player.facing = Direction::Down;
    state.walk_counter = 1;
    let mut map = make_map();
    map.warps.push(WarpPoint {
        x: 2,
        y: 3,
        target_map: MapId::RedsHouse1F,
        target_warp_id: 0,
    });
    // Player at (5,5), facing down, walk counter=1 -> step completes to (5,6)
    // Warp at block (2,3) = tiles (4-5, 6-7)... but our check uses x/2, y/2
    // Player tile (5,6) -> block (2,3) which matches warp
    let input = InputState::default();
    let result = process_frame(&mut state, &input, &map, 0x00, 0x00, &[]);
    assert_eq!(result, MoveResult::Warped { warp_index: 0 });
}

#[test]
fn test_full_walk_cycle() {
    let mut state = make_state();
    let map = make_map();
    let input_down = InputState {
        down: true,
        ..Default::default()
    };
    let no_input = InputState::default();

    // Start walking
    let r = process_frame(&mut state, &input_down, &map, 0x00, 0x00, &[]);
    assert_eq!(r, MoveResult::Walking);

    // Advance 8 frames to complete walk (walk_counter 8→0)
    for _ in 0..8 {
        let r = process_frame(&mut state, &no_input, &map, 0x00, 0x00, &[]);
        assert!(r == MoveResult::StillMoving || r == MoveResult::Walking);
    }

    // After 8 total frames, position should be updated
    assert_eq!(state.player.y, 6);
    assert_eq!(state.player.movement_state, MovementState::Idle);
}
