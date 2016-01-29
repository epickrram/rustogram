use std::fmt;
use histogram::*;

#[derive(PartialEq)]
pub struct HistogramIterationValue {
    value_iterated_to: i64,
    value_iterated_from: i64,
    count_at_value_iterated_to: i64,
    count_added_in_this_iteration_step: i64,
    total_count_to_this_value: i64,
    total_value_to_this_value: i64,
    percentile: f64,
    percentile_level_iterated_to: f64,
}

impl HistogramIterationValue {
    pub fn new() -> HistogramIterationValue {
        HistogramIterationValue {
            value_iterated_to: 0,
            value_iterated_from: 0,
            count_at_value_iterated_to: 0,
            count_added_in_this_iteration_step: 0,
            total_count_to_this_value: 0,
            total_value_to_this_value: 0,
            percentile: 0.0,
            percentile_level_iterated_to: 0.0,
        }
    }

    fn set(&mut self,
           _value_iterated_to: i64,
           _value_iterated_from: i64,
           _count_at_value_iterated_to: i64,
           _count_added_in_this_iteration_step: i64,
           _total_count_to_this_value: i64,
           _total_value_to_this_value: i64,
           _percentile: f64,
           _percentile_level_iterated_to: f64) {
        self.value_iterated_to = _value_iterated_to;
        self.value_iterated_from = _value_iterated_from;
        self.count_at_value_iterated_to = _count_at_value_iterated_to;
        self.count_added_in_this_iteration_step = _count_added_in_this_iteration_step;
        self.total_count_to_this_value = _total_count_to_this_value;
        self.total_value_to_this_value = _total_value_to_this_value;
        self.percentile = _percentile;
        self.percentile_level_iterated_to = _percentile_level_iterated_to;
    }

    fn reset(&mut self) {
        self.value_iterated_to = 0;
        self.value_iterated_from = 0;
        self.count_at_value_iterated_to = 0;
        self.count_added_in_this_iteration_step = 0;
        self.total_count_to_this_value = 0;
        self.total_value_to_this_value = 0;
        self.percentile = 0.0;
        self.percentile_level_iterated_to = 0.0;
    }

    pub fn copy_to(&self, target: &mut HistogramIterationValue) {
        target.value_iterated_to = self.value_iterated_to;
        target.value_iterated_from = self.value_iterated_from;
        target.count_at_value_iterated_to = self.count_at_value_iterated_to;
        target.count_added_in_this_iteration_step = self.count_added_in_this_iteration_step;
        target.total_count_to_this_value = self.total_count_to_this_value;
        target.total_value_to_this_value = self.total_value_to_this_value;
        target.percentile = self.percentile;
        target.percentile_level_iterated_to = self.percentile_level_iterated_to;
    }

    pub fn get_value_iterated_to(&self) -> i64 {
        self.value_iterated_to
    }

    pub fn get_count_at_value_iterated_to(&self) -> i64 {
        self.count_at_value_iterated_to
    }

    pub fn get_count_added_in_this_iteration_step(&self) -> i64 {
        self.count_added_in_this_iteration_step
    }

    pub fn get_total_value_to_this_value(&self) -> i64 {
        self.total_value_to_this_value
    }

    pub fn get_total_count_to_this_value(&self) -> i64 {
        self.total_count_to_this_value
    }
}

struct IteratorSharedState {
    saved_histogram_total_raw_count: i64,
    current_index: i32,
    current_value_at_index: i64,
    next_value_at_index: i64,
    prev_value_iterated_to: i64,
    total_count_to_prev_index: i64,
    total_count_to_current_index: i64,
    total_value_to_current_index: i64,
    array_total_count: i64,
    count_at_this_value: i64,
    fresh_sub_bucket: bool,
    visited_index: i32,
    current_iteration_value: HistogramIterationValue,
}

impl IteratorSharedState {
    fn exhausted_sub_buckets(&self, histogram: &Histogram) -> bool {
        self.current_index >= histogram.get_counts_array_length()
    }
    
    fn reset(&mut self, total_count: i64, unit_magnitude: i32) {
        self.saved_histogram_total_raw_count = total_count;
        self.array_total_count = total_count;
        self.current_index = 0;
        self.current_value_at_index = 0;
        self.next_value_at_index = 1 << unit_magnitude;
        self.prev_value_iterated_to = 0;
        self.total_count_to_prev_index = 0;
        self.total_count_to_current_index = 0;
        self.total_value_to_current_index = 0;
        self.count_at_this_value = 0;
        self.fresh_sub_bucket = true;
        self.visited_index = -1;
        self.current_iteration_value.reset();
    }
    
    fn get_percentile_iterated_to(&self) -> f64 {
        (100.0f64 * self.total_count_to_current_index as f64) / self.array_total_count as f64
    }
    
    fn get_value_iterated_to(&self, histogram: &Histogram) -> i64 {
        histogram.highest_equivalent_value(self.current_value_at_index)
    }

    fn increment_iteration_level(&mut self) {
        self.visited_index = self.current_index;
    }
    
    fn increment_sub_bucket(&mut self, histogram: &Histogram) {
        self.fresh_sub_bucket = true;
        self.current_index += 1;
        self.current_value_at_index = histogram.value_from_index(self.current_index);
        self.next_value_at_index = histogram.value_from_index(self.current_index + 1);
    }
    
