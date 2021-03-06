extern crate rustogram;
const HIGHEST_TRACKABLE_VALUE: i64 = 3600 * 1000 * 1000;
const NUMBER_OF_SIGNIFICANT_VALUE_DIGITS: i32 = 3;

use rustogram::histogram::*;
use rustogram::iter::*;

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

    assert_float_eq(expected_raw_mean, raw_histogram.get_mean(), expected_raw_mean * 0.001);
    assert_float_eq(expected_mean, histogram.get_mean(), expected_mean * 0.001);
}

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

#[test]
fn test_get_value_at_percentile() {
    let histogram = get_histogram();
    let raw_histogram = get_raw_histogram();

    assert_float_eq(1000.0, raw_histogram.get_value_at_percentile(30.0) as f64, 1000.0 * 0.001);
    assert_float_eq(1000.0, raw_histogram.get_value_at_percentile(99.0) as f64, 1000.0 * 0.001);
    assert_float_eq(1000.0, raw_histogram.get_value_at_percentile(99.99) as f64, 1000.0 * 0.001);
    assert_float_eq(100000000.0, raw_histogram.get_value_at_percentile(99.999) as f64, 100000000.0 * 0.001);
    assert_float_eq(100000000.0, raw_histogram.get_value_at_percentile(100.0) as f64, 100000000.0 * 0.001);

    assert_float_eq(1000.0, histogram.get_value_at_percentile(30.0) as f64, 1000.0 * 0.001);
    assert_float_eq(1000.0, histogram.get_value_at_percentile(50.0) as f64, 1000.0 * 0.001);
    assert_float_eq(50000000.0, histogram.get_value_at_percentile(75.0) as f64, 50000000.0 * 0.001);
    assert_float_eq(80000000.0, histogram.get_value_at_percentile(90.0) as f64, 80000000.0 * 0.001);
    assert_float_eq(98000000.0, histogram.get_value_at_percentile(99.0) as f64, 98000000.0 * 0.001);
    assert_float_eq(100000000.0, histogram.get_value_at_percentile(99.999) as f64, 100000000.0 * 0.001);
    assert_float_eq(100000000.0, histogram.get_value_at_percentile(100.0) as f64, 100000000.0 * 0.001);
    
}

#[test]
fn test_get_value_at_percentile_for_large_histogram() {
    let largest_value = 1000000000000;
    let mut histogram = new_histogram(largest_value, 5);
    histogram.record_value(largest_value);

    assert!(histogram.get_value_at_percentile(100.0) > 0);
}

#[test]
fn test_get_percentile_at_or_below_value() {
    let histogram = get_histogram();
    let raw_histogram = get_raw_histogram();

    assert_float_eq(99.99, raw_histogram.get_percentile_at_or_below_value(5000), 0.0001);
    assert_float_eq(50.0, histogram.get_percentile_at_or_below_value(5000), 0.0001);
    assert_float_eq(100.0, histogram.get_percentile_at_or_below_value(100000000), 0.0001);
}

#[test]
fn test_get_count_between_values() {
    let histogram = get_histogram();
    let raw_histogram = get_raw_histogram();

    assert_eq!(10_000, raw_histogram.get_count_between_values(1000, 1000));
    assert_eq!(1, raw_histogram.get_count_between_values(5000, 150_000_000));
    assert_eq!(10_000, histogram.get_count_between_values(5000, 150_000_000));
}

#[test]
fn test_get_count_at_value() {
    let histogram = get_histogram();
    let raw_histogram = get_raw_histogram();

    assert_eq!(10000, raw_histogram.get_count_at_value(1000));
    assert_eq!(10000, histogram.get_count_at_value(1000));
}

#[test]
fn test_collect_method_returns_same_as_callback_method_for_recorded_values() {
	let mut value_recorder = ValueRecorder { elements: Vec::new() };
	let histogram = get_histogram();
	let histogram_asserter = |record: Option<(i64, &HistogramIterationValue, &mut ValueRecorder)>| {
		if record.is_some() {
			let index_and_value = record.unwrap();
			let (_, value, counter) = index_and_value;
			counter.add_element(value);
		}
	};
	histogram.get_recorded_values(histogram_asserter, &mut value_recorder);
	
	let mut collected = Vec::new();
	histogram.collect_recorded_values(&mut collected);
	
	assert_eq!(collected.len(), value_recorder.elements.len());
	let mut index = 0;
	for collected_value in collected {
		assert!(collected_value == value_recorder.elements[index]);
		index += 1;
	}
}

#[test]
fn test_collect_method_returns_same_as_callback_method_for_all_values() {
	let mut value_recorder = ValueRecorder { elements: Vec::new() };
	let histogram = get_histogram();
	let histogram_asserter = |record: Option<(i64, &HistogramIterationValue, &mut ValueRecorder)>| {
		if record.is_some() {
			let index_and_value = record.unwrap();
			let (_, value, counter) = index_and_value;
			counter.add_element(value);
		}
	};
	histogram.get_all_values(histogram_asserter, &mut value_recorder);
	
	let mut collected = Vec::new();
	histogram.collect_all_values(&mut collected);
	
	assert_eq!(collected.len(), value_recorder.elements.len());
	let mut index = 0;
	for collected_value in collected {
		assert!(collected_value == value_recorder.elements[index]);
		index += 1;
	}
}

