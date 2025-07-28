use clap::{Parser, Subcommand};
use pnet::datalink::{self};

#[derive(Parser)]
#[command(name = "snokkit", version, about = "A Rust packet sniffer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List available interfaces")]
    List,

    #[command(about = "Start capturing packets")]
    Capture {
        #[arg(short, long, help = "Interface to listen on")]
        interface: String,
        #[arg(short, long, help = "Filter for specific protocol")]
        filter: Option<String>,
    },

    #[command(about = "Export captured packets to a file")]
    Export {
        #[arg(short, long, help = "Path to output file")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => list_interfaces(),
        Commands::Capture { interface, filter } => start_capture(interface, filter),
        Commands::Export { output } => export_to_file(output),
    }
}

fn list_interfaces() {
    let interfaces = datalink::interfaces();

    println!("\nInterfaces found: {}\n\n", interfaces.len());
    for i in interfaces {
        println!("{i}\n");
    }
}

fn start_capture(interface: String, filter: Option<String>) {
    println!("Starting capture on interace: {interface}");
    if let Some(f) = &filter {
        println!("With filter: {f}");
    }

    print!("\n\n")
}

fn export_to_file(output: String) {
    println!("Saving to output file: {output}");
}
