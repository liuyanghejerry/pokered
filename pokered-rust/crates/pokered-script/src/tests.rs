use crate::command::{CommandResult, ScriptCommand};
use crate::engine::ScriptEngine;

#[test]
fn test_engine_creation() {
    let engine = ScriptEngine::new();
    assert!(engine.is_idle());
}

#[test]
fn test_load_and_call_simple_script() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        export async function onEnter() {
            await game.showText("Hello world!");
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "Hello world!".to_string()
        })
    );
    assert!(engine.is_waiting());
}

#[test]
fn test_signal_done_continues_script() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        export async function onEnter() {
            await game.showText("Line 1");
            await game.showText("Line 2");
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "Line 1".to_string()
        })
    );

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "Line 2".to_string()
        })
    );

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(cmd, None);
    assert!(engine.is_idle());
}

#[test]
fn test_show_choice_returns_number() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        var chosen = -1;
        export async function onEnter() {
            chosen = await game.showChoice(["Yes", "No"]);
            await game.showText("You picked: " + chosen);
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowChoice {
            options: vec!["Yes".to_string(), "No".to_string()]
        })
    );

    let cmd = engine.signal_done(CommandResult::Number(1.0)).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "You picked: 1".to_string()
        })
    );
}

#[test]
fn test_flag_operations() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        var result = false;
        export async function checkFlags() {
            result = game.getFlag("TEST_FLAG");
            game.setFlag("TEST_FLAG");
            result = game.getFlag("TEST_FLAG");
            if (result) {
                await game.showText("Flag is set!");
            }
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("checkFlags", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "Flag is set!".to_string()
        })
    );
}

#[test]
fn test_conditional_branching() {
    let mut engine = ScriptEngine::new();
    engine.set_flag("GOT_STARTER", true);
    engine
        .load_script(
            r#"
        export async function onEnter() {
            if (game.getFlag("GOT_STARTER")) {
                await game.showText("You already have a starter!");
            } else {
                await game.showText("Choose your starter!");
                await game.givePokemon("BULBASAUR", 5);
            }
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "You already have a starter!".to_string()
        })
    );
}

#[test]
fn test_conditional_branching_else() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        export async function onEnter() {
            if (game.getFlag("GOT_STARTER")) {
                await game.showText("You already have a starter!");
            } else {
                await game.showText("Choose your starter!");
                await game.givePokemon("BULBASAUR", 5);
            }
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "Choose your starter!".to_string()
        })
    );

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::GivePokemon {
            species: "BULBASAUR".to_string(),
            level: 5
        })
    );
}

#[test]
fn test_move_npc_command() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        export async function onEnter() {
            await game.moveNpc("oak", [[2, 3], [2, 5], [4, 5]]);
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::MoveNpc {
            npc_id: "oak".to_string(),
            path: vec![(2, 3), (2, 5), (4, 5)]
        })
    );
}

#[test]
fn test_multiple_commands_sequence() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        export async function onEnter() {
            await game.playMusic("PALLET_TOWN");
            await game.showText("Welcome!");
            await game.delay(30);
            await game.heal();
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::PlayMusic {
            music_id: "PALLET_TOWN".to_string()
        })
    );

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "Welcome!".to_string()
        })
    );

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(cmd, Some(ScriptCommand::Delay { frames: 30 }));

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(cmd, Some(ScriptCommand::Heal));

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(cmd, None);
    assert!(engine.is_idle());
}

#[test]
fn test_tick_returns_pending_command() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        export async function onEnter() {
            await game.showText("Tick test");
        }
    "#,
        )
        .unwrap();

    engine.call_function("onEnter", &[]).unwrap();

    let cmd = engine.tick();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "Tick test".to_string()
        })
    );
    assert!(engine.is_waiting());
}

#[test]
fn test_function_not_found() {
    let mut engine = ScriptEngine::new();
    engine.load_script("export function foo() {}").unwrap();

    let result = engine.call_function("nonExistent", &[]);
    assert!(result.is_err());
}

#[test]
fn test_warp_command() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        export async function doWarp() {
            await game.warpTo("OAKS_LAB", 5, 3);
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("doWarp", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::WarpTo {
            map: "OAKS_LAB".to_string(),
            x: 5,
            y: 3
        })
    );
}

#[test]
fn test_battle_with_result() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        var battleResult = "";
        export async function onEnter() {
            battleResult = await game.startBattle("RIVAL_1");
            if (battleResult === "won") {
                await game.showText("You won!");
            } else {
                await game.showText("You lost...");
            }
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::StartBattle {
            trainer_id: "RIVAL_1".to_string()
        })
    );

    let cmd = engine
        .signal_done(CommandResult::Text("won".to_string()))
        .unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "You won!".to_string()
        })
    );
}

#[test]
fn test_reset_flag() {
    let mut engine = ScriptEngine::new();
    engine.set_flag("MY_FLAG", true);
    assert!(engine.get_flag("MY_FLAG"));

    engine
        .load_script(
            r#"
        export async function doReset() {
            game.resetFlag("MY_FLAG");
            if (!game.getFlag("MY_FLAG")) {
                await game.showText("Flag was reset!");
            }
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("doReset", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "Flag was reset!".to_string()
        })
    );
    assert!(!engine.get_flag("MY_FLAG"));
}

#[test]
fn test_set_map_script() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        export async function onEnter() {
            await game.setMapScript("scriptFollowedOak");
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::SetMapScript {
            state_name: "scriptFollowedOak".to_string()
        })
    );
}

#[test]
fn test_set_joy_ignore() {
    let mut engine = ScriptEngine::new();
    engine
        .load_script(
            r#"
        export async function onEnter() {
            await game.setJoyIgnore(0xFF);
            await game.showText("Input disabled");
            await game.clearJoyIgnore();
        }
    "#,
        )
        .unwrap();

    let cmd = engine.call_function("onEnter", &[]).unwrap();
    assert_eq!(cmd, Some(ScriptCommand::SetJoyIgnore { mask: 255 }));

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(
        cmd,
        Some(ScriptCommand::ShowText {
            text: "Input disabled".to_string()
        })
    );

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(cmd, Some(ScriptCommand::ClearJoyIgnore));

    let cmd = engine.signal_done(CommandResult::Void).unwrap();
    assert_eq!(cmd, None);
    assert!(engine.is_idle());
}
