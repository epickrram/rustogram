use std::fmt;
use histogram::*;

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
    fn new() -> HistogramIterationValue {
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

    pub fn set(&mut self,
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

    pub fn reset(&mut self) {
        self.value_iterated_to = 0;
        self.value_iterated_from = 0;
        self.count_at_value_iterated_to = 0;
        self.count_added_in_this_iteration_step = 0;
        self.total_count_to_this_value = 0;
        self.total_value_to_this_value = 0;
        self.percentile = 0.0;
        self.percentile_level_iterated_to = 0.0;
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

impl<'a> fmt::Display for RecordedValuesIterator<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Iterator["));
        try!(write!(f, "saved_count: {}, ", self.saved_histogram_total_raw_count));
        try!(write!(f, "current_index: {}, ", self.current_index));
        try!(write!(f,
                    "current_value_at_index: {}, ",
                    self.current_value_at_index));
        try!(write!(f, "next_value_at_index: {}, ", self.next_value_at_index));
        try!(write!(f,
                    "prev_value_iterated_to: {}, ",
                    self.prev_value_iterated_to));
        try!(write!(f,
                    "total_count_to_prev_index: {}, ",
                    self.total_count_to_prev_index));
        try!(write!(f,
                    "total_count_to_current_index: {}, ",
                    self.total_count_to_current_index));
        try!(write!(f,
                    "total_value_to_current_index: {}, ",
                    self.total_value_to_current_index));
        try!(write!(f, "array_total_count: {}, ", self.array_total_count));
        try!(write!(f, "count_at_this_value: {}, ", self.count_at_this_value));
        try!(write!(f, "fresh_sub_bucket: {}, ", self.fresh_sub_bucket));
        try!(write!(f, "visited_index: {}, ", self.visited_index));

        write!(f, "]")
    }
}

pub struct RecordedValuesIterator<'a> {
    histogram: &'a Histogram,
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

pub fn new_iterator<'a>(_histogram: &'a Histogram) -> RecordedValuesIterator {
    RecordedValuesIterator {
        histogram: _histogram,
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

impl<'a> RecordedValuesIterator<'a> {
    pub fn reset(&mut self, total_count: i64, unit_magnitude: i32) {
        self.reset_iterator(total_count, unit_magnitude);
    }

    pub fn has_next(&mut self) -> bool {
        self.total_count_to_current_index < self.array_total_count
    }

    pub fn next(&mut self) -> &HistogramIterationValue {
        println!("\n\nstart of next: {}\n\n", self);
        while !self.exhausted_sub_buckets() {
            self.count_at_this_value = self.histogram.get_count_at_index(self.current_index);
            if self.fresh_sub_bucket {
                self.total_count_to_current_index += self.count_at_this_value;
                self.total_value_to_current_index +=
                    self.count_at_this_value *
                    self.histogram.highest_equivalent_value(self.current_value_at_index);
                self.fresh_sub_bucket = false;
            }

            if self.reached_iteration_level() {
                let value_iterated_to = self.get_value_iterated_to();
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

                println!("before return: {}", self);
                return &self.current_iteration_value;
            }

            self.increment_sub_bucket();
        }
        panic!("Histogram may have overflowed!")
    }

    fn increment_sub_bucket(&mut self) {
        self.fresh_sub_bucket = true;
        self.current_index += 1;
        self.current_value_at_index = self.histogram.value_from_index(self.current_index);
        self.next_value_at_index = self.histogram.value_from_index(self.current_index + 1);
    }

    fn increment_iteration_level(&mut self) {
        self.visited_index = self.current_index;
    }

    fn get_value_iterated_to(&mut self) -> i64 {
        self.histogram.highest_equivalent_value(self.current_value_at_index)
    }

    fn get_percentile_iterated_to(&mut self) -> f64 {
        (100.0f64 * self.total_count_to_current_index as f64) / self.array_total_count as f64
    }

    fn reached_iteration_level(&mut self) -> bool {
        let current_count = self.histogram.get_count_at_index(self.current_index);
        (current_count != 0) && (self.visited_index != self.current_index)
    }

    fn exhausted_sub_buckets(&mut self) -> bool {
        self.current_index >= self.histogram.get_counts_array_length()
    }

    fn reset_iterator(&mut self, total_count: i64, unit_magnitude: i32) {
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
}
