
var username;
var password;
var email;
var keyID;

function register() {

	username = $('#register-username');
	password = $('#register-password');
	email = $('#register-email');

	var msg = $('#error-msg');
	msg.html("Please ensure you have entered all fields correctly");
}