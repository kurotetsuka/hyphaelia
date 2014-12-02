// library uses

// local uses

#[deriving( PartialEq, Eq)]
#[deriving( Hash)]
#[deriving( Encodable, Decodable)]
pub enum Mode {
	Track,
	Bootstrap,
	Push,
	Pull,
	Serve,
}
