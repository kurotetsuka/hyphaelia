
pub enum DataClass {
	//text classes
	Plain,
	Markdown,
	// text data classes
	Json,
	//binary classes
	Raw,
	// image classes
	Png,
	// video classes
	Mp4,
}

pub struct Auth {
	pub user: Option<String>,
	pub comment: Option<String>,
	pub email: Option<String>,
	pub id: Option<[u8, ..4]>,
}

pub struct Datetime {
	pub year: u64,
	pub day: u64,
	pub milli: u64,
}

pub enum Data {
	Text( String),
	Binary( [u8]),
}

pub struct Mote {
	pub meta: String,
	//the type of data
	pub class: DataClass,
	//the party signing the mote
	pub auth: Auth,
	//the release date of the mote
	pub datetime: Datetime,
	//pregen'd salt
	pub salt: u64,
	//attached signature
	pub sig: [u8, ..8],
	//the data field
	pub data: Data,
}
