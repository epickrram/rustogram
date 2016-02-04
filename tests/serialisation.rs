extern crate rustogram;
extern crate rustc_serialize;
static SERIALISED_FORM: &'static str = "HISTEwAAAAQAAAAAAAAABQAAAAABMS0AAAAAAAX14QA/8AAAAAAAAAAEBQI=";

use rustogram::encoding::*;
use rustogram::histogram::*;
use rustc_serialize::base64::*;

#[test]
fn test_zig_zag_encoding_for_single_value() {
	let mut buffer : Vec<u8> = Vec::new();
	
	let value: i64 = 982374923847239;
	encode(value, &mut buffer);
	
	let (decoded_value, bytes_read) = decode(&buffer, 0);
	assert_eq!(value, decoded_value);
	assert_eq!(8, bytes_read);
}

#[test]
fn test_zig_zag_encoding_for_multiple_values() {
	let mut buffer : Vec<u8> = Vec::new();
	
	let value0: i64 = 47;
	encode(value0, &mut buffer);
	
	let value1: i64 = 23746;
	encode(value1, &mut buffer);
	
	let (decoded_value0, bytes_read0) = decode(&buffer, 0);
	let (decoded_value1, bytes_read1) = decode(&buffer, bytes_read0);
	
	assert_eq!(value0, decoded_value0);
	assert_eq!(1, bytes_read0);
	
	assert_eq!(value1, decoded_value1);
	assert_eq!(3, bytes_read1);
}

#[test]
fn test_zig_zag_encoding_for_limits() {
	let mut buffer : Vec<u8> = Vec::new();
	
	let value0: i64 = std::i64::MAX;
	encode(value0, &mut buffer);
	
	let value1: i64 = std::i64::MIN;
	encode(value1, &mut buffer);
	
	let (decoded_value0, bytes_read0) = decode(&buffer, 0);
	let (decoded_value1, bytes_read1) = decode(&buffer, bytes_read0);
	
	assert_eq!(value0, decoded_value0);
	assert_eq!(9, bytes_read0);
	
	assert_eq!(value1, decoded_value1);
	assert_eq!(9, bytes_read1);
	
}

#[ignore]
#[test]
fn test_deserialise() {
    let mut histogram = new_histogram_lower_bound(20000000, 100000000, 5);
    histogram.record_value(100000000);
    histogram.record_value(20000000);
    histogram.record_value(30000000);

	let byte_array = SERIALISED_FORM.from_base64().unwrap();
	
	let deserialised_histogram = deserialise_histogram(&byte_array).unwrap();
	
	
}