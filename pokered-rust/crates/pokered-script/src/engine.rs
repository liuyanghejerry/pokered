use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use boa_engine::builtins::promise::PromiseState;
#[cfg(target_arch = "wasm32")]
use boa_engine::module::IdleModuleLoader;
#[cfg(not(target_arch = "wasm32"))]
use boa_engine::module::SimpleModuleLoader;
use boa_engine::object::builtins::{JsFunction, JsPromise};
use boa_engine::property::Attribute;
use boa_engine::{js_string, Context, JsArgs, JsResult, JsValue, Module, NativeFunction, Source};

use crate::command::{CommandResult, ScriptCommand};

#[derive(Debug, thiserror::Error)]
pub enum ScriptEngineError {
    #[error("JS error: {0}")]
    JsError(String),
    #[error("Script not found for map: {0}")]
    ScriptNotFound(String),
    #[error("Function not found: {0}")]
    FunctionNotFound(String),
    #[error("Engine not initialized")]
    NotInitialized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineState {
    Idle,
    Running,
    WaitingForCommand,
    Finished,
}

struct PendingResolve {
    resolve_fn: JsFunction,
}

/// Shared state between the JS runtime and the Rust game loop.
/// Commands issued by JS `await game.showText(...)` are placed here;
/// the game loop reads them, executes the operation, then calls `signal_done`.
struct SharedBridge {
    pending_command: Option<ScriptCommand>,
    pending_resolve: Option<PendingResolve>,
    flags: std::collections::HashMap<String, bool>,
}

impl SharedBridge {
    fn new() -> Self {
        Self {
            pending_command: None,
            pending_resolve: None,
            flags: std::collections::HashMap::new(),
        }
    }
}

pub struct ScriptEngine {
    context: Context,
    bridge: Rc<RefCell<SharedBridge>>,
    state: EngineState,
    /// The currently loaded ES6 module (holds exported function bindings).
    current_module: Option<Module>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        let mut context = Context::builder()
            .module_loader(Rc::new(IdleModuleLoader))
            .build()
            .expect("failed to build JS context");

        #[cfg(not(target_arch = "wasm32"))]
        let mut context = Context::builder()
            .module_loader(Rc::new(SimpleModuleLoader::new(".").expect(
                "failed to create module loader (current directory must exist)",
            )))
            .build()
            .expect("failed to build JS context");
        let bridge = Rc::new(RefCell::new(SharedBridge::new()));

        register_game_api(&mut context, bridge.clone());

        Self {
            context,
            bridge,
            state: EngineState::Idle,
            current_module: None,
        }
    }

    pub fn state(&self) -> &EngineState {
        &self.state
    }

    pub fn is_idle(&self) -> bool {
        self.state == EngineState::Idle
    }

    pub fn is_waiting(&self) -> bool {
        self.state == EngineState::WaitingForCommand
    }

    pub fn set_flag(&mut self, flag: &str, value: bool) {
        self.bridge
            .borrow_mut()
            .flags
            .insert(flag.to_string(), value);
    }

    pub fn get_flag(&self, flag: &str) -> bool {
        self.bridge
            .borrow()
            .flags
            .get(flag)
            .copied()
            .unwrap_or(false)
    }

    pub fn load_script(&mut self, source: &str) -> Result<(), ScriptEngineError> {
        let src = Source::from_reader(source.as_bytes(), Some(Path::new("script.mjs")));
        let module = Module::parse(src, None, &mut self.context)
            .map_err(|e| ScriptEngineError::JsError(e.to_string()))?;

        // Register in the loader (required by Boa even for in-memory modules).
        self.context
            .module_loader()
            .register_module(js_string!("script.mjs"), module.clone());

        let promise = module.load_link_evaluate(&mut self.context);
        self.context.run_jobs();

        match promise.state() {
            PromiseState::Fulfilled(_) => {}
            PromiseState::Rejected(err) => {
                return Err(ScriptEngineError::JsError(format!(
                    "Module evaluation failed: {:?}",
                    err
                )));
            }
            PromiseState::Pending => {
                return Err(ScriptEngineError::JsError(
                    "Module evaluation stuck in pending state".to_string(),
                ));
            }
        }

        self.current_module = Some(module);
        Ok(())
    }

