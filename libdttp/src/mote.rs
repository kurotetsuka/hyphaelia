// library uses
use std::fmt;

use serialize::base64;
use serialize::base64::*;

// local uses
use auth::*;

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

/// class that defines release date of the mote
pub struct Datetime {
	pub year: u16,
	pub day: u16,
	pub milli: u32,
}
impl Datetime {
	pub fn new( year: u16, day: u16, milli: u32) -> Datetime {
		Datetime {
			year: year,
			day: day,
			milli: milli}}
}
impl fmt::Show for Datetime {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "{:03x}.{:03x}.{:07x}",
			self.year, self.day, self.milli)}
}

/// a unit of signed communication
pub struct Mote {
	pub meta: String,
	// the type of data
	pub class: Class,
	// the party signing the mote
	pub auth: Auth,
	// the release date of the mote
	pub datetime: Datetime,
	// pregen'd salt
	pub salt: u64,
	// attached signature
	pub sig: u64,
	// the data field
	pub data: Vec<u8>,
}
impl Mote {
	pub fn new_test() -> Mote {
		Mote {
			meta: "test test :)".to_string(),
			class: Markdown,
			auth: Auth::new(),
			datetime: Datetime::new( 1964, 256, 43200_000),
			salt: 0x0ab1cf28,
			sig: 0x0000000000000000,
			data: "test test yo yo bro".as_bytes().to_vec()}}
}
impl fmt::Show for Mote {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let b64_config = base64::Config {
			char_set: Standard,
			pad: true,
			line_length: None };
		write!( formatter,
			"{}, {}, {}, {}, {:08x}, {:08x}, {:s}",
			self.meta, self.class, self.auth,
			self.datetime, self.salt, self.sig,
			self.data.as_slice().to_base64( b64_config))
	}
}

/// a mote, prepared for serialization
#[deriving( Decodable, Encodable)]
pub struct MoteMsg {
	pub meta: String,
	// the type of data
	pub class: String,
	// the party signing the mote
	pub auth: String,
	// the release date of the mote
	pub datetime: String,
	// pregen'd salt
	pub salt: String,
	// attached signature
	pub sig: String,
	// the data field
	pub data: String,
}
impl fmt::Show for MoteMsg {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "")
	}
}
