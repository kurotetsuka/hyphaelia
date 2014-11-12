#![crate_name="dttp"]
#![crate_type="lib"]

//standard library imports
extern crate serialize;
use serialize::{ Decodable, Encodable};

pub mod mote {
	enum DataClass {
		//text classes
		Plain,
		Ascii,
		Markdown,
		// text data classes
		Json,
		//binary classes
		Raw,
		// image classes
		Png,
	}

	struct Auth {
		user: Option<String>,
		comment: Option<String>,
		email: Option<String>,
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

	#[deriving( Decodable, Encodable)]
	struct Mote {
		meta: String,
		//the type of data
		class: DataClass,
		//the party signing the mote
		auth: Auth,
		//the release date of the mote
		datetime: Datetime,
		//pregen'd salt
		salt: u64,
		//attached signature
		sig: [u8, ..8],
		//the data field
		data: Data,
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
}
