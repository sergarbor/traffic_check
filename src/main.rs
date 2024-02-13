use clap::Parser;
use cli::Cli;
use pcap::Device;
use traffic_check::{get_protocol_from_data, get_readable_time};

pub mod cli;

fn capture_packets(n_packets: u8) {
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    let mut cap_counter = 0;

    while let Ok(packet) = cap.next_packet() {
        if cap_counter >= n_packets {
            break;
        }
        let packet_data: &[u8] = packet.data;
        let packet_len: u32 = packet.header.len;
        let protocol: String = get_protocol_from_data(packet_data);
        let capture_time = get_readable_time(packet.header.ts.tv_sec.to_string());
        println!("- {} - Len: {} {}", capture_time, packet_len, protocol);
        cap_counter += 1;
    }
}

fn main() {
    let args = Cli::parse();

    println!("Command: {} Value {}", args.command, args.value);

    // If the command is not check we skip
    if args.command != "check" {
        return;
    }

    println!("THIS IS THE EXPECTED COMMAND!");

    // Start capturing packets!!
    capture_packets(args.value);
}
