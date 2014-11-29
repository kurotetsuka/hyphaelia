//library uses

//local uses
use auth::*;
use key::*;
use mote::*;

pub struct Hub {
	// this hub's stored motes
	pub motes: Vec<Mote>,
	// this hub's authorizing party
	pub auth: Auth,
	// this hub's authorizing key
	//pub sec_key: AuthSecKey,
	// this hub's verifying key
	//pub pub_key: AuthPubKey,
}
impl Hub {
	pub fn new() -> Hub {
		Hub {
			motes: Vec::new(),
			auth: Auth::null()}}

	pub fn say_hi( &self){
		println!("dttp daemon says hi :)");}

	pub fn add( &mut self, mote: Mote){
		self.motes.push( mote);}
}
