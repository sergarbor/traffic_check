use traffic_check;
#[test]
fn test_get_readable_time() {
    let ts: String = String::from("1707843091");
    let expected_time: String = String::from("2024-02-13 16:51:31");

    let date_str = traffic_check::get_readable_time(ts);
    assert_eq!(date_str, expected_time);
}

#[test]
fn test_bytes_to_arp_operation() {
    let operation_bytes = [0x00, 0x01];
    let expected_time: String = String::from("ARP Request");

    let operation = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x00, 0x02];
    let expected_time: String = String::from("ARP Reply");

    let operation = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x00, 0x03];
    let expected_time: String = String::from("RARP Request");

    let operation = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x00, 0x04];
    let expected_time: String = String::from("RARP Reply");

    let operation = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x00, 0x08];
    let expected_time: String = String::from("InARP Request");

    let operation = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0xFF, 0xFF];
    let expected_time: String = String::from("Unknown ARP Operation");

    let operation = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x00, 0x09];
    let expected_time: String = String::from("InARP Reply");

    let operation = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(operation, expected_time);
}

#[test]
fn test_bytes_to_ip_address() {
    let address_bytes = [0xC0, 0xA8, 0x01, 0x0A];
    let expected_time: String = String::from("192.168.1.10");

    let ip_address = traffic_check::bytes_to_ip_address(&address_bytes);
    assert_eq!(ip_address, expected_time);
}

#[test]
fn test_bytes_to_arp_protocol_type() {
    let operation_bytes = [0x08, 0x00];
    let expected_time: String = String::from("IPv4");

    let operation = traffic_check::bytes_to_arp_protocol_type(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x08, 0x06];
    let expected_time: String = String::from("ARP");

    let operation = traffic_check::bytes_to_arp_protocol_type(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x86, 0xDD];
    let expected_time: String = String::from("IPv6");

    let operation = traffic_check::bytes_to_arp_protocol_type(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x80, 0x35];
    let expected_time: String = String::from("RARP");

    let operation = traffic_check::bytes_to_arp_protocol_type(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x80, 0x9B];
    let expected_time: String = String::from("Appletalk");

    let operation = traffic_check::bytes_to_arp_protocol_type(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0xFF, 0xFF];
    let expected_time: String = String::from("Unknown PTYPE");

    let operation = traffic_check::bytes_to_arp_protocol_type(&operation_bytes);
    assert_eq!(operation, expected_time);

    let operation_bytes = [0x80, 0x9D];
    let expected_time: String = String::from("Appletalk ARP");

    let operation = traffic_check::bytes_to_arp_protocol_type(&operation_bytes);
    assert_eq!(operation, expected_time);
}
