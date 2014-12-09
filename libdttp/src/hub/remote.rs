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
impl PartialEq for RemoteHub {
	fn eq( &self, other: &RemoteHub) -> bool {
		self.addr.eq( &other.addr)}
	fn ne( &self, other: &RemoteHub) -> bool {
		self.addr.ne( &other.addr)}
}
impl fmt::Show for RemoteHub {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!( formatter, "(remote {} {})", self.addr, self.motedb)}
}
