#![crate_name="dttpd"]
#![crate_type="bin"]

//local imports
extern crate dttp;

//local uses
use dttp::mote::Auth;

fn main(){
	let auth = Auth::new();
	println!("dttp daemon says hi :)");}
