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

    let date_str = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(date_str, expected_time);

    let operation_bytes = [0x00, 0x02];
    let expected_time: String = String::from("ARP Reply");

    let date_str = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(date_str, expected_time);

    let operation_bytes = [0x00, 0x03];
    let expected_time: String = String::from("RARP Request");

    let date_str = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(date_str, expected_time);

    let operation_bytes = [0x00, 0x04];
    let expected_time: String = String::from("RARP Reply");

    let date_str = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(date_str, expected_time);

    let operation_bytes = [0x00, 0x08];
    let expected_time: String = String::from("InARP Request");

    let date_str = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(date_str, expected_time);

    let operation_bytes = [0x00, 0x09];
    let expected_time: String = String::from("InARP Reply");

    let date_str = traffic_check::bytes_to_arp_operation(&operation_bytes);
    assert_eq!(date_str, expected_time);
}
