// library uses
use std::fmt;

// local uses

pub enum DataClass {
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
impl fmt::Show for DataClass {
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

pub struct Auth {
	pub user: Option<String>,
	pub comment: Option<String>,
	pub email: Option<String>,
	pub id: Option<[u8, ..4]>,
}
impl Auth {
	pub fn new() -> Auth {
		Auth {
			user: Some( "kurotetsuka".to_string()),
			comment: None,
			email: Some( "kurotetsuka@gmail.com".to_string()),
			id: Some( [ 0x0a, 0x1a, 0x20, 0xc0])}}
}
impl fmt::Show for Auth {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "")
	}
}

pub struct Datetime {
	pub year: u64,
	pub day: u64,
	pub milli: u64,
}
impl Datetime {
	pub fn new( year: u64, day: u64, milli: u64) -> Datetime {
		Datetime {
			year: year,
			day: day,
			milli: milli}}
}
impl fmt::Show for Datetime {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "")
	}
}

pub enum Data {
	Text( String),
	Binary( Vec<u8>),
}
impl fmt::Show for Data {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "")
	}
}

// a minimal unit of signed conversation
pub struct Mote {
	pub meta: String,
	// the type of data
	pub class: DataClass,
	// the party signing the mote
	pub auth: Auth,
	// the release date of the mote
	pub datetime: Datetime,
	// pregen'd salt
	pub salt: u64,
	// attached signature
	pub sig: [u8, ..8],
	// the data field
	pub data: Data,
}
impl Mote {
	pub fn new() -> Mote {
		Mote {
			meta: "test test :)".to_string(),
			class: Markdown,
			auth: Auth::new(),
			datetime: Datetime::new( 1964, 256, 43200_000),
			salt: 0x0ab1cf28,
			sig: [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			data: Text( "test test yo yo bro".to_string())}}
}
impl fmt::Show for Mote {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter,
			"{}, {}, {}, {}, {}, {}, {}",
			self.meta, self.class, self.auth, self.datetime,
			self.salt, "sig", self.data)
	}
}

// a mote, prepared for serialization
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
