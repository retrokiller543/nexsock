use tokio::signal::ctrl_c;
use crate::daemon::config::DaemonConfig;
use crate::daemon::Daemon;
use crate::error;

#[derive(Debug)]
pub struct DaemonServer {
    daemon: Daemon,
}

impl DaemonServer {
    pub fn new(config: DaemonConfig) -> error::Result<Self> {
        let daemon = Daemon::new(config)?;
        Ok(Self { daemon })
    }
    
    #[inline]
    async fn shutdown(&self) -> error::Result<()> {
        self.daemon.clone().shutdown().await?;
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