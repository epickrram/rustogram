extern crate rustogram;
extern crate rustc_serialize;
static SERIALISED_FORM: &'static str = "HISTEwAAAAQAAAAAAAAABQAAAAABMS0AAAAAAAX14QA/8AAAAAAAAAAEBQI=";

use rustogram::histogram::*;
use rustc_serialize::base64::*;

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