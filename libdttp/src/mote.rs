// library uses
use std::fmt;
use std::rand::Rng;

use serialize::base64;
use serialize::base64::*;

// local uses
use auth::*;
use dt::*;
use key::*;

/// class that defines the types of data carried by a mote
#[deriving( Hash)]
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
#[deriving( Hash)]
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
	pub fn new_bin(
			meta: String, class: Class,
			datetime: Datetime, data: Vec<u8>) -> Mote {
		Mote {
			meta: meta,
			class: class,
			auth: Auth::null(),
			datetime: datetime,
			salt: 0x00000000,
			data: data,
			sig: Vec::new(),}}
	pub fn new_text(
			meta: String, class: Class,
			datetime: Datetime, data: String) -> Mote {
		Mote {
			meta: meta,
			class: class,
			auth: Auth::null(),
			datetime: datetime,
			salt: 0x00000000,
			data: data.into_bytes(),
			sig: Vec::new(),}}

	pub fn set_meta( &mut self, meta: String){
		self.meta = meta;}

	pub fn set_data( &mut self, class: Class, data: Vec<u8>){
		self.class = class;
		self.data = data;}

	pub fn set_text( &mut self, class: Class, data: String){
		self.set_data( class, data.into_bytes());}

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

	pub fn to_msg( &self) -> MoteMsg {
		let b64_config = base64::Config {
			char_set: Standard,
			pad: true,
			line_length: None };
		MoteMsg {
			meta: self.meta.to_string(),
			class: self.class.to_string(),
			auth: self.auth.to_string(),
			datetime: self.datetime.to_string(),
			salt: format!( "{:08x}", self.salt),
			data: self.data.as_slice().to_base64( b64_config),
			sig: self.sig.as_slice().to_base64( b64_config),}}
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

/// a mote, prepared for transmittal
#[deriving( Hash)]
#[deriving( Encodable, Decodable)]
pub struct MoteMsg {
	// a string describing the data
	pub meta: String,
	// the type of data
	pub class: String,
	// the party signing the mote
	pub auth: String,
	// the release date of the mote
	pub datetime: String,
	// pregen'd salt
	pub salt: String,
	// the data field
	pub data: String,
	// attached signature
	pub sig: String,
}
