mod capture;
mod cli;
mod list;
mod utils;

use crate::capture::start_capture;
use crate::list::list_interfaces;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => list_interfaces(),
        Commands::Capture { interface, filter } => start_capture(interface, filter),
    }
}
