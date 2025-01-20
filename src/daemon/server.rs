use crate::daemon::config::DaemonConfig;
use crate::daemon::Daemon;
use crate::error;
use crate::statics::SERVICE_MANAGER;
use tokio::signal::ctrl_c;

#[derive(Debug)]
pub struct DaemonServer {
    daemon: Daemon,
}

impl DaemonServer {
    pub async fn new(config: DaemonConfig) -> error::Result<Self> {
        #[cfg(unix)]
        let daemon = Daemon::new(config)?;
        #[cfg(windows)]
        let daemon = Daemon::new(config).await?;
        Ok(Self { daemon })
    }

    #[inline]
    async fn shutdown(&self) -> error::Result<()> {
        self.daemon.clone().shutdown().await?;
        SERVICE_MANAGER.kill_all().await?;
        Ok(())
    }

    pub async fn run(&self) -> error::Result<()> {
        loop {
            tokio::select! {
                conn = self.daemon.accept() => {
                    match conn {
                        Ok(mut connection) => {
                            tokio::spawn(async move {
                                if let Err(e) = connection.handle().await {
                                    eprintln!("Connection error: {}", e);
                                }
                            });
                        }
                        Err(e) => eprintln!("Accept error: {}", e),
                    }
                }
                _ = ctrl_c() => {
                    self.shutdown().await?;
                    break;
                }
            }
        }
        Ok(())
    }
}
