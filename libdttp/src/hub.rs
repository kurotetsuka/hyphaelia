//library uses

//local uses
use auth::*;
use mote::*;

pub struct Hub {
	// this hub's stored motes
	pub motes: Vec<Mote>,
	// this hub's authorizing party
	pub auth: Auth,
}
impl Hub {
	pub fn new() -> Hub {
		Hub{
			motes: Vec::new(),
			auth: Auth::null()}}

	pub fn say_hi( &self){
		println!("dttp daemon says hi :)");}
}
