#![crate_name="dttpd"]
#![crate_type="bin"]

// library imports
extern crate dttp;

// local uses
use dttp::hub::Hub;

// entry function
fn main(){
	println!("test \
		test \
		test");

	let hub = Hub::new();
	hub.say_hi()}
