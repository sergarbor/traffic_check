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