#[test]
fn test_get_recorded_values() {
	let histogram = get_histogram();
	let raw_histogram = get_raw_histogram();
	
	let mut raw_values: Vec<HistogramIterationValue> = Vec::new();
	raw_histogram.collect_recorded_values(&mut raw_values);
	let mut index = 0;
	for value in raw_values {
		if index == 0 {
			assert_eq!(10_000, value.get_count_added_in_this_iteration_step());
		} else {
			assert_eq!(1, value.get_count_added_in_this_iteration_step());
		}
		index += 1;
	}
	
	assert_eq!(2, index);

	let mut values: Vec<HistogramIterationValue> = Vec::new();
	index = 0;
	
	histogram.collect_recorded_values(&mut values);
	let mut total_added_counts = 0;
	
	for value in values {
		if index == 0 {
			assert_eq!(10_000, value.get_count_added_in_this_iteration_step());
		}
			
		assert!(value.get_count_at_value_iterated_to() != 0);
		assert_eq!(value.get_count_at_value_iterated_to(), value.get_count_added_in_this_iteration_step());
		total_added_counts += value.get_count_added_in_this_iteration_step();
		
		index += 1;
	}
	
	assert_eq!(20_000, total_added_counts);
}

#[test]
fn test_get_all_values() {
	let histogram = get_histogram();
	let raw_histogram = get_raw_histogram();
	
	let mut all_raw_values: Vec<HistogramIterationValue> = Vec::new();
	raw_histogram.collect_all_values(&mut all_raw_values); 
	
	let mut index = 0;
	let mut total_count_to_this_point = 0;
	let mut total_value_to_this_point = 0;
	
	for value in all_raw_values {
		if index == 1000 {
			assert_eq!(10000, value.get_count_added_in_this_iteration_step());
		} else if histogram.values_are_equivalent(value.get_value_iterated_to(), 100_000_000) {
			assert_eq!(1, value.get_count_added_in_this_iteration_step());
		} else {
			assert_eq!(0, value.get_count_added_in_this_iteration_step());
		}
		
		let latest_value_at_index = value.get_value_iterated_to();
		total_count_to_this_point += value.get_count_at_value_iterated_to();
		assert_eq!(total_count_to_this_point, value.get_total_count_to_this_value());
		
		total_value_to_this_point += value.get_count_at_value_iterated_to() * latest_value_at_index;
		assert_eq!(total_value_to_this_point, value.get_total_value_to_this_value()); 	
			
		index += 1;
	}
	
	assert_eq!(index, raw_histogram.get_counts_array_length());
	
	let mut all_values: Vec<HistogramIterationValue> = Vec::new();
	index = 0;
	
	histogram.collect_all_values(&mut all_values);
	let mut total_added_counts = 0;
	
	for value in all_values {
		if index == 1000 {
			assert_eq!(10000, value.get_count_added_in_this_iteration_step());
		}
		
		assert_eq!(value.get_count_at_value_iterated_to(), value.get_count_added_in_this_iteration_step());
		total_added_counts += value.get_count_added_in_this_iteration_step();
		
		let value_from_index = histogram.value_from_index(index);
		assert!(histogram.values_are_equivalent(value_from_index, value.get_value_iterated_to()));
		
		index += 1;
	}
	
	assert_eq!(index, histogram.get_counts_array_length());
	assert_eq!(20_000, total_added_counts);
}

fn assert_float_eq(expected: f64, actual: f64, delta: f64) {
    if !(actual > expected - delta && actual < expected + delta) {
        panic!(format!("Expected {} to be equal to {} +/-{}", actual, expected, delta));
    }
}

fn get_histogram() -> Histogram {
    let mut h = new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);

    let mut i = 10_000;
    while i != 0 {
        h.record_value_with_expected_interval(1_000, 10_000);
        i -= 1;
    }
    h.record_value_with_expected_interval(100_000_000, 10_000);
    h
}

fn get_raw_histogram() -> Histogram {
    let mut h = new_histogram(HIGHEST_TRACKABLE_VALUE, NUMBER_OF_SIGNIFICANT_VALUE_DIGITS);

    let mut i = 10_000;
    while i != 0 {
        h.record_value(1_000);
        i -= 1;
    }
    h.record_value(100_000_000);
    h
}

struct ValueRecorder {
	elements: Vec<HistogramIterationValue>
}

impl ValueRecorder {
	fn add_element(&mut self, value: &HistogramIterationValue) {
		let mut copy = HistogramIterationValue::new();
		value.copy_to(&mut copy);
		self.elements.push(copy);
	}
}