use clap::Parser;
use tracing::Level;

#[derive(Debug, Parser)]
#[clap(name = "rusty-board", about, version, author)]
pub struct Config {
    #[clap(
        about = "Port to bind the server to",
        env = "PORT",
        long,
        default_value = "3000"
    )]
    port: u16,
    #[clap(
        about = "Host to bind the server to",
        env = "HOST",
        long,
        default_value = "localhost"
    )]
    host: String,

    #[clap(
        about = "Maximum log level to output",
        env = "LOG",
        long,
        default_value = "debug"
    )]
    log_level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Self::parse()
    }
}

impl Config {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn init_logger(&self) {
        let collector = tracing_subscriber::fmt()
            .with_max_level(self.log_level)
            .finish();
        tracing::subscriber::set_global_default(collector).expect("unable to set log collector");
    }
}
