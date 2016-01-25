use std::cmp;
use std::fmt;
use std;
use iter::*;

pub fn new_histogram(_highest_trackable_value: i64,
                     _number_of_significant_digits: i32)
                     -> Histogram {
    new_histogram_lower_bound(1, _highest_trackable_value, _number_of_significant_digits)
}

pub fn new_histogram_lower_bound(_lowest_discernible_value: i64,
                                 _highest_trackable_value: i64,
                                 _number_of_significant_digits: i32)
                                 -> Histogram {
    let largest_value_with_single_unit_resolution = 2 *
                                                    10i64.pow(_number_of_significant_digits as u32);
    let _unit_magnitude = ((_lowest_discernible_value as f64).ln() / 2f64.ln()) as i32;
    let sub_bucket_count_magnitude = ((largest_value_with_single_unit_resolution as f64).ln() /
                                      2f64.ln())
                                         .ceil() as i32;
    let _sub_bucket_half_count_magnitude = if sub_bucket_count_magnitude > 1 {
        sub_bucket_count_magnitude - 1
    } else {
        0
    };
    let _sub_bucket_count = 2i64.pow((_sub_bucket_half_count_magnitude + 1) as u32) as i32;
    let _sub_bucket_half_count = _sub_bucket_count / 2;
    let _sub_bucket_mask = ((_sub_bucket_count as i64) - 1) << _unit_magnitude;

    let _leading_zero_count_base = 64 - _unit_magnitude - _sub_bucket_half_count_magnitude - 1;

    // establish size (highest_trackable_value)
    let _counts_array_length = determine_array_length_needed(_highest_trackable_value,
                                                             _sub_bucket_count,
                                                             _unit_magnitude);
    let _bucket_count = get_buckets_needed_to_cover_value(_highest_trackable_value,
                                                          _sub_bucket_count,
                                                          _unit_magnitude);

    Histogram {
        values: vec![0; _counts_array_length as usize].into_boxed_slice(),
        total_count: 0,
        highest_trackable_value: _highest_trackable_value,
        lowest_discernible_value: _lowest_discernible_value,
        number_of_significant_digits: _number_of_significant_digits,
        bucket_count: _bucket_count,
        sub_bucket_count: _sub_bucket_count,
        counts_array_length: _counts_array_length,
        word_size_in_bytes: 8,
        unit_magnitude: _unit_magnitude,
        sub_bucket_half_count_magnitude: _sub_bucket_half_count_magnitude,
        sub_bucket_half_count: _sub_bucket_half_count,
        leading_zero_count_base: _leading_zero_count_base,
        sub_bucket_mask: _sub_bucket_mask,
        max_value: 0,
        min_non_zero_value: std::i64::MAX,
    }

}

pub struct Histogram {
    values: Box<[i64]>,
    total_count: i64,
    highest_trackable_value: i64,
    lowest_discernible_value: i64,
    number_of_significant_digits: i32,
    bucket_count: i32,
    sub_bucket_count: i32,
    counts_array_length: i32,
    word_size_in_bytes: i32,
    unit_magnitude: i32,
    sub_bucket_half_count_magnitude: i32,
    sub_bucket_half_count: i32,
    leading_zero_count_base: i32,
    sub_bucket_mask: i64,
    max_value: i64,
    min_non_zero_value: i64,
}

impl Histogram {
    pub fn get_count_at_index(&self, index: i32) -> i64 {
        self.values[index as usize]
    }

    pub fn value_from_index(&self, index: i32) -> i64 {
        let mut bucket_index: i32 = (index >> self.sub_bucket_half_count_magnitude) - 1;
        let mut sub_bucket_index: i32 = (index & (self.sub_bucket_half_count - 1)) +
                                        self.sub_bucket_half_count;
        if bucket_index < 0 {
            sub_bucket_index -= self.sub_bucket_half_count;
            bucket_index = 0;
        }
        self.value_from_index_by_bucket(bucket_index, sub_bucket_index)
    }

    pub fn lowest_equivalent_value(&self, value: i64) -> i64 {
        let bucket_index = self.get_bucket_index(value);
        let sub_bucket_index = self.get_sub_bucket_index(value, bucket_index);
        self.value_from_index_by_bucket(bucket_index, sub_bucket_index)
    }

    pub fn highest_equivalent_value(&self, value: i64) -> i64 {
        self.next_non_equivalent_value(value) - 1
    }

    pub fn record_value_with_expected_interval(&mut self,
                                               value: i64,
                                               expected_interval_between_value_samples: i64) {
        if expected_interval_between_value_samples <= 0 {
            return;
        }
        self.record_value(value);
        let mut missing_value = value - expected_interval_between_value_samples;
        while missing_value >= expected_interval_between_value_samples {
            self.record_single_value(missing_value);
            missing_value -= expected_interval_between_value_samples;
        }
    }

