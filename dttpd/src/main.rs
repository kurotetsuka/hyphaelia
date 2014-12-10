#![crate_name="dttpd"]
#![crate_type="bin"]

// rustc feature enables
#![feature(globs)]

// library imports
extern crate dttp;
extern crate serialize;

// library uses
use std::hash;
use std::io::net::ip::{ SocketAddr, ToSocketAddr};
use std::io::BufferedReader;
use std::path::Path;
use std::io::fs::File;
use std::num;
use std::os;

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
	let bs_list = load_bootstrap_list( "config/dev.bs");
	println!( "bs list: {}", bs_list);
	for &bs in bs_list.iter() {
		hub.add_remote( bs);}

	let mote = test_mote();
	let mote_hash = hash::hash( &mote);
	println!( "generated test mote: {:x} :: {}", mote_hash, mote);
	//println!( "mote json: {}", 
	//	serialize::json::encode( &mote.to_msg()));
	hub.add_mote( mote);

	// launch_hub
	hub.launch();}

fn test_mote() -> Mote {
	// get rng
	let rng = std::rand::OsRng::new();
	let mut rng = rng.unwrap();

	// create new auth
	let auth = Auth::new(
		Some( "kurotetsuka".to_string()),
		None,
		Some( "kurotetsuka@gmail.com".to_string()),
		Some( 0x0a1a20c0));
	//println!( "auth: {}", auth);

	// create fake keypair
	let keypair = key::keygen_fake( &mut rng);
	let ( sec_key, _pub_key ) = keypair;

	// create mote
	let mut mote = Mote::new_text(
		"test test :)".to_string(),
		dttp::mote::Class::Markdown,
		Datetime::new( 1964, 256, 43200_000),
		"test test yo yo bro".to_string());
	mote.salt( &mut rng);
	mote.sign( &auth, &sec_key);

	return mote;}

fn load_bootstrap_list( bs_filename: &str) -> Vec<SocketAddr> {
	let mut bs_list : Vec<SocketAddr> = Vec::new();

	//for each line in file
	let lines = read_lines( bs_filename);
	for line in lines.iter() {
		let line = line.as_slice().trim();
		// ignore comments
		if line.char_at( 0) == '#' { continue;}
		// parse address
		let addr : Option<SocketAddr> = line.to_socket_addr().ok();
		if addr.is_none() { continue;}
		// push address
		bs_list.push( addr.unwrap());}

	// done
	return bs_list;}

fn read_lines( filename : &str) -> Vec<String> {
	// open the file
	let path = Path::new( filename);
	let file = File::open( &path);
	let mut reader = BufferedReader::new( file);
	// read the file
	let mut result : Vec<String> = Vec::new();
	for line_result in reader.lines() {
		if line_result.is_err() { continue;}
		let line = line_result.unwrap();
		// strip the newline chars
		let line_stripped = line.as_slice().trim_chars('\n');
		result.push( String::from_str( line_stripped));}
	return result;}
