//library uses

//local uses
use auth::*;
use mote::*;

pub struct Hub {
	pub motes: Vec<Mote>,
	pub auth: Auth,
}
impl Hub {
	pub fn new() -> Hub {
		Hub{
			motes: Vec::new(),
			auth: Auth::new()}}

	pub fn say_hi( &self){
		println!("dttp daemon says hi :)");}
}
