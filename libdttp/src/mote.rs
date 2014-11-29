// library uses
use std::fmt;

use serialize::base64;
use serialize::base64::*;
use serialize::json;

// local uses
use auth::*;
use dt::*;

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
			auth: Auth::new_test(),
			datetime: Datetime::new( 1964, 256, 43200_000),
			salt: 0x0ab1cf28,
			data: "test test yo yo bro".as_bytes().to_vec(),
			sig: vec!( 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00),}}

	/*fn to_msg ( &self) -> MoteMsg {
		let b64_config = base64::Config {
			char_set: Standard,
			pad: true,
			line_length: None };
		MoteMsg {
			meta: self.meta.to_string(),
			class: self.class.to_string(),
			auth: self.auth.to_string(),
			datetime: self.datetime.to_string(),
			salt: self.salt.to_string(),
			data: self.data.as_slice().to_base64( b64_config),
			sig: self.sig.as_slice().to_base64( b64_config),}}*/
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
		
}