    /// Call a JS async function by name (e.g., "scriptDefault", "talkOak").
    /// The function must be `export`-ed from the loaded module.
    /// Returns the first ScriptCommand if the function immediately awaits one.
    pub fn call_function(
        &mut self,
        fn_name: &str,
        args: &[JsValue],
    ) -> Result<Option<ScriptCommand>, ScriptEngineError> {
        let module = self
            .current_module
            .as_ref()
            .ok_or(ScriptEngineError::NotInitialized)?;

        let func = module
            .get_value(js_string!(fn_name), &mut self.context)
            .map_err(|e| ScriptEngineError::JsError(e.to_string()))?;

        if func.is_undefined() || func.is_null() {
            return Err(ScriptEngineError::FunctionNotFound(fn_name.to_string()));
        }

        let func_obj = func
            .as_callable()
            .ok_or_else(|| ScriptEngineError::FunctionNotFound(fn_name.to_string()))?;

        let _result = func_obj
            .call(&JsValue::undefined(), args, &mut self.context)
            .map_err(|e| ScriptEngineError::JsError(e.to_string()))?;

        self.context.run_jobs();

        self.state = EngineState::Running;
        self.check_pending_command()
    }

    /// Called each frame by the game loop.
    /// Returns the current pending command if the script is waiting.
    pub fn tick(&mut self) -> Option<ScriptCommand> {
        match self.state {
            EngineState::WaitingForCommand => self.bridge.borrow().pending_command.clone(),
            EngineState::Idle | EngineState::Finished => None,
            EngineState::Running => match self.check_pending_command() {
                Ok(cmd) => cmd,
                Err(_) => {
                    self.state = EngineState::Finished;
                    None
                }
            },
        }
    }

    /// Signal that the game has completed the pending command.
    /// Resolves the JS promise so the async function can continue.
    pub fn signal_done(
        &mut self,
        result: CommandResult,
    ) -> Result<Option<ScriptCommand>, ScriptEngineError> {
        if self.state != EngineState::WaitingForCommand {
            return Ok(None);
        }

        let resolve = self.bridge.borrow_mut().pending_resolve.take();
        self.bridge.borrow_mut().pending_command = None;

        if let Some(pending) = resolve {
            let js_result = command_result_to_js(&result, &mut self.context);
            pending
                .resolve_fn
                .call(&JsValue::undefined(), &[js_result], &mut self.context)
                .map_err(|e| ScriptEngineError::JsError(e.to_string()))?;

            self.context.run_jobs();
        }

        self.state = EngineState::Running;
        self.check_pending_command()
    }

