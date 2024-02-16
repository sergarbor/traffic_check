use traffic_check;
#[test]
fn test_get_readable_time() {
    let ts: String = String::from("1707843091");
    let expected_time: String = String::from("2024-02-13 16:51:31");

    let date_str = traffic_check::get_readable_time(ts);
    assert_eq!(date_str, expected_time);
}
