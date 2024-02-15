use clap::Parser;
use models::Cli;
use pcap::Device;
use traffic_check::{get_protocol_from_data, get_readable_time};

use crate::models::{EthernetFrame, IPV4Frame};

pub mod models;

fn capture_packets(n_packets: u8) {
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    let mut cap_counter = 0;

    while let Ok(packet) = cap.next_packet() {
        if cap_counter >= n_packets {
            break;
        }

        let packet_data: &[u8] = packet.data;
        let packet_len: u32 = packet.header.len;

        // Get the ethernet layer info
        let ether_bytes: &[u8] = &packet_data;
        let ether_header: EthernetFrame = EthernetFrame::new(ether_bytes);

        // get the IPV4 layer info
        let ipv4_bytes: &[u8] = &packet_data[14..];
        let ipv4_header: IPV4Frame = IPV4Frame::new(ipv4_bytes);

        //let protocol: String = get_protocol_from_data(packet_data);
        let capture_time = get_readable_time(packet.header.ts.tv_sec.to_string());

        println!(
            "- {} - Len: {} \n {}\n\n",
            capture_time,
            packet_len,
            //protocol,
            ether_header.to_string()
        );

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
