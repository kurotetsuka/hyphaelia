// library uses
use std::fmt;

// local uses

/// class that defines the signing agent of a mote
pub struct Auth {
	pub user: Option<String>,
	pub comment: Option<String>,
	pub email: Option<String>,
	pub id: Option<u32>,
}
impl Auth {
	pub fn new() -> Auth {
		Auth {
			user: Some( "kurotetsuka".to_string()),
			comment: None,
			email: Some( "kurotetsuka@gmail.com".to_string()),
			id: Some( 0x0a1a20c0)}}
}
impl fmt::Show for Auth {
	fn fmt( &self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let self_tuple = (
			self.user.as_ref(), self.comment.as_ref(),
			self.email.as_ref(), self.id.as_ref());
		match self_tuple {
			( Some( user), Some( comment), Some( email), Some( &id)) =>
				write!( formatter,
					"{} ({}) <{}> :: {:08x}",
					user, comment, email, id),
			( Some( user), Some( comment), Some( email), None) =>
				write!( formatter,
					"{} ({}) <{}>",
					user, comment, email),
			( Some( user), Some( comment), None, Some( &id)) =>
				write!( formatter,
					"{} ({}) :: {:08x}",
					user, comment, id),
			( Some( user), Some( comment), None, None) =>
				write!( formatter,
					"{} ({})",
					user, comment),
			( Some( user), None, Some( email), Some( &id)) =>
				write!( formatter,
					"{} <{}> :: {:08x}",
					user, email, id),
			( Some( user), None, Some( email), None) =>
				write!( formatter,
					"{} <{}>",
					user, email),
			( Some( user), None, None, Some( &id)) =>
				write!( formatter,
					"{} :: {:08x}",
					user, id),
			( Some( user), None, None, None) =>
				write!( formatter,
					"{}",
					user),
			( None, Some( comment), Some( email), Some( &id)) =>
				write!( formatter,
					"({}) <{}> :: {:08x}",
					comment, email, id),
			( None, Some( comment), Some( email), None) =>
				write!( formatter,
					"({}) <{}>",
					comment, email),
			( None, Some( comment), None, Some( &id)) =>
				write!( formatter,
					"({}) :: {:08x}",
					comment, id),
			( None, Some( comment), None, None) =>
				write!( formatter,
					"({})",
					comment),
			( None, None, Some( email), Some( &id)) =>
				write!( formatter,
					"<{}> :: {:08x}",
					email, id),
			( None, None, Some( email), None) =>
				write!( formatter,
					"<{}>",
					email),
			( None, None, None, Some( &id)) =>
				write!( formatter,
					":: {:08x}",
					id),
			( None, None, None, None) =>
				write!( formatter, ":null:"),}}
}
