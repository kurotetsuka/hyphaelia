// library uses
use std::num;
use std::fmt;
use serialize::json;
use serialize::json::Json;
use regex::Regex;

// local uses

/*
pub enum MoteSpec {
	ShortHash( u8),
	LongHash( u64),
	Meta( String),
	Auth( Auth),
}*/

pub enum Command {
	Hello,
	OthersReq,
	HaveDec( u64),
	HaveReq( u64),
	Get( u64),
	WantReq( u64),
	Take( Json),
}
impl Command {
	pub fn from_str( string: &str) -> Option<Command> {
		// match hello command
		let cmd = "hi?";
		if string.equiv( &cmd) {
			return Some( Hello);}

		// match others request
		let cmd = "others?";
		if string.equiv( &cmd) {
			return Some( OthersReq);}

		// match have command
		let cmd = "have:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"have:([:xdigit:]{16}).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let hash_str = cap.at( 1);
			let hash : Option<u64> = 
				num::from_str_radix( hash_str, 16);
			if hash.is_none() { return None;}
			let hash = hash.unwrap();
			// return
			return Some( HaveDec( hash));}

		// match have request
		let cmd = "have?:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"have\?:([:xdigit:]{16}).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let hash_str = cap.at( 1);
			let hash : Option<u64> = 
				num::from_str_radix( hash_str, 16);
			if hash.is_none() { return None;}
			let hash = hash.unwrap();
			// return
			return Some( HaveReq( hash));}

		// match get request
		let cmd = "get?:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"get\?:([:xdigit:]{16}).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let hash_str = cap.at( 1);
			let hash : Option<u64> = 
				num::from_str_radix( hash_str, 16);
			if hash.is_none() { return None;}
			let hash = hash.unwrap();
			// return
			return Some( Get( hash));}

		// match want request
		let cmd = "want?:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"want\?:([:xdigit:]{16}).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let hash_str = cap.at( 1);
			let hash : Option<u64> = 
				num::from_str_radix( hash_str, 16);
			if hash.is_none() { return None;}
			let hash = hash.unwrap();
			// return
			return Some( WantReq( hash));}

		// match take command
		let cmd = "take:";
		if string.starts_with( cmd) {
			let regex = Regex::new( r"take:(.+).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse hash
			let json_str = cap.at( 1);
			let json : Option<Json> = 
				json::from_str( json_str).ok();
			if json.is_none() { return None;}
			let json = json.unwrap();
			// return
			return Some( Take( json));}

		// fallback
		return None;}
}
impl fmt::Show for Command {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Hello =>
				write!( formatter, "hi?"),
			&OthersReq =>
				write!( formatter, "others?"),
			&HaveDec( ref hash) =>
				write!( formatter, "have:{:016x}.", *hash),
			&HaveReq( ref hash) =>
				write!( formatter, "have?:{:016x}.", *hash),
			&Get( ref hash) =>
				write!( formatter, "get?:{:016x}.", *hash),
			&WantReq( ref hash) =>
				write!( formatter, "want?:{:016x}.", *hash),
			&Take( ref data) =>
				write!( formatter, "take:{}.", json::encode( data)),}}
}


pub enum Response {
	Okay,
	OkayResult( Json),
	Deny,
	Error,
	ErrorMsg( String),
}
impl Response {
	pub fn from_str( string: &str) -> Option<Response> {
		// match okay response
		let res = "ok.";
		if string.equiv( &res) {
			return Some( Okay);}

		// match deny response
		let res = "no.";
		if string.equiv( &res) {
			return Some( Deny);}

		// match error response
		let res = "err.";
		if string.equiv( &res) {
			return Some( Error);}

		// match okay result response
		let res = "ok:";
		if string.starts_with( res) {
			let regex = Regex::new( r"ok:(.+).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse json
			let json_str = cap.at( 1);
			let json : Option<Json> = 
				json::from_str( json_str).ok();
			if json.is_none() { return None;}
			let json = json.unwrap();
			// return
			return Some( OkayResult( json));}

		// match error message response
		let res = "err:";
		if string.starts_with( res) {
			let regex = Regex::new( r"err:(.+).").unwrap();
			let cap = regex.captures( string);
			if cap.is_none() { return None;}
			let cap = cap.unwrap();

			// parse message
			let message = cap.at( 1);
			return Some( ErrorMsg( message.to_string()));}

		// fallback
		return None;}
}
impl fmt::Show for Response {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Okay =>
				write!( formatter, "ok."),
			&OkayResult( ref data) =>
				write!( formatter, "ok:{}.", json::encode( data)),
			&Deny =>
				write!( formatter, "no."),
			&Error =>
				write!( formatter, "err."),
			&ErrorMsg( ref message) =>
				write!( formatter, "err:{}.", message),}}
}
