use crate::PLUGINS_DIR;
use savefile_abi::{AbiConnection, AbiExportable};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, error, info};

#[tracing::instrument]
pub async fn external_native_plugins<T: AbiExportable + ?Sized + 'static>(
) -> anyhow::Result<HashMap<PathBuf, AbiConnection<T>>> {
    info!("Loading external native plugins...");
    let mut connections = HashMap::new();
    let dir = &*PLUGINS_DIR.join("native");

    let mut read_dir = tokio::fs::read_dir(dir).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();

        match path.extension() {
            Some(extension) if extension == "so" || extension == "dll" || extension == "dylib" => {
                debug!(path = %path.display(), "Loading external native plugin");
                let connection =
                    match AbiConnection::<T>::load_shared_library(path.to_str().unwrap()) {
                        Ok(connection) => connection,
                        Err(err) => {
                            error!(error = %err, "Failed to load external native plugin");

                            continue;
                        }
                    };

                connections.insert(path, connection);
            }
            _ => {}
        }
    }

    Ok(connections)
}
