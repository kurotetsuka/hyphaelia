// library uses
use std::io::net::ip::SocketAddr;

// local uses

pub struct RemoteHub {
	pub addr: SocketAddr,
	pub motedb: Vec<u64>,
}
impl RemoteHub {
	pub fn new( addr: SocketAddr) -> RemoteHub {
		RemoteHub {
			addr: addr,
			motedb: Vec::new(),}}
}
