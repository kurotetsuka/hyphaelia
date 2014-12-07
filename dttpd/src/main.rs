#![crate_name="dttpd"]
#![crate_type="bin"]

// rustc feature enables
#![feature(globs)]

// library imports
extern crate dttp;
extern crate serialize;

// library uses
use std::os;
use std::num;
use std::io::net::ip::SocketAddr;

// dttp lib uses
use dttp::Auth;
use dttp::Datetime;
use dttp::Mote;
use dttp::Hub;
use dttp::key;

// entry function
fn main(){
	let args = os::args();
	let mut port = 8960;
	if args.len() > 1 {
		let port_arg = args[ 1].as_slice();
		let port_arg : Option<u16> = 
			num::from_str_radix( port_arg, 10);
		if port_arg.is_some() {
			port = port_arg.unwrap()}}

	let mut hub = Hub::new( port);
	hub.say_hi();

	//add bootstrap remotes
	let bs_list = [
		"localhost:8960",
		"localhost:8961",
		"localhost:8962",
		"localhost:8963",
		"localhost:8964"];
	for &bs in bs_list.iter() {
		let bs : Option<SocketAddr> = from_str( bs);
		if bs.is_none() { continue;}
		hub.add_remote( bs.unwrap());}

	let mote = test_mote();
	//println!( "mote: {}", mote);
	//println!( "mote json: {}", 
	//	serialize::json::encode( &mote.to_msg()));
	hub.add_mote( mote);

	// launch_hub
	hub.launch();
}

fn test_mote() -> Mote {
	let rng = std::rand::OsRng::new();
	let mut rng = rng.unwrap();

	let auth = Auth::new(
		Some( "kurotetsuka".to_string()),
		None,
		Some( "kurotetsuka@gmail.com".to_string()),
		Some( 0x0a1a20c0));
	//println!( "auth: {}", auth);

	let keypair = key::keygen_fake( &mut rng);
	let ( sec_key, _pub_key ) = keypair;

	let mut mote = Mote::new_text(
		"test test :)".to_string(),
		dttp::mote::Class::Markdown,
		Datetime::new( 1964, 256, 43200_000),
		"test test yo yo bro".to_string());
	mote.salt( &mut rng);
	mote.sign( &auth, &sec_key);

	return mote;}