    pub fn record_value(&mut self, value: i64) {
        self.record_single_value(value);
    }

    pub fn get_min_value(&self) -> i64 {
        if self.min_non_zero_value == std::i64::MAX {
            0
        } else {
            self.min_non_zero_value
        }
    }

    pub fn get_max_value(&self) -> i64 {
        self.max_value
    }

    pub fn get_mean(&self) -> f64 {
        if self.total_count == 0 {
            return 0f64;
        }

        let mut iter = new_iterator(self);

        iter.reset(self.total_count, self.unit_magnitude);
        let mut total_value = 0.0f64;

        while iter.has_next() {
            let iteration_value = iter.next();
            total_value +=
                (self.median_equivalent_value(iteration_value.get_value_iterated_to()) *
                 iteration_value.get_count_at_value_iterated_to()) as f64;
        }

        return total_value / self.total_count as f64;
    }

    pub fn get_std_deviation(&self) -> f64 {
        if self.total_count == 0 {
            return 0f64;
        }

        let mean = self.get_mean();
        let mut geometric_deviation_total = 0f64;

        let mut iter = new_iterator(self);

        iter.reset(self.total_count, self.unit_magnitude);
        while iter.has_next() {
            let iteration_value = iter.next();
            let deviation =
                self.median_equivalent_value(iteration_value.get_value_iterated_to()) as f64 - mean;
            geometric_deviation_total +=
                (deviation * deviation) *
                iteration_value.get_count_added_in_this_iteration_step() as f64;
        }

        (geometric_deviation_total / self.total_count as f64).sqrt()
    }

    pub fn get_percentile_at_or_below_value(&self, value: i64) -> f64 {
        if self.total_count == 0 {
            return 100f64;
        }
        let counts_array_index = self.counts_array_index(value);
        let target_index = cmp::min(counts_array_index, self.counts_array_length - 1);

        let mut total_to_current_index = 0i64;

        for i in 0..(target_index + 1) {
            total_to_current_index += self.get_count_at_index(i);
        }

        (100 * total_to_current_index) as f64 / self.total_count as f64
    }

    pub fn get_count_between_values(&self, lower: i64, upper: i64) -> i64 {
        let low_index = cmp::max(0, self.counts_array_index(lower));
        let high_index = cmp::min(self.counts_array_index(upper), self.counts_array_length - 1);
        let mut count = 0i64;
        for i in low_index..(high_index + 1) {
            count += self.get_count_at_index(i as i32);
        }
        count
    }

    pub fn reset(&mut self) {
        self.total_count = 0;
        for i in 0..self.counts_array_length {
            self.values[i as usize] = 0;
        }
        self.max_value = 0;
        self.min_non_zero_value = std::i64::MAX;
    }

    pub fn get_counts_array_length(&self) -> i32 {
        self.counts_array_length
    }

    pub fn get_value_at_percentile(&self, percentile: f64) -> i64 {
        let requested_percentile = percentile.min(100f64);
        let count_at_percentile =
            cmp::max((((requested_percentile / 100f64) * self.get_total_count() as f64) +
                      0.5f64) as i64,
                     1i64);
        let mut total_to_current_index: i64 = 0;
        for i in 0..self.counts_array_length {
            total_to_current_index += self.get_count_at_index(i);
            if total_to_current_index >= count_at_percentile {
                let value_at_index: i64 = self.value_from_index(i);
                if percentile == 0f64 {
                    return self.lowest_equivalent_value(value_at_index);
                } else {
                    return self.highest_equivalent_value(value_at_index);
                }
            }
        }

        0
    }

    pub fn values_are_equivalent(&self, value_one: i64, value_two: i64) -> bool {
        self.lowest_equivalent_value(value_one) == self.lowest_equivalent_value(value_two)
    }

    pub fn get_lowest_discernible_value(&self) -> i64 {
        self.lowest_discernible_value
    }

    pub fn get_highest_trackable_value(&self) -> i64 {
        self.highest_trackable_value
    }

    pub fn get_number_of_significant_value_digits(&self) -> i32 {
        self.number_of_significant_digits
    }

    pub fn get_count_at_value(&self, value: i64) -> i64 {
        let counts_array_index = self.counts_array_index(value);
        let counts_idx = if counts_array_index < 0 {
            0
        } else {
            counts_array_index
        };
        let index = cmp::min(counts_idx, self.counts_array_length - 1);
        self.values[index as usize]
    }

    pub fn get_total_count(&self) -> i64 {
        self.total_count
    }
    
    pub fn get_recorded_values<F, T>(&self, f: F, t: &mut T) where F: Fn(Option<(i64, &HistogramIterationValue, &mut T)>) {
    	let mut iter = new_iterator(self);
    	iter.reset(self.total_count, self.unit_magnitude);
    	let mut index = 0;
    	while iter.has_next() {
    		f(Some((index, iter.next(), t)));
    		index += 1;
    	}
    	f(None)
    }

