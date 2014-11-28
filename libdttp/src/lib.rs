#![crate_name="dttp"]
#![crate_type="lib"]

// rustc feature enables
#![feature(globs)]

// library imports
extern crate serialize;
extern crate time;

//reexports
pub use auth::Auth;
pub use dt::Datetime;
pub use hub::Hub;
pub use mote::Mote;

// modules
pub mod auth;
pub mod dt;
pub mod hub;
pub mod mote;

// tests
#[test]
fn it_works(){}
