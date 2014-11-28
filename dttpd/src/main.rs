#![crate_name="dttpd"]
#![crate_type="bin"]

// library imports
extern crate dttp;

// local uses
use dttp::Mote;
use dttp::Hub;

// entry function
fn main(){
	println!("test \
		test \
		test");

	let mote = Mote::new_test();
	println!( "mote: {}", mote);

	let hub = Hub::new();
	hub.say_hi()}
