use clap::Parser;
use cli::Cli;
use pcap::Device;

use chrono::prelude::*;
pub mod cli;

fn get_readable_time(time_stamp: String) -> String {
    // Convert the timestamp string into an i64
    let timestamp = time_stamp.parse::<i64>().unwrap();
    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
    // Print the newly formatted date and time
    return newdate.to_string();
}

fn capture_packets(n_packets: u8) {
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    let mut cap_counter = 0;

    while let Ok(packet) = cap.next_packet() {
        if cap_counter >= n_packets {
            break;
        }
        let packet_data: &[u8] = packet.data;
        let packet_len: u32 = packet.header.len;
        let capture_time = get_readable_time(packet.header.ts.tv_sec.to_string());
        println!(
            "- {} - Len: {} \n {:?}",
            capture_time, packet_len, packet_data
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
