use chrono::Local;
use clap::{Parser, Subcommand};
use colored::*;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherType, EtherTypes, EthernetPacket};
use std::collections::HashMap;

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

fn build_ethertype_map() -> HashMap<EtherType, ColoredString> {
    let mut map = HashMap::new();

    map.insert(EtherTypes::Ipv4, "IPv4".green());
    map.insert(EtherTypes::Ipv6, "IPv6".blue());
    map.insert(EtherTypes::Arp, "ARP".yellow());
    map.insert(EtherTypes::Rarp, "RARP".bright_yellow());
    map.insert(EtherTypes::PppoeDiscovery, "PPPoE-Discovery".bright_cyan());
    map.insert(EtherTypes::PppoeSession, "PPPoE-Session".cyan());
    map.insert(EtherTypes::Mpls, "MPLS".bright_blue());
    map.insert(EtherTypes::MplsMcast, "MPLS-MCAST".blue());
    map.insert(EtherTypes::WakeOnLan, "Wake-on-LAN".bright_green());
    map.insert(EtherTypes::Vlan, "802.1Q VLAN".bright_white());

    map
}

fn build_filter_map() -> HashMap<String, EtherType> {
    let mut map = HashMap::new();

    map.insert("ipv4".to_string(), EtherTypes::Ipv4);
    map.insert("ipv6".to_string(), EtherTypes::Ipv6);
    map.insert("arp".to_string(), EtherTypes::Arp);
    map.insert("rarp".to_string(), EtherTypes::Rarp);
    map.insert("pppoe-discovery".to_string(), EtherTypes::PppoeDiscovery);
    map.insert("pppoe-session".to_string(), EtherTypes::PppoeSession);
    map.insert("mpls".to_string(), EtherTypes::Mpls);
    map.insert("mpls-mcast".to_string(), EtherTypes::MplsMcast);
    map.insert("wakeonlan".to_string(), EtherTypes::WakeOnLan);
    map.insert("vlan".to_string(), EtherTypes::Vlan);

    map
}

fn start_capture(interface_name: String, filter: Option<String>) {
    let ethertype_map = build_ethertype_map();
    let filter_map = build_filter_map();

    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .expect("Failed to find the specified interface");

    let (mut _tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Error creating datalink channel: {}", e),
    };

    println!("\nCapturing packets on interface: {interface_name}\n\n");

    let expected_ethertype = filter
        .as_ref()
        .and_then(|f| filter_map.get(&f.to_lowercase()))
        .copied();

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();

                // Skip if filter set and packet doesn't match
                if let Some(expected) = expected_ethertype {
                    if packet.get_ethertype() != expected {
                        continue;
                    }
                }

                let timestamp = get_timestamp();
                let source = format!("{}", packet.get_source()).cyan();
                let destination = format!("{}", packet.get_destination()).magenta();

                let protocol = ethertype_map
                    .get(&packet.get_ethertype())
                    .cloned()
                    .unwrap_or_else(|| format!("Other (0x{:04x})", packet.get_ethertype().0).red());

                println!(
                    "{} {} → {} [{}]",
                    timestamp.dimmed(),
                    source,
                    destination,
                    protocol
                );
            }
            Err(e) => {
                eprintln!("Error while reading: {}", e.to_string().red());
            }
        }
    }
}

fn export_to_file(output: String) {
    println!("Saving to output file: {output}");
}

fn get_timestamp() -> String {
    Local::now().format("%H:%M:%S%.3f").to_string()
}
