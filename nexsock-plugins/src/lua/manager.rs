use crate::lua::{ScriptContext, SerializableLuaValue};
use crate::{PluginResult, PLUGINS_DIR};
use anyhow::{anyhow, Context};
use derive_more::{AsMut, AsRef};
use mlua::{Function, Lua, Value};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Thread-safe Lua plugin manager
#[derive(Debug, AsRef, AsMut)]
pub struct LuaPluginManager {
    lua: Mutex<Lua>,
    plugins: Mutex<HashMap<PathBuf, ScriptContext>>,
    pub discovered: Mutex<HashMap<String, PathBuf>>,
}

// Implement Send and Sync explicitly to document thread-safety
unsafe impl Send for LuaPluginManager {}
unsafe impl Sync for LuaPluginManager {}

impl LuaPluginManager {
    pub fn new() -> PluginResult<Self> {
        let lua = Lua::new();
        Self::setup_shared_environment(&lua)?;

        Ok(Self {
            lua: Mutex::new(lua),
            plugins: Mutex::new(HashMap::new()),
            discovered: Mutex::new(HashMap::new()),
        })
    }

    fn setup_shared_environment(lua: &Lua) -> PluginResult<()> {
        let shared = lua.create_table()?;

        // Example shared items for each lua instance
        shared.set(
            "log",
            lua.create_function(|_, msg: String| {
                println!("[Lua] {msg}");
                Ok(())
            })?,
        )?;

        let counter = lua.create_table()?;
        counter.set("value", 0i64)?;
        shared.set("counter", counter)?;

        lua.globals().set("shared", shared)?;

        Ok(())
    }

    pub fn discover_plugins(&self) -> PluginResult<HashMap<String, PathBuf>> {
        let mut plugins = HashMap::new();

        for entry in PLUGINS_DIR
            .join("lua")
            .read_dir()
            .context("Failed to read directory")?
        {
            let entry = entry?;
            let path = entry.path();
            let name = entry
                .file_name()
                .into_string()
                .ok()
                .context("Failed to read name")?;

            plugins.insert(name, path);
        }

        Ok(plugins)
    }

    pub async fn load_plugins(&self) -> PluginResult<()> {
        let plugins = self.discover_plugins()?;

        for (_, path) in plugins.iter() {
            self.load_script(path)?;
        }

        *self.discovered.lock() = plugins;
        Ok(())
    }

    pub fn load_script(&self, path: impl AsRef<Path>) -> PluginResult<PathBuf> {
        let lua = self.lua.lock();

        // Create a new environment table
        let environment = lua.create_table()?;

        // Set up environment metatable to fall back to global env
        let globals = lua.globals();
        let metatable = lua.create_table()?;
        metatable.set("__index", globals)?;
        environment.set_metatable(Some(metatable));

        // Load and evaluate the script with the custom environment
        let chunk = lua.load(path.as_ref()).set_name("Plugin");
        chunk.set_environment(environment.clone()).eval::<()>()?;

        let path = path.as_ref().to_owned();
        self.plugins
            .lock()
            .insert(path.clone(), ScriptContext { environment });

        Ok(path)
    }

    pub fn call_function(
        &self,
        script_path: &Path,
        fn_name: &str,
        args: Vec<SerializableLuaValue>,
    ) -> PluginResult<SerializableLuaValue> {
        let lua = self.lua.lock();
        let plugins = self.plugins.lock();

        let context = plugins
            .get(script_path)
            .ok_or_else(|| anyhow!("Script not found: {}", script_path.display()))?;

        let func: Function = context.environment.get(fn_name)?;
        let args = SerializableLuaValue::into_args(args, &lua)?;
        let result: Value = func.call(args)?;

        Ok(SerializableLuaValue::try_from(result)?)
    }

    pub fn call_function_on_all(
        &self,
        fn_name: &str,
        args: Vec<SerializableLuaValue>,
    ) -> PluginResult<Vec<(PathBuf, PluginResult<SerializableLuaValue>)>> {
        let plugins = self.discovered.lock();

        let mut results = Vec::with_capacity(plugins.len());

        for (_, path) in plugins.iter() {
            results.push((
                path.clone(),
                self.call_function(path, fn_name, args.clone()),
            ));
        }

        Ok(results)
    }

    pub fn reload_plugin(&self, path: impl AsRef<Path>) -> PluginResult<()> {
        self.load_script(path)?;
        Ok(())
    }

    pub fn get_plugin_functions(&self, script_path: &Path) -> PluginResult<Vec<String>> {
        let plugins = self.plugins.lock();

        let context = plugins
            .get(script_path)
            .ok_or_else(|| anyhow!("Script not found: {}", script_path.display()))?;

        let mut functions = Vec::new();
        for pair in context.environment.pairs::<Value, Value>() {
            let (key, value) = pair?;
            if let (Value::String(key), Value::Function(_)) = (key, value) {
                functions.push(key.to_string_lossy());
            }
        }

        Ok(functions)
    }
}
