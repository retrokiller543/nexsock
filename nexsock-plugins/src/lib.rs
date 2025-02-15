use anyhow::Context;
use nexsock_config::PROJECT_DIRECTORIES;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::LazyLock;
use tosic_utils::logging::info;

#[cfg(feature = "lua")]
pub mod lua;
#[cfg(feature = "native")]
pub mod native;

/// The parent plugins directory used by Nexsock
///
/// It will ensure that the provided plugins directory adheres to the expected structure Nexsock expects
pub static PLUGINS_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| get_plugins_dir().expect("Failed to get plugins dir."));

fn get_plugins_dir() -> anyhow::Result<PathBuf> {
    let dir = std::env::var("PLUGINS_DIR").unwrap_or_else(|_| {
        let config_dir = PROJECT_DIRECTORIES.config_dir();

        config_dir.join("plugins").display().to_string()
    });

    let dir_path = PathBuf::from(&dir);

    if !dir_path.exists() {
        info!(directory = dir, "Creating plugin directory");
        create_dir_all(&dir).context(format!("Failed to construct plugins directory '{dir}'"))?;
    }

    #[cfg(feature = "native")]
    let native_dir = dir_path.join("native");

    #[cfg(feature = "lua")]
    let lua_dir = dir_path.join("lua");

    #[cfg(feature = "native")]
    if !native_dir.exists() {
        info!(
            directory = native_dir.display().to_string(),
            "Creating plugin directory"
        );
        create_dir_all(&native_dir).context(format!(
            "Failed to construct plugins directory '{}'",
            native_dir.display()
        ))?;
    }

    #[cfg(feature = "lua")]
    if !lua_dir.exists() {
        info!(
            directory = lua_dir.display().to_string(),
            "Creating plugin directory"
        );
        create_dir_all(&lua_dir).context(format!(
            "Failed to construct plugins directory '{}'",
            lua_dir.display()
        ))?;
    }

    Ok(dir_path)
}

pub type PluginResult<T, E = anyhow::Error> = anyhow::Result<T, E>;
