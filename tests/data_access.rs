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

#[test]
fn test_get_min_value() {
    let histogram = get_histogram();
    assert!(histogram.values_are_equivalent(1_000, histogram.get_min_value()));
}

#[test]
fn test_get_mean_value() {
    let histogram = get_histogram();
    let raw_histogram = get_raw_histogram();

    let expected_raw_mean = (10_000_000f64 + 100_000_000f64) / 10001f64;
    let expected_mean = (1_000f64 + 50_000_000f64) / 2f64;

    assert_eq!(expected_raw_mean, raw_histogram.get_mean());
    assert_eq!(expected_mean, histogram.get_mean());
}

#[test]
fn test_get_value_at_percentile() {
    let histogram = get_histogram();
    let raw_histogram = get_raw_histogram();

    assert_eq!(1_000, raw_histogram.get_value_at_percentile(30f64));
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
