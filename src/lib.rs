use chrono::prelude::*;

pub fn get_readable_time(time_stamp: String) -> String {
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

pub fn get_protocol_from_data(packet_data: &[u8]) -> String {
    let prot_data = packet_data.get(9);
    let protocol: String;
    match prot_data {
        Some(v) if *v == 6 => {
            protocol = String::from("TCP");
        }
        Some(v) if *v == 17 => {
            protocol = String::from("UDP");
        }
        Some(_) => {
            protocol = String::from("UNKNOWN");
        }
        None => {
            protocol = String::from("NONE");
        }
    }
    return protocol;
}
