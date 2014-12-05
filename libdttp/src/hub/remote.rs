// library uses
use std::io::net::ip::SocketAddr;

// local uses

pub struct RemoteHub {
	pub addr: SocketAddr,
	pub motedb: Vec<u64>,
}
