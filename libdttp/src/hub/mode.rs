// library uses

// local uses

#[deriving( PartialEq, Eq)]
#[deriving( Copy, Hash)]
#[deriving( Encodable, Decodable)]
pub enum Mode {
	Track,
	Bootstrap,
	Push,
	Pull,
	Serve,
}
