use pnet::datalink::{self};

pub fn list_interfaces() {
    let interfaces = datalink::interfaces();

    println!("\nInterfaces found: {}\n\n", interfaces.len());
    for i in interfaces {
        println!("{i}\n");
    }
}
