extern crate rustogram;
const HIGHEST_TRACKABLE_VALUE: i64 = 3600 * 1000 * 1000;
const NUMBER_OF_SIGNIFICANT_VALUE_DIGITS: i32 = 3;   

#[test]
fn test_get_total_count() {
    let histogram = get_histogram();
    let raw_histogram = get_raw_histogram();
    assert_eq!(10_001, raw_histogram.get_total_count());
    assert_eq!(20_000, histogram.get_total_count());
}

#[test]
fn test_get_max_value() {
    let histogram = get_histogram();
    assert!(histogram.values_are_equivalent(100_000_000, histogram.get_max_value()));
}

fn get_histogram() -> rustogram::Histogram {
    let mut h = rustogram::new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);

    let mut i = 10_000;
    while i != 0 {
        h.record_value_with_expected_interval(1_000, 10_000);
        i -= 1;
    }
    h.record_value_with_expected_interval(100_000_000, 10_000);
    h
}

fn get_raw_histogram() -> rustogram::Histogram {
     let mut h = rustogram::new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);

    let mut i = 10_000;
    while i != 0 {
        h.record_value(1_000);
        i -= 1;
    }
    h.record_value(100_000_000);
    h
}
