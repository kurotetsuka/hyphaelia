// library uses
use std::io::BufferedReader;
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::TcpStream;
use std::io::timer::sleep;
use std::sync::{ Arc, Mutex};
use std::time::duration::Duration;

use serialize::json;

// local uses
//use auth::*;
use mote::*;
//use hub::mode::Mode;
use hub::remote::RemoteHub;
use protocol::*;

// modules
pub mod mode;
pub mod remote;

pub struct Hub {
	// this hub's address
	pub port: u16,
	// this hub's stored motes
	pub motedb: Arc<Mutex<Vec<Mote>>>,
	// this hub's auth-key database
	//pub authdb: Vec<Auth>,
	// this hub's auth database
	pub remotedb: Arc<Mutex<Vec<RemoteHub>>>,
	// this hub's authorizing party
	//pub auth: Auth,
	// this hub's authorizing key
	//pub sec_key: AuthSecKey,
	// this hub's verifying key
	//pub pub_key: AuthPubKey,
	// this hub's operation modes
	//pub modes: HashMap<Mode, bool>,
}
impl Hub {
	pub fn new( port: u16) -> Hub {
		Hub {
			port: port,
			motedb: Arc::new( Mutex::new( Vec::new())),
			remotedb: Arc::new( Mutex::new( Vec::new())),
			//modes: HashMap::new(),
		}}

	pub fn say_hi( &self){
		println!("dttp daemon says hi :)");}

	pub fn add_remote( &mut self, addr: SocketAddr){
		let remote = RemoteHub::new( addr);
		self.remotedb.lock().push( remote);}

	pub fn add_mote( &mut self, mote: Mote){
		self.motedb.lock().push( mote);}

	/*pub fn mode_get( &self, mode: &Mode) -> bool {
		match self.modes.find( mode) {
			Some( &true) => true,
			_ => false,}}
	pub fn mode_set( &mut self, mode: Mode, enabled: bool) {
		self.modes.insert( mode, enabled);}*/

	pub fn launch( &self){
		self.spawn_daemon();
		self.spawn_bootstrap();
		self.spawn_push();}

	pub fn spawn_daemon( &self){}

	pub fn spawn_bootstrap( &self){
		let remotedb_mutex = self.remotedb.clone();
		spawn( proc(){
			let others_req_msg = OthersReq.to_string();
			loop {
				// copy addresses from current remotedb
				let remotedb = remotedb_mutex.lock();
				let mut remotes_addr : Vec<SocketAddr> = Vec::new();
				for ref remote in remotedb.deref().iter() {
					remotes_addr.push( remote.addr.clone());}
				drop( remotedb);

				// get new 
				let mut new_remotes : Vec<SocketAddr> = Vec::new();
				for &addr in remotes_addr.iter() {
					// connect to remote
					println!( "attempting to bootstrap against: {}", addr);
					let remote_stream =
						TcpStream::connect_timeout(
							addr.clone(), Duration::seconds( 20));
					if remote_stream.is_err() {
						println!( "failed to connect to {}", addr);
						continue;}
					let mut remote_stream = remote_stream.unwrap();

					// send request
					remote_stream.write_line( others_req_msg.as_slice()).ok();
					remote_stream.close_write().ok();

					// parse response
					let mut reader = BufferedReader::new( remote_stream.clone());
					let line = reader.read_line().ok();
					remote_stream.close_read().ok();
					if line.is_none() {
						println!( "failed to read response from {}", addr);
						continue;}
					let line = line.unwrap();
					let line_trimmed = line.as_slice().trim();
					let response = Response::from_str( line_trimmed);
					if response.is_none() {
						println!( "failed to parse response from {}: {}",
							addr, line_trimmed);
						continue;}
					let response = response.unwrap();

					// handle response
					match response {
						OkayResult( json::List( list)) => {
							for ref entry in list.iter() {
								match *entry {
									&json::String( ref string) => {
										let new_addr : Option<SocketAddr> =
											from_str( string.as_slice());
										if new_addr.is_some(){
											new_remotes.push( new_addr.unwrap());}}
									_ => ()}}}
						Deny => {
							println!( "deny response from {}", addr);}
						bad => {
							println!( "bad response from {}: {}", addr, bad);}}

					// move on
					continue;}

				let mut remotedb = remotedb_mutex.lock();
				for &new_addr in new_remotes.iter() {
					remotedb.push( RemoteHub::new( new_addr.clone()));}
				drop( remotedb);
				sleep( Duration::seconds( 30));}});

		println!( "bootstrap proc spawned.");}

	pub fn spawn_push( &self){}
}

