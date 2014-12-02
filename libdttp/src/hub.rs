//library uses
use std::io::net::ip::SocketAddr;

//local uses
use auth::*;
use mote::*;

pub struct RemoteHub {
	pub addr: SocketAddr,
	pub motedb: Vec<u64>,
	pub authdb: Vec<u64>,
}

pub struct Hub {
	// this hub's stored motes
	pub motedb: Vec<Mote>,
	// this hub's auth-key database
	pub authdb: Vec<Auth>,
	// this hub's auth database
	pub remotedb: Vec<RemoteHub>,
	// this hub's authorizing party
	//pub auth: Auth,
	// this hub's authorizing key
	//pub sec_key: AuthSecKey,
	// this hub's verifying key
	//pub pub_key: AuthPubKey,
}
impl Hub {
	pub fn new() -> Hub {
		Hub {
			motedb: Vec::new(),
			authdb: Vec::new(),
			remotedb: Vec::new(),}}

	pub fn say_hi( &self){
		println!("dttp daemon says hi :)");}

	pub fn add( &mut self, mote: Mote){
		self.motedb.push( mote);}
}
