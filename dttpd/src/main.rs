#![crate_name="dttpd"]
#![crate_type="bin"]

// library imports
extern crate dttp;
extern crate serialize;

//library uses
use std::hash::hash;
use serialize::json;

// dttp lib uses
use dttp::Auth;
use dttp::Datetime;
use dttp::Mote;
use dttp::Hub;
use dttp::key;

// entry function
fn main(){

	let hub = Hub::new();
	hub.say_hi();

	let rng = std::rand::OsRng::new();
	let mut rng = rng.unwrap();

	let auth = Auth::new(
		Some( "kurotetsuka".to_string()),
		None,
		Some( "kurotetsuka@gmail.com".to_string()),
		Some( 0x0a1a20c0));
	println!( "auth: {}", auth);

	let keypair = key::keygen_fake( &mut rng);
	let ( sec_key, _pub_key ) = keypair;

	let mut mote = Mote::new_text(
		"test test :)".to_string(),
		dttp::mote::Markdown,
		Datetime::new( 1964, 256, 43200_000),
		"test test yo yo bro".to_string());
	mote.salt( &mut rng);
	mote.sign( &auth, &sec_key);
	println!( "mote: {}", mote);
	println!( "mote hash: {:x}", hash( &mote));

	let mote_msg = mote.to_msg();
	let mote_json = json::encode( &mote_msg);
	println!( "mote json: {}", mote_json);

	let mote = Mote::from_str( mote_json.as_slice()).unwrap();
	println!( "mote decoded hash: {:x}", hash( &mote));
	println!("mode decoded: {}", mote);
}
