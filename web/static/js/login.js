
var username;
var password;
var obj = new XMLHttpRequest();

$(function login()) {

	var u = document.getElementByID('login-username');
	var p = document.getElementByID('login-password');

	username = u;
	password = p;

	obj.addEventListener('error', function(event)){
		alert('Login unsuccessful, please try again.');
	});

}