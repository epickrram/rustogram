use std::cmp;
use std::fmt;
use std;
use iter::*;
use encoding::*;

const I32_BYTES: i32 = 4;
const I64_BYTES: i32 = 8;

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

pub fn deserialise_histogram(byte_array: &Vec<u8>, offset: i32) -> Option<Histogram> {
	let cookie = get_i32(byte_array, offset);
	if cookie != (0x1c849303i32 | 0x10i32) {
		return None;
	}
	let payload_length_in_bytes = get_i32(byte_array, offset + I32_BYTES);
	// TODO assert that normalising_index_offset is zero - any other value is unsupported
//	let normalising_index_offset = get_i32(byte_array, offset + 2 * I32_BYTES);
	let number_of_significant_digits = get_i32(byte_array, offset + 3 * I32_BYTES);
	let lowest_trackable_unit_value = get_i64(byte_array, offset + 4 * I32_BYTES);
	let highest_trackable_value = get_i64(byte_array, offset + (4 * I32_BYTES) + I64_BYTES);
//	let placeholder = get_i64(byte_array, offset + (4 * I32_BYTES) + (2 * I64_BYTES));
	
	let mut histogram = new_histogram_lower_bound(lowest_trackable_unit_value, highest_trackable_value, number_of_significant_digits);
	let filled_length = histogram.fill_counts_array_from_source_buffer(byte_array, (4 * I32_BYTES) + (3 * I64_BYTES), payload_length_in_bytes, I64_BYTES);
	histogram.establish_internal_tracking_values(filled_length);
	
	Some(histogram)
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

    pub fn get_recorded_values<F, T>(&self, f: F, t: &mut T)
        where F: Fn(Option<(i64, &HistogramIterationValue, &mut T)>)
    {
        let mut iter = new_iterator(self);
        iter.reset(self.total_count, self.unit_magnitude);
        let mut index = 0;
        while iter.has_next() {
            f(Some((index, iter.next(), t)));
            index += 1;
        }
        f(None)
    }

    pub fn collect_recorded_values(&self, container: &mut Vec<HistogramIterationValue>) {
        let mut iter = new_iterator(self);
        iter.reset(self.total_count, self.unit_magnitude);

        while iter.has_next() {
            let mut value = HistogramIterationValue::new();
            iter.next().copy_to(&mut value);
            container.push(value);
        }
    }

    pub fn get_all_values<F, T>(&self, f: F, t: &mut T)
        where F: Fn(Option<(i64, &HistogramIterationValue, &mut T)>)
    {
        let mut iter = new_all_values_iterator(self);
        iter.reset(self.total_count, self.unit_magnitude);
        let mut index = 0;
        while iter.has_next() {
            f(Some((index, iter.next(), t)));
            index += 1;
        }
        f(None)
    }

    pub fn collect_all_values(&self, container: &mut Vec<HistogramIterationValue>) {
        let mut iter = new_all_values_iterator(self);
        iter.reset(self.total_count, self.unit_magnitude);

        while iter.has_next() {
            let mut value = HistogramIterationValue::new();
            iter.next().copy_to(&mut value);
            container.push(value);
        }
    }
    
    pub fn serialise(&self, target_buffer: &mut Vec<u8>) {
    	put_i32(0x1c849303i32 | 0x10i32, target_buffer);

    	let index_of_payload_length = target_buffer.len() as i32;
    	put_i32(0, target_buffer);
    	// normalising index offset - always 0
    	put_i32(0, target_buffer);
    	put_i32(self.number_of_significant_digits, target_buffer);
    	put_i64(self.lowest_discernible_value, target_buffer);
    	put_i64(self.highest_trackable_value, target_buffer);
    	// value conversion ratio - currently unsupported
    	put_i64(0, target_buffer);
    	
    	let counts_payload_length = self.fill_buffer_from_counts_array(target_buffer);
    	
    	put_i32_at_offset(counts_payload_length, target_buffer, index_of_payload_length);
    }
    
    fn establish_internal_tracking_values(&mut self, length_to_cover: i32) {
    	let mut max_index: i32 = -1;
    	let mut min_non_zero_index: i32 = -1;
    	let mut observed_total_count: i64 = 0;
    	
    	for index in 0..length_to_cover {
    		let count_at_index = self.get_count_at_index(index);
            println!("count_at_index[{}] = {}", index, count_at_index);
    		if count_at_index > 0 {
    			observed_total_count += count_at_index;
    			max_index = index;
    			if min_non_zero_index == -1 && index != 0 {
    				min_non_zero_index = index;
    			}
    		}
    	}
    	
    	if max_index >= 0 {
    		let value_from_index = self.value_from_index(max_index);
    		let max_equivalent_value = self.highest_equivalent_value(value_from_index);
    		self.max_value = max_equivalent_value;
    	}
    	
    	if min_non_zero_index >= 0 {
    		let value_from_index = self.value_from_index(min_non_zero_index);
    		self.min_non_zero_value = value_from_index;
    	}
    	
    	self.total_count = observed_total_count;
    }
    
    fn fill_buffer_from_counts_array(&self, target_buffer: &mut Vec<u8>) -> i32 {
    	let max_value = self.max_value;
    	let counts_limit = self.counts_array_index(max_value) + 1;
    	let mut src_index = 0;
    	let buffer_start_length = target_buffer.len();
    	
    	while src_index < counts_limit {
    		let count = self.get_count_at_index(src_index);
    		src_index += 1;
    		
    		if count < 0 {
    			panic!("Cannot encode histogram containing negative counts!");
    		}
    		
    		let mut zeroes_count = 0;
    		if count == 0 {
    			zeroes_count = 1;
    			while src_index < counts_limit && self.get_count_at_index(src_index) == 0 {
    				zeroes_count += 1;
    				src_index += 1;
    			}
    		}
    		
    		if zeroes_count > 1 {
    			encode(-zeroes_count, target_buffer);
    		} else {
    			encode(count, target_buffer);
    		}
    		
    	}
    	(target_buffer.len() - buffer_start_length) as i32
    }
    
    fn fill_counts_array_from_source_buffer(&mut self, source_buffer: &Vec<u8>, offset: i32, length_in_bytes: i32, word_size_in_bytes: i32) -> i32 {
    	let end_position = offset + length_in_bytes;
    	let mut offset_within_payload = offset;
    	let mut dst_index = 0;
    	while offset_within_payload < end_position {
    		
    		let mut zeroes_count: i32 = 0;
    		
    		if word_size_in_bytes != I64_BYTES {
    			panic!("Only 8-byte word size is supported. Is input buffer in v2 format?");
    		}
    		let (value, length) = decode(source_buffer, offset_within_payload);
            println!("Decoded value {} of length {} from source buffer", value, length);
    		let count = value;
    		offset_within_payload += length;
    		if count < 0 {
    			let zc = -value;
    			if zc > std::i64::MAX {
    				panic!("An encoded zero count of > i64::MAX was encountered in the source");
    			}
    			
    			zeroes_count = zc as i32;
    		}
    		
    		if zeroes_count > 0 {
    			dst_index += zeroes_count;
                println!("Advanced dst_index to {} due to {} zeroes", dst_index, zeroes_count);
    		} else {
                println!("Setting count to {} at index {}", count, dst_index);
    			self.set_count_at_index(dst_index, count);
                dst_index += 1;
    		}
    	}
    	
    	dst_index
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
    
    fn set_count_at_index(&mut self, counts_index: i32, value: i64) {
    	self.values[counts_index as usize] = value;
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
