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
    let mut histogram = get_histogram();
    let mut raw_histogram = get_raw_histogram();

    let expected_raw_mean = (10_000_000f64 + 100_000_000f64) / 10001f64;
    let expected_mean = (1_000f64 + 50_000_000f64) / 2f64;

    assert_float_eq(expected_raw_mean, raw_histogram.get_mean(), expected_raw_mean * 0.001);
    assert_float_eq(expected_mean, histogram.get_mean(), expected_mean * 0.001);
}

#[test]
fn test_get_value_at_percentile() {
    let histogram = get_histogram();
    let raw_histogram = get_raw_histogram();

    assert_eq!(1_000, raw_histogram.get_value_at_percentile(30f64));
}

#[ignore]
#[test]
fn test_get_std_deviation() {
    let histogram = get_histogram();
    let raw_histogram = get_raw_histogram();

    let expected_raw_mean = (10_000_000f64 + 100_000_000f64) / 10001f64;
    let expected_raw_std_dev = (((10000.0 * (1000.0 - expected_raw_mean).powf(2f64)) + 
                                (100000000.0 - expected_raw_mean).powf(2f64)) / 10001f64).sqrt();
    let expected_mean = (1_000f64 + 50_000_000f64) / 2f64;
    let mut expected_square_deviation_sum = 10_000f64 * (1_000f64 - expected_mean).powf(2f64);
    let mut value = 10_000;
    while value <= 100_000_000 {
        expected_square_deviation_sum += (value as f64 - expected_mean).powf(2f64);
        value += 10_000;
    }
    let expected_std_dev = (expected_square_deviation_sum as f64 / 20_000f64).sqrt();

    assert_float_eq(expected_raw_std_dev, raw_histogram.get_std_deviation(), expected_raw_std_dev * 0.001);
    assert_float_eq(expected_std_dev, histogram.get_std_deviation(), expected_std_dev * 0.001);
}

fn assert_float_eq(expected: f64, actual: f64, delta: f64) {
    // TODO improve error message
    assert!(actual > expected - delta && actual < expected + delta);
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