    fn check_pending_command(&mut self) -> Result<Option<ScriptCommand>, ScriptEngineError> {
        let cmd = self.bridge.borrow().pending_command.clone();
        if cmd.is_some() {
            self.state = EngineState::WaitingForCommand;
        } else if self.state == EngineState::Running {
            self.state = EngineState::Idle;
        }
        Ok(cmd)
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ── Convenience call methods ─────────────────────────────────────
// These allow pokered-core to call JS functions without depending on boa_engine directly.

impl ScriptEngine {
    /// Call a JS function with no arguments.
    pub fn call_function_no_args(
        &mut self,
        fn_name: &str,
    ) -> Result<Option<ScriptCommand>, ScriptEngineError> {
        self.call_function(fn_name, &[])
    }

    /// Call a JS function with a single u8 argument (e.g., npc text_id lookup).
    pub fn call_function_with_u8(
        &mut self,
        fn_name: &str,
        arg: u8,
    ) -> Result<Option<ScriptCommand>, ScriptEngineError> {
        self.call_function(fn_name, &[JsValue::from(arg as i32)])
    }

    /// Call a JS function with two u16 arguments (e.g., coord event trigger).
    pub fn call_function_with_xy(
        &mut self,
        fn_name: &str,
        x: u16,
        y: u16,
    ) -> Result<Option<ScriptCommand>, ScriptEngineError> {
        self.call_function(fn_name, &[JsValue::from(x as i32), JsValue::from(y as i32)])
    }

    /// Call a JS function with a single string argument.
    pub fn call_function_with_str(
        &mut self,
        fn_name: &str,
        arg: &str,
    ) -> Result<Option<ScriptCommand>, ScriptEngineError> {
        self.call_function(fn_name, &[JsValue::from(js_string!(arg))])
    }

    /// Check if a JS function exists in the module's exports.
    pub fn has_function(&mut self, fn_name: &str) -> bool {
        let module = match self.current_module.as_ref() {
            Some(m) => m,
            None => return false,
        };
        match module.get_value(js_string!(fn_name), &mut self.context) {
            Ok(val) => val.is_callable(),
            Err(_) => false,
        }
    }
}

fn command_result_to_js(result: &CommandResult, _context: &mut Context) -> JsValue {
    match result {
        CommandResult::Void => JsValue::undefined(),
        CommandResult::Bool(b) => JsValue::from(*b),
        CommandResult::Number(n) => JsValue::from(*n),
        CommandResult::Text(s) => JsValue::from(js_string!(s.as_str())),
    }
}

fn register_game_api(context: &mut Context, bridge: Rc<RefCell<SharedBridge>>) {
    let mut game_obj = boa_engine::object::ObjectInitializer::new(context);
    let game_obj = game_obj.build();

    macro_rules! register_async_command {
        ($name:expr, $bridge:expr, $context:expr, $game_obj:expr, $cmd_builder:expr) => {{
            let bridge = $bridge.clone();
            // SAFETY: The closure captures only `Rc<RefCell<SharedBridge>>` which contains no
            // GC-traced (boa `Trace`) types, so it cannot cause use-after-free with the GC.
            let func = unsafe {
                NativeFunction::from_closure(move |_this, args, ctx| {
                    let (promise, resolvers) = JsPromise::new_pending(ctx);

                    let cmd = ($cmd_builder)(args, ctx)?;

                    let mut b = bridge.borrow_mut();
                    b.pending_command = Some(cmd);
                    b.pending_resolve = Some(PendingResolve {
                        resolve_fn: resolvers.resolve,
                    });

                    Ok(promise.into())
                })
            };
            $game_obj
                .set(
                    js_string!($name),
                    func.to_js_function($context.realm()),
                    true,
                    $context,
                )
                .expect(concat!("failed to register game.", $name));
        }};
    }

    // game.showText(text: string) -> Promise<void>
    register_async_command!(
        "showText",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let text = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            Ok(ScriptCommand::ShowText { text })
        }
    );

    // game.showChoice(options: string[]) -> Promise<number>
    register_async_command!(
        "showChoice",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let arr = args.get_or_undefined(0).to_object(ctx)?;
            let len = arr.get(js_string!("length"), ctx)?.to_u32(ctx)?;
            let mut options = Vec::new();
            for i in 0..len {
                let val = arr.get(i, ctx)?;
                options.push(val.to_string(ctx)?.to_std_string_lossy());
            }
            Ok(ScriptCommand::ShowChoice { options })
        }
    );

