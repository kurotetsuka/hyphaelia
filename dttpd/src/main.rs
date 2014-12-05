#![crate_name="dttpd"]
#![crate_type="bin"]

// rustc feature enables
#![feature(globs)]

// library imports
extern crate dttp;
extern crate serialize;

// library uses
use std::hash::hash;
//use serialize::json;
use serialize::json::ToJson;

// dttp lib uses
use dttp::Auth;
use dttp::Datetime;
use dttp::Mote;
use dttp::Hub;
use dttp::key;
use dttp::protocol::*;

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
	//println!( "auth: {}", auth);

	let keypair = key::keygen_fake( &mut rng);
	let ( sec_key, _pub_key ) = keypair;

	let mut mote = Mote::new_text(
		"test test :)".to_string(),
		dttp::mote::Markdown,
		Datetime::new( 1964, 256, 43200_000),
		"test test yo yo bro".to_string());
	mote.salt( &mut rng);
	mote.sign( &auth, &sec_key);
	//println!( "mote: {}", mote);
	//println!( "mote hash: {:x}", hash( &mote));

	let mote_hash = hash( &mote);
	let mote_msg = mote.to_msg();
	let mote_json = mote_msg.to_json();

	let cmd = Hello;
	println!( "cmd: {} : {}", cmd,
		Command::from_str( cmd.to_string().as_slice()));
	let cmd = OthersReq;
	println!( "cmd: {} : {}", cmd,
		Command::from_str( cmd.to_string().as_slice()));
	let cmd = HaveDec( mote_hash);
	println!( "cmd: {} : {}", cmd,
		Command::from_str( cmd.to_string().as_slice()));
	let cmd = HaveReq( mote_hash);
	println!( "cmd: {} : {}", cmd,
		Command::from_str( cmd.to_string().as_slice()));
	let cmd = Get( mote_hash);
	println!( "cmd: {} : {}", cmd,
		Command::from_str( cmd.to_string().as_slice()));
	let cmd = WantReq( mote_hash);
	println!( "cmd: {} : {}", cmd,
		Command::from_str( cmd.to_string().as_slice()));
	let cmd = Take( mote_json.clone());
	println!( "cmd: {} : {}", cmd,
		Command::from_str( cmd.to_string().as_slice()));

	let res = Okay;
	println!( "res: {} : {}", res,
		Response::from_str( res.to_string().as_slice()));
	let res = OkayResult( mote_json);
	println!( "res: {} : {}", res,
		Response::from_str( res.to_string().as_slice()));
	let res = Deny;
	println!( "res: {} : {}", res,
		Response::from_str( res.to_string().as_slice()));
	let res = Error;
	println!( "res: {} : {}", res,
		Response::from_str( res.to_string().as_slice()));
	let res = ErrorMsg( "asdf".to_string());
	println!( "res: {} : {}", res,
		Response::from_str( res.to_string().as_slice()));
}
