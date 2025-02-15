use crate::lua::{LuaMessage, LuaResponses, ScriptContext, SerializableLuaValue};
use crate::PluginResult;
use mlua::{Function, Lua, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use tracing::{debug, debug_span};

pub struct LuaPluginRunner {
    lua: Lua,

    sender: mpsc::Sender<LuaResponses>,
    receiver: mpsc::Receiver<LuaMessage>,
    plugins: HashMap<PathBuf, ScriptContext>,
}

impl LuaPluginRunner {
    pub fn new(sender: Sender<LuaResponses>, receiver: Receiver<LuaMessage>) -> PluginResult<Self> {
        let lua = Lua::new();

        Self::setup_shared_environment(&lua)?;

        let plugins = HashMap::new();

        Ok(Self {
            lua,
            sender,
            receiver,
            plugins,
        })
    }

    pub fn run(&mut self) -> PluginResult<()> {
        while let Ok(message) = self.receiver.recv() {
            match message {
                LuaMessage::LoadScript(path) => match self.load_script(&path) {
                    Ok(context) => {
                        self.plugins.insert(path.clone(), context);
                        let _ = self.sender.send(LuaResponses::ScriptLoaded(path));
                    }
                    Err(e) => {
                        let _ = self.sender.send(LuaResponses::Error(e.to_string()));
                    }
                },
                LuaMessage::CallFunction(script_path, fn_name, args) => {
                    if let Some(context) = self.plugins.get(&script_path) {
                        match self.call_function(context, &fn_name, args) {
                            Ok(result) => {
                                let _ = self.sender.send(LuaResponses::FunctionResult(result));
                            }
                            Err(e) => {
                                let _ = self.sender.send(LuaResponses::Error(e.to_string()));
                            }
                        }
                    } else {
                        let _ = self.sender.send(LuaResponses::Error(format!(
                            "Script not found: {}",
                            script_path.display()
                        )));
                    }
                }
                LuaMessage::Shutdown => break,
            }
        }
        Ok(())
    }

    fn setup_shared_environment(lua: &Lua) -> PluginResult<()> {
        let span = debug_span!("setup_shared_environment");
        let _span = span.enter();

        debug!("Setting up lua environment");
        let shared = lua.create_table()?;

        // Example shared items for each lua instance
        shared.set(
            "log",
            lua.create_function(|_, msg: String| {
                println!("[Lua] {}", msg);
                Ok(())
            })?,
        )?;

        let counter = lua.create_table()?;
        counter.set("value", 0i64)?;
        shared.set("counter", counter)?;

        lua.globals().set("shared", shared)?;

        Ok(())
    }

    // TODO: Add verification of the loaded plugin to see if it actually conforms to our needs
    #[tracing::instrument(level = "debug", skip(self))]
    fn load_script(&self, path: &Path) -> PluginResult<ScriptContext> {
        debug!("Loading lua script");

        let lua = &self.lua;

        // Create a new environment table
        let environment = lua.create_table()?;

        // Set up environment metatable to fall back to global env
        let globals = lua.globals();
        let metatable = lua.create_table()?;
        metatable.set("__index", globals)?;
        environment.set_metatable(Some(metatable));

        let cloned_env = environment.clone();

        // Load and evaluate the script with the custom environment
        let chunk = lua.load(path).set_name("Plugin");
        chunk.set_environment(cloned_env).eval::<()>()?;

        debug!("Loaded lua script");

        Ok(ScriptContext { environment })
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn call_function(
        &self,
        context: &ScriptContext,
        name: &str,
        args: Vec<SerializableLuaValue>,
    ) -> PluginResult<SerializableLuaValue> {
        let func: Function = context.environment.get(name)?;
        let args = SerializableLuaValue::into_args(args, &self.lua)?;
        let result: Value = func.call(args)?;

        Ok(SerializableLuaValue::try_from(result)?)
    }
}
