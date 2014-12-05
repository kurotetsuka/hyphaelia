// library uses
use std::collections::HashMap;

// local uses
//use auth::*;
use mote::*;
use hub::mode::Mode;
use hub::remote::RemoteHub;
//use protocol::*;

// modules
pub mod mode;
pub mod remote;

pub struct Hub {
	// this hub's stored motes
	pub motedb: Vec<Mote>,
	// this hub's auth-key database
	//pub authdb: Vec<Auth>,
	// this hub's auth database
	pub remotedb: Vec<RemoteHub>,
	// this hub's authorizing party
	//pub auth: Auth,
	// this hub's authorizing key
	//pub sec_key: AuthSecKey,
	// this hub's verifying key
	//pub pub_key: AuthPubKey,
	// this hub's operation modes
	pub modes: HashMap<Mode, bool>,
}
impl Hub {
	pub fn new() -> Hub {
		Hub {
			motedb: Vec::new(),
			remotedb: Vec::new(),
			modes: HashMap::new(),}}

	pub fn say_hi( &self){
		println!("dttp daemon says hi :)");}

	pub fn add( &mut self, mote: Mote){
		self.motedb.push( mote);}

	pub fn mode_get( &self, mode: &Mode) -> bool {
		match self.modes.find( mode) {
			Some( &true) => true,
			_ => false,}}

	pub fn mode_set( &mut self, mode: Mode, enabled: bool) {
		self.modes.insert( mode, enabled);}
}

pub fn launch( _hub: Hub){}
