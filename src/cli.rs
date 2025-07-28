use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "snokkit", version, about = "A Rust packet sniffer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "List available interfaces")]
    List,

    #[command(about = "Start capturing packets")]
    Capture {
        #[arg(short, long, help = "Interface to listen on")]
        interface: String,
        #[arg(short, long, help = "Filter for specific protocol")]
        filter: Option<String>,
    },
}
