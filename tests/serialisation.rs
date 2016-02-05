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

#[test]
fn test_i32_encoding_and_decoding() {
	assert_i32_encoding_and_decoding(9834795);
}

#[test]
fn test_i32_encoding_and_decoding_limits() {
	assert_i32_encoding_and_decoding(std::i32::MAX);
	assert_i32_encoding_and_decoding(std::i32::MIN);
}

#[test]
fn test_i64_encoding_and_decoding() {
	assert_i64_encoding_and_decoding(9834795);
}

#[test]
fn test_i64_encoding_and_decoding_limits() {
	assert_i64_encoding_and_decoding(std::i64::MAX);
	assert_i64_encoding_and_decoding(std::i64::MIN);
}

#[ignore]
#[test]
fn test_deserialise() {
    let mut histogram = new_histogram_lower_bound(20000000, 100000000, 5);
    histogram.record_value(100000000);
    histogram.record_value(20000000);
    histogram.record_value(30000000);
    
    let mut target_buffer: Vec<u8> = Vec::new();
    histogram.serialise(&mut target_buffer);
    
    
    println!("serialised");
    print_byte_vec(&target_buffer);

	let byte_array = SERIALISED_FORM.from_base64().unwrap();
	
	println!("java serialised");
	print_byte_vec(&byte_array);
	
	let deserialised_histogram = deserialise_histogram(&byte_array, 0).unwrap();
	
	assert_eq!(histogram.get_total_count(), deserialised_histogram.get_total_count());
}

fn print_byte_vec(buffer: &Vec<u8>) {
	for b in buffer {
		print!("{}{} ", char_for_nibble(b / 16), char_for_nibble(b % 16));
	}
	println!("");
}

fn char_for_nibble(input: u8) -> &'static str {
	match input {
		0 => "0",
		1 => "1",
		2 => "2",
		3 => "3",
		
		4 => "4",
		5 => "5",
		6 => "6",
		7 => "7",
		
		8 => "8",
		9 => "9",
		10 => "10",
		11 => "11",
		
		12 => "12",
		13 => "13",
		14 => "14",
		15 => "15",
		
		_ => panic!("bad byte value!"),
	}
}

fn assert_i64_encoding_and_decoding(value: i64) {
	let mut buffer : Vec<u8> = Vec::new();
	put_i64(value, &mut buffer);
	
	let decoded = get_i64(&buffer, 0);
	
	assert_eq!(value, decoded);
}

fn assert_i32_encoding_and_decoding(value: i32) {
	let mut buffer : Vec<u8> = Vec::new();
	put_i32(value, &mut buffer);
	
	let decoded = get_i32(&buffer, 0);
	
	assert_eq!(value, decoded);
}
