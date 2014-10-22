#![crate_name="dttp"]
#![crate_type="lib"]

enum Class {
	//text classes
	Plain,
	Markdown,
	Json,
	//binary classes
	Binary,
	Png,
}

struct Auth {
	user: Option( String),
	comment: Option( String),
	email: Option( String),
	id: [u8, ..4],
}

struct Datetime {
	year: u64,
	day: u64,
	milli: u64,
}

enum Data {
	Text( String),
	Binary( [u8]),
}

struct Mote {
	meta: Meta,
	data: Data,
	//the 
	class: Class,
	//the party signing the mote
	auth: String,
	//the release date of the mote
	datetime: Datetime,
	//pregen'd salt
	salt: u64,
	//attached signature
	sig: [u8],
}

/// a mote, prepared for transmittal
#[deriving( Decodable, Encodable)]
struct MoteMsg {
	dttpv: String,
	meta: String,
	data: String,
	class: String,
	auth: String,
	datetime: String,
	salt: String,
	sig: String,
}