// library uses
//use std::fmt;
use std::rand::Rng;

// local uses


pub trait SecretKey {
	//fn decrypt( &self, data: &[u8]) -> Vec<u8>;
	fn sign( &self, data: &[u8]) -> Vec<u8>;
}
pub trait PublicKey {
	//fn encrypt( &self, data: &[u8]) -> Vec<u8>;
	fn verify( &self, data: &[u8], sig :&[u8]) -> bool;
}
pub trait KeyPair< T: SecretKey, U: PublicKey > {
	fn generate<R: Rng>( rng: &mut R) -> ( T, U );
	fn retrieve_pubkey( sec_key: &T) -> U;
}


pub type FakeSecKey = [u8, .. 8];
pub type FakePubKey = [u8, .. 8];
impl SecretKey for FakeSecKey {
	//fn decrypt( &self, data: &[u8]) -> Vec<u8>;
	fn sign( &self, _data: &[u8]) -> Vec<u8> {
		Vec::from_elem( 8, 0x00)}
}
impl PublicKey for FakePubKey {
	//fn encrypt( &self, data: &[u8]) -> Vec<u8>;
	fn verify( &self, _data: &[u8], _sig :&[u8]) -> bool {
		true}
}

pub type FakeKeyPair = ( FakeSecKey, FakePubKey);
impl KeyPair< FakeSecKey, FakePubKey > for FakeKeyPair {
	fn generate<R: Rng>( _rng: &mut R) -> ( FakeSecKey, FakePubKey ) {
		( [ 0u8, 0, 0, 0, 0, 0, 0, 0],
			[ 0u8, 0, 0, 0, 0, 0, 0, 0])}
	fn retrieve_pubkey( _sec_key: &FakeSecKey) -> FakePubKey {
		[ 0u8, 0, 0, 0, 0, 0, 0, 0]}
}