    // game.moveNpc(npcId: string, path: [number, number][]) -> Promise<void>
    register_async_command!(
        "moveNpc",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let npc_id = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            let arr = args.get_or_undefined(1).to_object(ctx)?;
            let len = arr.get(js_string!("length"), ctx)?.to_u32(ctx)?;
            let mut path = Vec::new();
            for i in 0..len {
                let point = arr.get(i, ctx)?.to_object(ctx)?;
                let x = point.get(0, ctx)?.to_u32(ctx)? as u8;
                let y = point.get(1, ctx)?.to_u32(ctx)? as u8;
                path.push((x, y));
            }
            Ok(ScriptCommand::MoveNpc { npc_id, path })
        }
    );

    // game.startNpcMove(npcId: string, path: [number, number][]) -> Promise<void>
    // Fire-and-forget: starts NPC moving along path, resolves immediately.
    register_async_command!(
        "startNpcMove",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let npc_id = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            let arr = args.get_or_undefined(1).to_object(ctx)?;
            let len = arr.get(js_string!("length"), ctx)?.to_u32(ctx)?;
            let mut path = Vec::new();
            for i in 0..len {
                let point = arr.get(i, ctx)?.to_object(ctx)?;
                let x = point.get(0, ctx)?.to_u32(ctx)? as u8;
                let y = point.get(1, ctx)?.to_u32(ctx)? as u8;
                path.push((x, y));
            }
            Ok(ScriptCommand::StartNpcMove { npc_id, path })
        }
    );

    // game.awaitNpcMove(npcId: string) -> Promise<void>
    // Blocks until the NPC's scripted path is complete.
    register_async_command!(
        "awaitNpcMove",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let npc_id = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            Ok(ScriptCommand::AwaitNpcMove { npc_id })
        }
    );

    // game.movePlayer(path: [number, number][]) -> Promise<void>
    // Blocks until the player finishes walking the path.
    register_async_command!(
        "movePlayer",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let arr = args.get_or_undefined(0).to_object(ctx)?;
            let len = arr.get(js_string!("length"), ctx)?.to_u32(ctx)?;
            let mut path = Vec::new();
            for i in 0..len {
                let point = arr.get(i, ctx)?.to_object(ctx)?;
                let x = point.get(0, ctx)?.to_u32(ctx)? as u8;
                let y = point.get(1, ctx)?.to_u32(ctx)? as u8;
                path.push((x, y));
            }
            Ok(ScriptCommand::MovePlayer { path })
        }
    );

    // game.faceNpc(npcId: string, direction: string) -> Promise<void>
    register_async_command!(
        "faceNpc",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let npc_id = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            let direction = args
                .get_or_undefined(1)
                .to_string(ctx)?
                .to_std_string_lossy();
            Ok(ScriptCommand::FaceNpc { npc_id, direction })
        }
    );

    // game.facePlayer(direction: string) -> Promise<void>
    register_async_command!(
        "facePlayer",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let direction = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            Ok(ScriptCommand::FacePlayer { direction })
        }
    );

    // game.giveItem(itemId: string, quantity: number) -> Promise<void>
    register_async_command!(
        "giveItem",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let item_id = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            let quantity = args.get_or_undefined(1).to_u32(ctx)? as u8;
            Ok(ScriptCommand::GiveItem { item_id, quantity })
        }
    );

    // game.takeItem(itemId: string, quantity: number) -> Promise<void>
    register_async_command!(
        "takeItem",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let item_id = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            let quantity = args.get_or_undefined(1).to_u32(ctx)? as u8;
            Ok(ScriptCommand::TakeItem { item_id, quantity })
        }
    );

    // game.givePokemon(species: string, level: number) -> Promise<void>
    register_async_command!(
        "givePokemon",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let species = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            let level = args.get_or_undefined(1).to_u32(ctx)? as u8;
            Ok(ScriptCommand::GivePokemon { species, level })
        }
    );

    // game.startBattle(trainerId: string) -> Promise<string>
    register_async_command!(
        "startBattle",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let trainer_id = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            Ok(ScriptCommand::StartBattle { trainer_id })
        }
    );

    // game.playMusic(musicId: string) -> Promise<void>
    register_async_command!(
        "playMusic",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let music_id = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            Ok(ScriptCommand::PlayMusic { music_id })
        }
    );

    // game.playSound(soundId: string) -> Promise<void>
    register_async_command!(
        "playSound",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let sound_id = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            Ok(ScriptCommand::PlaySound { sound_id })
        }
    );

    // game.stopMusic() -> Promise<void>
    register_async_command!(
        "stopMusic",
        bridge,
        context,
        game_obj,
        |_args: &[JsValue], _ctx: &mut Context| -> JsResult<ScriptCommand> {
            Ok(ScriptCommand::StopMusic)
        }
    );

    // game.fadeOutMusic() -> Promise<void>
    register_async_command!(
        "fadeOutMusic",
        bridge,
        context,
        game_obj,
        |_args: &[JsValue], _ctx: &mut Context| -> JsResult<ScriptCommand> {
            Ok(ScriptCommand::FadeOutMusic)
        }
    );

    // game.delay(frames: number) -> Promise<void>
    register_async_command!("delay", bridge, context, game_obj, |args: &[JsValue],
                                                                 ctx: &mut Context|
     -> JsResult<
        ScriptCommand,
    > {
        let frames = args.get_or_undefined(0).to_u32(ctx)? as u16;
        Ok(ScriptCommand::Delay { frames })
    });

    // game.warpTo(map: string, x: number, y: number) -> Promise<void>
    register_async_command!("warpTo", bridge, context, game_obj, |args: &[JsValue],
                                                                  ctx: &mut Context|
     -> JsResult<
        ScriptCommand,
    > {
        let map = args
            .get_or_undefined(0)
            .to_string(ctx)?
            .to_std_string_lossy();
        let x = args.get_or_undefined(1).to_u32(ctx)? as u8;
        let y = args.get_or_undefined(2).to_u32(ctx)? as u8;
        Ok(ScriptCommand::WarpTo { map, x, y })
    });

    // game.heal() -> Promise<void>
    register_async_command!("heal", bridge, context, game_obj, |_args: &[JsValue],
                                                                _ctx: &mut Context|
     -> JsResult<
        ScriptCommand,
    > {
        Ok(ScriptCommand::Heal)
    });

    // game.fadeScreen(fadeType: string) -> Promise<void>
    register_async_command!(
        "fadeScreen",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let fade_type = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_lossy();
            Ok(ScriptCommand::FadeScreen { fade_type })
        }
    );

    // game.showObject(objectIndexOrToggleId: number | string) -> Promise<void>
    register_async_command!(
        "showObject",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let arg = args.get_or_undefined(0);
            if arg.is_string() {
                let toggle_id = arg.to_string(ctx)?.to_std_string_lossy();
                Ok(ScriptCommand::ShowObjectByName { toggle_id })
            } else {
                let object_index = arg.to_u32(ctx)? as u8;
                Ok(ScriptCommand::ShowObject { object_index })
            }
        }
    );

    // game.hideObject(objectIndexOrToggleId: number | string) -> Promise<void>
    register_async_command!(
        "hideObject",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let arg = args.get_or_undefined(0);
            if arg.is_string() {
                let toggle_id = arg.to_string(ctx)?.to_std_string_lossy();
                Ok(ScriptCommand::HideObjectByName { toggle_id })
            } else {
                let object_index = arg.to_u32(ctx)? as u8;
                Ok(ScriptCommand::HideObject { object_index })
            }
        }
    );

    // game.setJoyIgnore(mask: number) -> Promise<void>
    register_async_command!(
        "setJoyIgnore",
        bridge,
        context,
        game_obj,
        |args: &[JsValue], ctx: &mut Context| -> JsResult<ScriptCommand> {
            let mask = args.get_or_undefined(0).to_u32(ctx)? as u8;
            Ok(ScriptCommand::SetJoyIgnore { mask })
        }
    );

    // game.clearJoyIgnore() -> Promise<void>
    register_async_command!(
        "clearJoyIgnore",
        bridge,
        context,
        game_obj,
        |_args: &[JsValue], _ctx: &mut Context| -> JsResult<ScriptCommand> {
            Ok(ScriptCommand::ClearJoyIgnore)
        }
    );

    // game.getFlag(flag: string) -> boolean
    {
        let bridge = bridge.clone();
        // SAFETY: closure captures Rc<RefCell<SharedBridge>> — no GC-traced types.
        let func = unsafe {
            NativeFunction::from_closure(move |_this, args, ctx| {
                let flag = args
                    .get_or_undefined(0)
                    .to_string(ctx)?
                    .to_std_string_lossy();
                let val = bridge.borrow().flags.get(&flag).copied().unwrap_or(false);
                Ok(JsValue::from(val))
            })
        };
        game_obj
            .set(
                js_string!("getFlag"),
                func.to_js_function(context.realm()),
                true,
                context,
            )
            .expect("failed to register game.getFlag");
    }

    // game.setFlag(flag: string) -> void
    {
        let bridge = bridge.clone();
        // SAFETY: closure captures Rc<RefCell<SharedBridge>> — no GC-traced types.
        let func = unsafe {
            NativeFunction::from_closure(move |_this, args, ctx| {
                let flag = args
                    .get_or_undefined(0)
                    .to_string(ctx)?
                    .to_std_string_lossy();
                bridge.borrow_mut().flags.insert(flag, true);
                Ok(JsValue::undefined())
            })
        };
        game_obj
            .set(
                js_string!("setFlag"),
                func.to_js_function(context.realm()),
                true,
                context,
            )
            .expect("failed to register game.setFlag");
    }

    // game.resetFlag(flag: string) -> void
    {
        let bridge = bridge.clone();
        // SAFETY: closure captures Rc<RefCell<SharedBridge>> — no GC-traced types.
        let func = unsafe {
            NativeFunction::from_closure(move |_this, args, ctx| {
                let flag = args
                    .get_or_undefined(0)
                    .to_string(ctx)?
                    .to_std_string_lossy();
                bridge.borrow_mut().flags.insert(flag, false);
                Ok(JsValue::undefined())
            })
        };
        game_obj
            .set(
                js_string!("resetFlag"),
                func.to_js_function(context.realm()),
                true,
                context,
            )
            .expect("failed to register game.resetFlag");
    }

    context
        .register_global_property(js_string!("game"), game_obj, Attribute::all())
        .expect("failed to register global game object");
}
