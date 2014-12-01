// library uses
use std::fmt;
use std::collections::TreeMap;
use std::rand::Rng;

use serialize::base64;
use serialize::base64::*;
use serialize::json;

// local uses
use auth::*;
use dt::*;
use key::*;

/// class that defines the types of data carried by a mote
pub enum Class {
	// text classes
	Plain,
	Markdown,
	//  text data classes
	Json,
	// binary classes
	Raw,
	//  image classes
	Png,
	//  video classes
	Mp4,
}
impl fmt::Show for Class {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter,
			"{}",
			match *self {
				Plain => "plain",
				Markdown => "markdown",
				Json => "json",
				Raw => "raw",
				Png => "png",
				Mp4 => "mp4"})
	}
}

/// a unit of signed communication
pub struct Mote {
	// a string describing the data
	pub meta: String,
	// the type of data
	pub class: Class,
	// the party signing the mote
	pub auth: Auth,
	// the release date of the mote
	pub datetime: Datetime,
	// pregen'd salt
	pub salt: u64,
	// the data field
	pub data: Vec<u8>,
	// attached signature
	pub sig: Vec<u8>,
}
impl Mote {
	pub fn null() -> Mote {
		Mote {
			meta: String::new(),
			class: Raw,
			auth: Auth::null(),
			datetime: Datetime::null(),
			salt: 0x00000000,
			data: Vec::new(),
			sig: Vec::new(),}}
	pub fn new_test() -> Mote {
		Mote {
			meta: "test test :)".to_string(),
			class: Markdown,
			auth: Auth::null(),
			datetime: Datetime::new( 1964, 256, 43200_000),
			salt: 0x0ab1cf28,
			data: "test test yo yo bro".as_bytes().to_vec(),
			sig: vec!( 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00),}}

	pub fn salt< R: Rng >( &mut self, rng: &mut R){
		self.salt = rng.next_u64();}

	pub fn sign< Key: SecretKey >( &mut self, auth: &Auth, key: &Key){
		//generate plainbytes to sign
		let mut plain : Vec<u8> = Vec::new();
		//push meta bytes
		plain.push_all( self.meta.as_slice().as_bytes());
		//push datetime bytes
		plain.push_all( self.datetime.to_bytes().as_slice());
		//push data bytes
		plain.push_all( self.data.as_slice());
		//push salt
		for &offset in [ 56, 48, 40, 32, 24, 16, 08, 00].iter() {
			plain.push( ( self.salt >> offset) as u8);}
		//set signature fields
		self.auth = ( *auth).clone();
		self.sig = key.sign( plain.as_slice());}
}

impl fmt::Show for Mote {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let b64_config = base64::Config {
			char_set: Standard,
			pad: true,
			line_length: None };
		write!( formatter,
			"{}, {}, {}, {}, {:08x}, {:s}, {:s}",
			self.meta, self.class, self.auth,
			self.datetime, self.salt,
			self.data.as_slice().to_base64( b64_config),
			self.sig.as_slice().to_base64( b64_config),)}
}

impl json::ToJson for Mote {
	fn to_json( &self) -> json::Json {
		let mut mote_map = TreeMap::new();
		mote_map.insert(
			"meta".to_string(), self.meta.to_string());
		mote_map.insert(
			"class".to_string(), self.class.to_string());
		mote_map.insert(
			"auth".to_string(), self.auth.to_string());
		mote_map.insert(
			"datetime".to_string(), self.datetime.to_string());
		mote_map.insert(
			"salt".to_string(), self.salt.to_string());
		mote_map.insert(
			"data".to_string(), self.data.to_string());
		mote_map.insert(
			"sig".to_string(), self.sig.to_string());
		return mote_map.to_json();}
}

