
var username;
var password;
var email;
var keyID;
var obj = new XMLHttpRequest();

$(function register()) {

	username = document.getElementByID('register-username');
	password = document.getElementByID('register-password');
	email = document.getElementByID('register-email');
	//keyID = ;

	function validateForm(){
		var x = document.forms["register-form"]["register-email"].value;
		var atpos = x.indexOf("@");
		var dotpos = x.lastIndexOf(".");
		if(atpost < 1 || dotpos < atpos +2 || dotpos + 2 >= x.length){
			alert("Not a valid e-mail address");
			return false;
		}
	}
	obj.addEventListener('error', function(event)){
		document.getElementByID('error-msg').innerHTML = 
			"Something went wrong, please try again.";
	}

	obj.open('POST', '');


	//add to db 

}