// library uses
use std::collections::HashMap;
use std::io::net::ip::SocketAddr;

// local uses
use super::mode::*;

pub struct RemoteHub {
	pub addr: SocketAddr,
	pub motedb: Vec<u64>,
	pub authdb: Vec<u64>,
	pub profile: HashMap<Mode, bool>,
}

#[deriving( Encodable, Decodable)]
pub struct RemoteSpec {
	pub addr: String,
	pub profile: HashMap<Mode, bool>,
}

