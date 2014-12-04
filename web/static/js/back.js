
function load_mote(){
	//var dest = $( "#content");
	var request = location.protocol + "//";
	request += location.hostname + ":7860";
	request += "/mote/304325";
	//dest.html( "loading...");
	$.get( request, load_mote_content)
		.fail( load_mote_exception);}

function load_mote_content( data){
	console.log( data);
	/*$( "#content").html(
		"<pre><code>" + JSON.stringify( data) + "</code></pre>");*/}
function load_mote_exception( exception){
	console.log( "connection failed");
	/*var dest = $( "#content");
	dest.html( "connection failed");*/}