    fn next<F>(&mut self, histogram: &Histogram, level_reached_function: F) -> &HistogramIterationValue 
    		where F: Fn(&mut IteratorSharedState, &Histogram) -> bool {
        while !self.exhausted_sub_buckets(histogram) {
            self.count_at_this_value = histogram.get_count_at_index(self.current_index);
            if self.fresh_sub_bucket {
                self.total_count_to_current_index += self.count_at_this_value;
                self.total_value_to_current_index +=
                    self.count_at_this_value *
                    histogram.highest_equivalent_value(self.current_value_at_index);
                self.fresh_sub_bucket = false;
            }

            if level_reached_function(self, histogram) {
                let value_iterated_to = self.get_value_iterated_to(histogram);
                let percentile_iterated_to = self.get_percentile_iterated_to();
                self.current_iteration_value.set(value_iterated_to,
                                                 self.prev_value_iterated_to,
                                                 self.count_at_this_value,
                                                 (self.total_count_to_current_index -
                                                  self.total_count_to_prev_index),
                                                 self.total_count_to_current_index,
                                                 self.total_value_to_current_index,
                                                 ((100.0f64 *
                                                   self.total_count_to_current_index as f64) /
                                                  (self.array_total_count as f64)),
                                                 percentile_iterated_to);
                self.prev_value_iterated_to = value_iterated_to;
                self.total_count_to_prev_index = self.total_count_to_current_index;
                self.increment_iteration_level();

                return &self.current_iteration_value;
            }

            self.increment_sub_bucket(histogram);
        }
        panic!("Histogram may have overflowed!")
    }
}

pub struct AllValuesIterator<'a> {
    histogram: &'a Histogram,
    state: IteratorSharedState
}

pub fn new_all_values_iterator<'a>(_histogram: &'a Histogram) -> AllValuesIterator {
    AllValuesIterator {
        histogram: _histogram,
        state: IteratorSharedState {
	        saved_histogram_total_raw_count: 0,
	        current_index: 0,
	        current_value_at_index: 0,
	        next_value_at_index: 0,
	        prev_value_iterated_to: 0,
	        total_count_to_prev_index: 0,
	        total_count_to_current_index: 0,
	        total_value_to_current_index: 0,
	        array_total_count: 0,
	        count_at_this_value: 0,
	        fresh_sub_bucket: true,
	        visited_index: -1,
	        current_iteration_value: HistogramIterationValue::new(),
        }
    }
}

impl<'a> AllValuesIterator<'a> {
    pub fn has_next(&mut self) -> bool {
        self.state.current_index < (self.histogram.get_counts_array_length() - 1)
    }

    pub fn reset(&mut self, total_count: i64, unit_magnitude: i32) {
        self.state.reset(total_count, unit_magnitude);
    }

    pub fn next(&mut self) -> &HistogramIterationValue {
    	self.state.next(self.histogram, |iterator_state: &mut IteratorSharedState, _histogram: &Histogram| {
    			iterator_state.visited_index != iterator_state.current_index
    	})
    }
}


pub struct RecordedValuesIterator<'a> {
    histogram: &'a Histogram,
    state: IteratorSharedState
}

pub fn new_iterator<'a>(_histogram: &'a Histogram) -> RecordedValuesIterator {
    RecordedValuesIterator {
        histogram: _histogram,
        state: IteratorSharedState {
	        saved_histogram_total_raw_count: 0,
	        current_index: 0,
	        current_value_at_index: 0,
	        next_value_at_index: 0,
	        prev_value_iterated_to: 0,
	        total_count_to_prev_index: 0,
	        total_count_to_current_index: 0,
	        total_value_to_current_index: 0,
	        array_total_count: 0,
	        count_at_this_value: 0,
	        fresh_sub_bucket: true,
	        visited_index: -1,
	        current_iteration_value: HistogramIterationValue::new(),
        }
    }
}

impl<'a> RecordedValuesIterator<'a> {
    pub fn has_next(&mut self) -> bool {
        self.state.total_count_to_current_index < self.state.array_total_count
    }
    
    pub fn reset(&mut self, total_count: i64, unit_magnitude: i32) {
        self.state.reset(total_count, unit_magnitude);
    }

    pub fn next(&mut self) -> &HistogramIterationValue {
    	self.state.next(self.histogram, |iterator_state: &mut IteratorSharedState, histogram: &Histogram| {
	        let current_count = histogram.get_count_at_index(iterator_state.current_index);
    	    (current_count != 0) && (iterator_state.visited_index != iterator_state.current_index)
    	})
    }
}

impl fmt::Display for HistogramIterationValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Value["));
        try!(write!(f, "value_iterated_to: {}, ", self.value_iterated_to));
        try!(write!(f, "value_iterated_from: {}, ", self.value_iterated_from));
        try!(write!(f,
                    "count_at_value_iterated_to: {}, ",
                    self.count_at_value_iterated_to));
        try!(write!(f,
                    "count_added_in_this_iteration_step: {}, ",
                    self.count_added_in_this_iteration_step));
        try!(write!(f,
                    "total_count_to_this_value: {}, ",
                    self.total_count_to_this_value));
        try!(write!(f,
                    "total_value_to_this_value: {}, ",
                    self.total_value_to_this_value));
        try!(write!(f, "percentile: {}, ", self.percentile));
        try!(write!(f,
                    "percentile_level_iterated_to: {}, ",
                    self.percentile_level_iterated_to));

        write!(f, "]")
    }
}

