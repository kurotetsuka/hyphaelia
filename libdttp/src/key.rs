// library uses
use std::fmt;
use std::rand::Rng;

// local uses


pub trait SecretKey {
	//fn decrypt( data: &[u8]) -> Vec<u8>;
	fn sign( data: &[u8]) -> Vec<u8>;
}
pub trait PublicKey {
	//fn encrypt( data: &[u8]) -> Vec<u8>;
	fn verify( data: &[u8], sig :&[u8]) -> bool;
}
pub trait KeyPair< T: SecretKey, U: PublicKey > {
	fn generate<R: Rng>( rng: &mut R) -> ( T, U );
	fn retrieve_pubkey( sec_key: &T) -> U;
}


pub type FakeSecKey = [u8, .. 8];
pub type FakePubKey = [u8, .. 8];
impl SecretKey for FakeSecKey {
	//fn decrypt( data: &[u8]) -> Vec<u8>;
	fn sign( _data: &[u8]) -> Vec<u8> {
		Vec::from_elem( 8, 0x00)}
}
impl PublicKey for FakePubKey {
	//fn encrypt( data: &[u8]) -> Vec<u8>;
	fn verify( _data: &[u8], _sig :&[u8]) -> bool {
		true}
}
