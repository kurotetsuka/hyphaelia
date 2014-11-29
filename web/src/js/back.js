
function load_overview(){
	var dest = $( "#content");
	var request = location.protocol + "//";
	request += location.hostname + ":7860";
	request += "/";
	dest.html( "loading...");
	$.get( request, load_overview_content)
		.fail( load_overview_exception);}

function load_overview_content( data){
	//console.log( data);
	$( "#content").html(
		"<pre><code>" + JSON.stringify( data) + "</code></pre>");}
function load_overview_exception( exception){
	var dest = $( "#content");
	dest.html( "connection failed");}
