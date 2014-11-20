
//local uses
use mote::Mote;

pub struct Hub {
	pub mote: Mote,
}
impl Hub {
	pub fn new() -> Hub {
		Hub{ mote: Mote::new()}}

	pub fn say_hi( &self){
		println!("dttp daemon says hi :)");}
}