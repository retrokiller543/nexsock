use nexsock_config::PROJECT_DIRECTORIES;
use savefile_abi::{AbiConnection, AbiExportable};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static PLUGINS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = std::env::var("PLUGINS_DIR").unwrap_or_else(|_| {
        let config_dir = PROJECT_DIRECTORIES.config_dir();

        config_dir.join("plugins").display().to_string()
    });

    dir.into()
});

pub fn external_native_plugins<T: AbiExportable + ?Sized + 'static>(
) -> anyhow::Result<HashMap<PathBuf, AbiConnection<T>>> {
    let mut connections = HashMap::new();

    for entry in std::fs::read_dir(&*PLUGINS_DIR)? {
        let entry = entry?;
        let path = entry.path();

        match path.extension() {
            Some(extension) if extension == "so" || extension == "dll" || extension == "dylib" => {
                let connection =
                    match AbiConnection::<T>::load_shared_library(path.to_str().unwrap()) {
                        Ok(connection) => connection,
                        _ => continue,
                    };

                connections.insert(path, connection);
            }
            _ => {}
        }
    }

    Ok(connections)
}
