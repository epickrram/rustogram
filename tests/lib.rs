extern crate rustogram;
const HIGHEST_TRACKABLE_VALUE: i64 = 3600 * 1000 * 1000;
const NUMBER_OF_SIGNIFICANT_VALUE_DIGITS: i32 = 3;
const TEST_VALUE_LEVEL: i64 = 4;

#[test]
fn test_empty_histogram() {
    let mut histogram = rustogram::new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);

    assert_eq!(0, histogram.get_min_value());
    assert_eq!(0, histogram.get_max_value());
    assert_eq!(0.0, histogram.get_mean());
    assert_eq!(0.0, histogram.get_std_deviation());
    assert_eq!(0.0, histogram.get_percentile_at_or_below_value());
}

#[test]
fn test_construction_argument_gets() {
    let histogram = rustogram::new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);

    assert_eq!(1, histogram.get_lowest_discernible_value());
    assert_eq!(HIGHEST_TRACKABLE_VALUE, histogram.get_highest_trackable_value());
    assert_eq!(NUMBER_OF_SIGNIFICANT_VALUE_DIGITS, histogram.get_number_of_significant_value_digits());
}

#[test]
fn test_record_value() {
    let mut histogram = rustogram::new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);

    histogram.record_value(TEST_VALUE_LEVEL);

    assert_eq!(1, histogram.get_total_count());
    assert_eq!(1, histogram.get_count_at_value(TEST_VALUE_LEVEL));
    assert_eq!(TEST_VALUE_LEVEL, histogram.get_max_value());
}

#[test]
fn test_construction_with_large_numbers() {
    let mut histogram = rustogram::new_histogram_lower_bound(20000000, 100000000, 5);

    histogram.record_value(100000000);
    println!("recorded value 1");
    histogram.record_value(20000000);
    histogram.record_value(30000000);

    assert!(histogram.values_are_equivalent(20000000, histogram.get_value_at_percentile(50.0)));
    assert!(histogram.values_are_equivalent(30000000, histogram.get_value_at_percentile(83.33)));
    assert!(histogram.values_are_equivalent(100000000, histogram.get_value_at_percentile(83.34)));
    assert!(histogram.values_are_equivalent(100000000, histogram.get_value_at_percentile(99.0)));
}

#[test]
fn test_reset() {
    let mut histogram = rustogram::new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);

    histogram.record_value(TEST_VALUE_LEVEL);

    histogram.reset();

    assert_eq!(0, histogram.get_count_at_value(TEST_VALUE_LEVEL));
    assert_eq!(0, histogram.get_total_count());
    verify_max_value(histogram);
}

#[test]
fn test_get_min_value() {
    let mut histogram = rustogram::new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);

    histogram.record_value(1_000);
    histogram.record_value(1_000_000);

    assert_eq!(1_000, histogram.get_min_value());
}

#[test]
fn test_get_max_value() {
   let mut histogram = rustogram::new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);
    histogram.record_value(1_000);
    histogram.record_value(1_000_000);

    assert_eq!(1_000_000, histogram.get_max_value());
}

#[test]
fn test_get_mean() {

}

fn verify_max_value(histogram: rustogram::Histogram) {
    let mut computed_max_value: i64 = 0;
    for i in 0..histogram.get_counts_array_length() {
        if histogram.get_count_at_index(i) > 0 {
            computed_max_value = histogram.value_from_index(i);
        }
    }

    computed_max_value = if computed_max_value == 0 { 0 } else { histogram.highest_equivalent_value(computed_max_value) };

    assert_eq!(computed_max_value, histogram.get_max_value());
}