    fn increment_total_count(&mut self) {
        self.total_count += 1;
    }

    fn counts_array_index_by_bucket(&self, bucket_index: i32, sub_bucket_index: i32) -> i32 {
        let bucket_base_index = (bucket_index + 1) << self.sub_bucket_half_count_magnitude;
        let offset_in_bucket = sub_bucket_index - self.sub_bucket_half_count;


        bucket_base_index + offset_in_bucket
    }

    fn increment_count_at_index(&mut self, counts_index: i32) {
        self.values[counts_index as usize] += 1;
    }

    fn update_min_and_max(&mut self, value: i64) {
        if value > self.max_value {
            self.max_value = value;
        }
        if value != 0 && value < self.min_non_zero_value {
            self.min_non_zero_value = value;
        }
    }

    fn get_bucket_index(&self, value: i64) -> i32 {
        (self.leading_zero_count_base as i64 -
         (value | self.sub_bucket_mask as i64).leading_zeros() as i64) as i32
    }

    fn get_sub_bucket_index(&self, value: i64, bucket_index: i32) -> i32 {
        ((value as u64) >> (bucket_index + self.unit_magnitude)) as i32
    }

    fn counts_array_index(&self, value: i64) -> i32 {
        let bucket_index = self.get_bucket_index(value);
        let sub_bucket_index = self.get_sub_bucket_index(value, bucket_index);
        self.counts_array_index_by_bucket(bucket_index, sub_bucket_index)
    }

    fn record_single_value(&mut self, value: i64) {
        let counts_index = self.counts_array_index(value);
        // TODO handle out of bounds exceptions
        self.increment_count_at_index(counts_index);
        self.update_min_and_max(value);
        self.increment_total_count();
    }

    fn value_from_index_by_bucket(&self, bucket_index: i32, sub_bucket_index: i32) -> i64 {
        (sub_bucket_index as i64) << (bucket_index + self.unit_magnitude)
    }

    fn size_of_equivalent_value_range(&self, value: i64) -> i64 {
        let bucket_index = self.get_bucket_index(value);
        let sub_bucket_index = self.get_sub_bucket_index(value, bucket_index);
        let mult = if sub_bucket_index >= self.sub_bucket_count {
            bucket_index + 1
        } else {
            bucket_index
        };
        1i64 << (self.unit_magnitude + mult)
    }

    fn next_non_equivalent_value(&self, value: i64) -> i64 {
        self.lowest_equivalent_value(value) + self.size_of_equivalent_value_range(value)
    }


    fn median_equivalent_value(&self, value: i64) -> i64 {
        self.lowest_equivalent_value(value) + (self.size_of_equivalent_value_range(value) >> 1)
    }
}

impl fmt::Display for Histogram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Histogram["));
        try!(write!(f, "total_count: {}, ", self.total_count));
        try!(write!(f,
                    "highest_trackable_value: {}, ",
                    self.highest_trackable_value));
        try!(write!(f,
                    "lowest_discernible_value: {}, ",
                    self.lowest_discernible_value));
        try!(write!(f,
                    "number_of_significant_digits: {}, ",
                    self.number_of_significant_digits));
        try!(write!(f, "bucket_count: {}, ", self.bucket_count));
        try!(write!(f, "sub_bucket_count: {}, ", self.sub_bucket_count));
        try!(write!(f, "counts_array_length: {}, ", self.counts_array_length));
        try!(write!(f, "word_size_in_bytes: {}, ", self.word_size_in_bytes));
        write!(f, "]")

    }
}

fn determine_array_length_needed(highest_trackable_value: i64,
                                 sub_bucket_count: i32,
                                 unit_magnitude: i32)
                                 -> i32 {
    // TODO error handling if highest < 2 * lowest_discernible
    let buckets_needed_to_cover_value = get_buckets_needed_to_cover_value(highest_trackable_value,
                                                                          sub_bucket_count,
                                                                          unit_magnitude);

    (buckets_needed_to_cover_value + 1) * (sub_bucket_count / 2)
}

fn get_buckets_needed_to_cover_value(highest_trackable_value: i64,
                                     sub_bucket_count: i32,
                                     unit_magnitude: i32)
                                     -> i32 {
    let mut smallest_untrackable_value = ((sub_bucket_count as i64) << unit_magnitude) as i64;
    let mut buckets_needed: i32 = 1;
    while smallest_untrackable_value <= highest_trackable_value {

        if smallest_untrackable_value > (std::i64::MAX / 2) {
            buckets_needed += 1;
            return buckets_needed;
        }

        smallest_untrackable_value <<= 1;
        buckets_needed += 1;
    }
    buckets_needed
}
