use clap::Parser;

// Command-line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about = "Load tester for Nexsock process manager")]
pub struct ConcurrentArgs {
    /*/// Socket path to use to communicate with the daemon
    /// 
    /// If none is provided it will default to the system nexsock config, 
    /// and if that does not exist one will be created 
    #[cfg(unix)]
    #[arg(short, long)]
    pub socket: Option<PathBuf>,

    /// Tcp address to use to communicate with the daemon
    ///
    /// If none is provided it will default to the system nexsock config, 
    /// and if that does not exist one will be created
    #[cfg(not(unix))]
    #[arg(short, long)]
    pub(crate) address: Option<SocketAddr>,*/

    /// Number of concurrent clients in the pool
    #[clap(short, long, default_value = "50")]
    pub pool_size: usize,

    /// Total number of requests to send
    #[clap(short, long, default_value = "10000")]
    pub total_requests: usize,

    /// Maximum concurrency level (how many requests to run simultaneously)
    #[clap(short, long, default_value = "100")]
    pub concurrency: usize,

    /// Size of each request in bytes
    #[clap(short = 'z', long, default_value = "1024")]
    pub request_size: usize,

    /// Test duration in seconds (0 means run until total_requests is reached)
    #[clap(short, long, default_value = "0")]
    pub duration: u64,
}