// library uses
use std::hash;
use std::io::{ Acceptor, BufferedReader, Listener};
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::{ TcpListener, TcpStream};
use std::io::timer::sleep;
use std::sync::{ Arc, Mutex};
use std::time::duration::Duration;

use serialize::Decodable;
use serialize::json;
use serialize::json::{ Json, ToJson};

// local uses
//use auth::*;
use mote::*;
//use hub::mode::Mode;
use hub::remote::RemoteHub;
use protocol::*;
use protocol::Command::*;
use protocol::Response::*;

// modules
pub mod mode;
pub mod remote;

// constants
static PUSH_PAUSE_SECONDS: i64 = 15;
static BOOTSTRAP_PAUSE_SECONDS: i64 = 15;

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
		self.spawn_server();
		self.spawn_bootstrap();
		self.spawn_push();}

	// thread launching functions

	pub fn spawn_server( &self){
		let port = self.port;
		let motedb_arc = self.motedb.clone();
		let remotedb_arc = self.remotedb.clone();
		spawn( proc(){
			Hub::server( port, motedb_arc, remotedb_arc);});
		println!( "server proc spawned.");}

	pub fn spawn_bootstrap( &self){
		let remotedb_arc = self.remotedb.clone();
		spawn( proc(){
			Hub::bootstrap( remotedb_arc);});
		println!( "bootstrap proc spawned.");}

	pub fn spawn_push( &self){
		let motedb_arc = self.motedb.clone();
		let remotedb_arc = self.remotedb.clone();
		spawn( proc(){
			Hub::push( motedb_arc, remotedb_arc);});
		println!( "push proc spawned.");}

	// thread functions

	fn server( port: u16, motedb_arc: Arc<Mutex<Vec<Mote>>>,
			remotedb_arc: Arc<Mutex<Vec<RemoteHub>>>){
		// open server
		println!( "[sv] opening listener on port: {}", port);
		let listener = TcpListener::bind( ( "localhost", port)).unwrap();
		let mut acceptor = listener.listen();
		// wait for incoming streams
		for stream in acceptor.incoming() {
			if let Ok( client) = stream {
				// clone motedb and remotedb handles
				let motedb_arc = motedb_arc.clone();
				let remotedb_arc = remotedb_arc.clone();
				// spawn client handler
				spawn( proc() {
					Hub::serve( client, motedb_arc, remotedb_arc);})}}
		// close server
		drop( acceptor);}

	fn serve( mut client_stream: TcpStream, motedb_arc: Arc<Mutex<Vec<Mote>>>,
			remotedb_arc: Arc<Mutex<Vec<RemoteHub>>>){
		let client_addr = client_stream.peer_name().unwrap();
		println!( "[sv] client connected: {}", client_addr);

		// start reading commands from client
		let mut reader = BufferedReader::new( client_stream.clone());
		while let Ok( line) = reader.read_line() {
			// parse command
			let line_trimmed = line.as_slice().trim();
			let command = Command::from_str( line_trimmed);
			if command.is_none() {
				println!( "[sv] failed to parse command from {}: {}",
					client_addr, line_trimmed);
				continue;}
			let command = command.unwrap();

			// handle command
			let response : Response = match command {
				// todo: adjust hello command to contain remote addr
				Hello => Okay,
				// request all the remotes we know about
				OthersReq => Hub::get_others( &remotedb_arc),
				// record that this remote has the specified mote
				HaveDec( _hash) => Deny,
				// reply with whether we have the specified mote
				HaveReq( hash) => Hub::have_mote( &motedb_arc, hash),
				// return the given mote ( if we have it )
				Get( hash) => Hub::get_mote( &motedb_arc, hash),
				// return whether we want the specified mote
				WantReq( hash) => Hub::want_mote( &motedb_arc, hash),
				// accept the given mote into our db
				Take( json) => Hub::take_mote( &motedb_arc, json),
			};

			client_stream.write_line(
				response.to_string().as_slice()).ok();}

		// clean up
		client_stream.close_write().ok();
		println!( "[sv] client disconnected: {}", client_addr);}

	fn bootstrap( remotedb_arc: Arc<Mutex<Vec<RemoteHub>>>){
		let others_req_msg = OthersReq.to_string();
		loop {
			// copy addresses from current remotedb
			let remotedb = remotedb_arc.lock();
			let mut remotes_addr : Vec<SocketAddr> = Vec::new();
			for ref remote in remotedb.deref().iter() {
				remotes_addr.push( remote.addr.clone());}
			drop( remotedb);

			// get new remotes from existing remotes
			let mut new_remotes : Vec<SocketAddr> = Vec::new();
			for &addr in remotes_addr.iter() {
				// connect to remote
				//println!( "attempting to bootstrap against: {}", addr);
				let remote_stream =
					TcpStream::connect_timeout(
						addr.clone(), Duration::seconds( 10));
				if remote_stream.is_err() {
					//println!( "failed to connect to {}", addr);
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
					println!( "[bs] failed to read response from {}", addr);
					continue;}
				let line = line.unwrap();
				let line_trimmed = line.as_slice().trim();
				let response = Response::from_str( line_trimmed);
				if response.is_none() {
					println!( "[bs] failed to parse response from {}: {}",
						addr, line_trimmed);
					continue;}
				let response = response.unwrap();

				// handle response
				match response {
					OkayResult( Json::Array( list)) => {
						for ref entry in list.iter() {
							match *entry {
								&Json::String( ref string) => {
									let new_addr : Option<SocketAddr> =
										from_str( string.as_slice());
									if new_addr.is_some(){
										new_remotes.push( new_addr.unwrap());}}
								_ => ()}}}
					Deny => {
						println!( "[bs] deny response from {}", addr);}
					bad => {
						println!( "[bs] bad response from {}: {}",
							addr, bad);}}

				// move on to next remote
				continue;}

			//write new remotes to db
			let mut remotedb = remotedb_arc.lock();
			let size = remotedb.len();
			for &new_addr in new_remotes.iter() {
				let new_remote = RemoteHub::new( new_addr.clone());
				if ! remotedb.as_slice().contains( &new_remote) {
					remotedb.push( new_remote);}}
			if size != remotedb.len() {
				println!( "[bs] remote list updated: {}", remotedb.as_slice());}
			drop( remotedb);

			//wait for a while before polling again
			sleep( Duration::seconds( BOOTSTRAP_PAUSE_SECONDS));}}

	fn push( motedb_arc: Arc<Mutex<Vec<Mote>>>,
			remotedb_arc: Arc<Mutex<Vec<RemoteHub>>>){
		loop {
			// copy addresses from current remotedb
			let remotedb = remotedb_arc.lock();
			let mut remotes_addr : Vec<SocketAddr> = Vec::new();
			for ref remote in remotedb.deref().iter() {
				remotes_addr.push( remote.addr.clone());}
			drop( remotedb);

			// copy motedb from current motedb
			let motedb = motedb_arc.lock();
			let motedb_copy = motedb.clone();
			drop( motedb);
			let motedb = motedb_copy;

			// push 
			for &remote_addr in remotes_addr.iter() {
				// connect to remote
				//println!( "attempting to bootstrap against: {}", remote_addr);
				let remote_stream =
					TcpStream::connect_timeout(
						remote_addr.clone(), Duration::seconds( 10));
				if remote_stream.is_err() {
					//println!( "failed to connect to {}", remote_addr);
					continue;}
				let mut remote_stream = remote_stream.unwrap();
				let mut reader = BufferedReader::new( remote_stream.clone());

				for mote in motedb.iter() {
					let mote_hash = hash::hash( mote);

					// send want? request
					let want_req_msg = WantReq( mote_hash).to_string();
					remote_stream.write_line( want_req_msg.as_slice()).ok();

					// parse want? response
					let line = reader.read_line().ok();
					if line.is_none() {
						println!( "[ps] failed to read response from {}", remote_addr);
						continue;}
					let line = line.unwrap();
					let line_trimmed = line.as_slice().trim();
					let response = Response::from_str( line_trimmed);
					if response.is_none() {
						println!( "[ps] failed to parse response from {}: {}",
							remote_addr, line_trimmed);
						continue;}
					let response = response.unwrap();

					// handle want? response
					match response {
						Affirm => (),
						Deny => {
							println!( "[ps] remote {} declined mote {:x}",
								remote_addr, mote_hash);
							continue;}
						bad => {
							println!( "[ps] bad response from {}: {}", remote_addr, bad);
							continue;}}

					//sent take request
					let take_req_msg = Take(
						mote.to_msg().to_json()).to_string();
					remote_stream.write_line( take_req_msg.as_slice()).ok();

					// parse take response
					let line = reader.read_line().ok();
					if line.is_none() {
						println!( "[ps] failed to read response from {}", remote_addr);
						continue;}
					let line = line.unwrap();
					let line_trimmed = line.as_slice().trim();
					let response = Response::from_str( line_trimmed);
					if response.is_none() {
						println!( "[ps] failed to parse response from {}: {}",
							remote_addr, line_trimmed);
						continue;}
					let response = response.unwrap();

					// handle take response
					match response {
						Affirm => {
							println!( "[ps] remote {} accepted mote {:x}",
								remote_addr, mote_hash);}
						Deny => {
							println!( "[ps] remote {} denied mote {:x}",
								remote_addr, mote_hash);}
						bad => {
							println!( "[ps] bad response from {}: {}",
								remote_addr, bad);}}

					//move on to next mote
					continue;}

				// move on to next remote
				drop( remote_stream);
				continue;}

			//wait for a while until polling
			sleep( Duration::seconds( PUSH_PAUSE_SECONDS));}}

	// command handling functions

	fn get_others(
			remotedb_arc: &Arc<Mutex<Vec<RemoteHub>>>) -> Response {
		let remotedb = remotedb_arc.lock();
		let mut list : Vec<Json> = Vec::new();
		// push the addr of every remote we know of
		for remote in remotedb.iter() {
			list.push( Json::String( remote.to_string()))}
		drop( remotedb);
		OkayResult( Json::Array( list))}

	fn have_mote(
			motedb_arc: &Arc<Mutex<Vec<Mote>>>, hash: u64) -> Response {
		let motedb = motedb_arc.lock();
		let mut response = Deny;
		for mote in motedb.iter() {
			if hash == hash::hash( mote) {
				response = Affirm;
				break;}}
		drop( motedb);
		response}

	fn get_mote(
			motedb_arc: &Arc<Mutex<Vec<Mote>>>, hash: u64) -> Response {
		let motedb = motedb_arc.lock();
		let mut response = Deny;
		for mote in motedb.iter() {
			if hash == hash::hash( mote) {
				let mote_json = mote.to_msg().to_json();
				response = OkayResult( mote_json);
				break;}}
		drop( motedb);
		response}

	fn want_mote(
			motedb_arc: &Arc<Mutex<Vec<Mote>>>, hash: u64) -> Response {
		let motedb = motedb_arc.lock();
		let mut response = Affirm;
		for mote in motedb.iter() {
			if hash == hash::hash( mote) {
				response = Deny;
				break;}}
		drop( motedb);
		response}

	fn take_mote(
			motedb_arc: &Arc<Mutex<Vec<Mote>>>, json: Json) -> Response {
		// decode mote
		let mut decoder = json::Decoder::new( json);
		let mote_msg : Option<MoteMsg> =
			Decodable::decode( &mut decoder).ok();
		if mote_msg.is_none() { return Error;}
		let mote = Mote::from_msg( &mote_msg.unwrap());
		if mote.is_none() { return Error;}
		let mote = mote.unwrap();
		let mote_hash = hash::hash( &mote);
		println!("[sv] received new mote: {:x} :: {}", mote_hash, mote);

		let mut motedb = motedb_arc.lock();
		motedb.push( mote);
		drop( motedb);
		Okay}
}

