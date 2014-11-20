#![crate_name="dttpd"]
#![crate_type="bin"]

//local imports
extern crate dttp;

//local uses
//use dttp::mote::Auth;
use dttp::hub::Hub;

fn main(){
	let hub = Hub::new();
	hub.say_hi()}
