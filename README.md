# rustogram

[![Build status](https://travis-ci.org/epickrram/rustogram.svg?branch=master)](https://travis-ci.org/epickrram/rustogram)

A Rust port of [HdrHistogram](https://github.com/HdrHistogram/HdrHistogram).


# usage

	extern crate rustogram;
	#[test]
	fn it_works() {
    	let mut h = rustogram::histogram::new_histogram(10000, 3);
    	h.record_value(42);
    	println!("Total recorded samples: {}", h.get_total_count());
	}
	