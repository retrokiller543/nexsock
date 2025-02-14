use crate::lua::runner::LuaPluginRunner;
use crate::lua::{LuaMessage, LuaResponses, SerializableLuaValue};
use crate::{PluginResult, PLUGINS_DIR};
use anyhow::{anyhow, Context};
use derive_more::{AsMut, AsRef};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug, AsRef, AsMut)]
pub struct LuaPluginManager {
    handle: Option<JoinHandle<PluginResult<()>>>,
    sender: mpsc::Sender<LuaMessage>,
    receiver: mpsc::Receiver<LuaResponses>,

    pub discovered: HashMap<String, PathBuf>,
}

impl LuaPluginManager {
    /// Constructs a new manager which in turn spawns a new thread with the lua process running.
    pub fn new() -> PluginResult<Self> {
        let (sender, rx) = mpsc::channel();
        let (handle, receiver) = Self::new_runner(rx);

        Ok(Self {
            handle,
            sender,
            receiver,
            discovered: HashMap::new(),
        })
    }

    /// Discover all lua plugins.
    pub fn discover_plugins(&mut self) -> PluginResult<HashMap<String, PathBuf>> {
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

    /// Loads all the plugins into the Runner.
    pub fn load_plugins(&mut self) -> PluginResult<()> {
        let plugins = self.discover_plugins()?;

        for (_, path) in plugins.iter() {
            self.load_script(path)?;
        }

        self.discovered = plugins;

        Ok(())
    }

    fn new_runner(
        receiver: Receiver<LuaMessage>,
    ) -> (
        Option<JoinHandle<PluginResult<()>>>,
        mpsc::Receiver<LuaResponses>,
    ) {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut runner = LuaPluginRunner::new(tx, receiver)?;

            runner.run()
        });

        (Some(handle), rx)
    }

    pub fn load_script(&mut self, path: impl AsRef<Path>) -> PluginResult<PathBuf> {
        self.sender
            .send(LuaMessage::LoadScript(path.as_ref().to_owned()))?;

        match self.receiver.recv()? {
            LuaResponses::ScriptLoaded(path) => Ok(path),
            LuaResponses::Error(e) => Err(anyhow!("Error loading script: {}", e)),
            _ => Err(anyhow!("Unexpected response received")),
        }
    }

    pub fn call_function(
        &self,
        script_path: PathBuf,
        fn_name: &str,
        args: Vec<SerializableLuaValue>,
    ) -> PluginResult<SerializableLuaValue> {
        self.sender.send(LuaMessage::CallFunction(
            script_path,
            fn_name.to_string(),
            args,
        ))?;

        match self.receiver.recv()? {
            LuaResponses::FunctionResult(value) => Ok(value),
            LuaResponses::Error(e) => Err(anyhow!("Error running function: {}", e)),
            _ => Err(anyhow!("Unexpected response received")),
        }
    }
}

impl Drop for LuaPluginManager {
    fn drop(&mut self) {
        let _ = self.sender.send(LuaMessage::Shutdown);

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
