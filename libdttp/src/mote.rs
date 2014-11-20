

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
impl Auth {
	pub fn new() -> Auth {
		Auth {
			user: Some( "kurotetsuka".to_string()),
			comment: None,
			email: Some( "kurotetsuka@gmail.com".to_string()),
			id: Some( [ 0x0a, 0x1a, 0x20, 0xc0])}}
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

pub enum Data {
	Text( String),
	Binary( Vec<u8>),
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
	pub data: Data,
	pub sig: [u8, ..8],
	//the data field
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
