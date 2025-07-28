use pnet::datalink::{self};

pub fn list_interfaces() {
    let interfaces = datalink::interfaces();
    println!("\nInterfaces found: {}\n", interfaces.len());
    interfaces
        .into_iter()
        .for_each(|iface| println!("{iface}\n"));
}
