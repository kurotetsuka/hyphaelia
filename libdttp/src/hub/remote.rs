// library uses
use std::io::net::ip::SocketAddr;
use std::fmt;

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
impl fmt::Show for RemoteHub {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "(remote {} {})", self.addr, self.motedb)}
}
