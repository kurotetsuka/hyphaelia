// library uses
use std::num;
use std::fmt;
use time::now_utc;

// local uses

/// class that defines a date
#[deriving( Copy, Hash)]
pub struct Datetime {
	pub year: u16,
	pub day: u16,
	pub milli: u32,
}
impl Datetime {
	pub fn null() -> Datetime {
		Datetime {
			year: 0,
			day: 0,
			milli: 0}}

	pub fn new( year: u16, day: u16, milli: u32) -> Datetime {
		Datetime {
			year: year,
			day: day,
			milli: milli}}

	pub fn now() -> Datetime {
		// get time
		let now = now_utc();
		// get millis
		let millis =
			24 * now.tm_hour.to_u32().unwrap() +
			60 * now.tm_min.to_u32().unwrap() +
			1000 * now.tm_sec.to_u32().unwrap() +
			now.tm_nsec.to_u32().unwrap();
		// return
		Datetime {
			year: ( now.tm_year + 1900).to_u16().unwrap(),
			day: now.tm_yday.to_u16().unwrap(),
			milli: millis,}}

	pub fn from_str( string : &str) -> Option<Datetime> {
		let mut split = string.split( '.');

		// parse year
		let year_str = split.next();
		if year_str.is_none() { return None;}
		let year_str = year_str.unwrap();
		let year : Option<u16> =
			num::from_str_radix( year_str, 16);
		if year.is_none() { return None;}
		let year = year.unwrap();

		// parse day
		let day_str = split.next();
		if day_str.is_none() { return None;}
		let day_str = day_str.unwrap();
		let day : Option<u16> =
			num::from_str_radix( day_str, 16);
		if day.is_none() { return None;}
		let day = day.unwrap();

		// parse milli
		let milli_str = split.next();
		if milli_str.is_none() { return None;}
		let milli_str = milli_str.unwrap();
		let milli : Option<u32> =
			num::from_str_radix( milli_str, 16);
		if milli.is_none() { return None;}
		let milli = milli.unwrap();

		Some( Datetime::new( year, day, milli))}

	pub fn to_bytes( &self) -> Vec<u8> {
		let mut result = Vec::new();
		result.push( ( self.year >> 8) as u8);
		result.push( ( self.year >> 0) as u8);
		result.push( ( self.day >> 8) as u8);
		result.push( ( self.day >> 0) as u8);
		result.push( ( self.milli >> 24) as u8);
		result.push( ( self.milli >> 16) as u8);
		result.push( ( self.milli >> 08) as u8);
		result.push( ( self.milli >> 00) as u8);
		return result;}
}

impl fmt::Show for Datetime {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "{:03x}.{:03x}.{:07x}",
			self.year, self.day, self.milli)}
}
