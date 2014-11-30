<!DOCTYPE html>
<html lang="en">
<head>
	<?php include $_SERVER['DOCUMENT_ROOT'].'/include/root.php';?>
	<?php include $root.'/include/head.php';?>
	<link href="/css/index.css" rel="stylesheet">
</head>
<body>
	<!-- page content start -->
	<!--[if lte IE 9 ]>
	<div class="ink-alert basic" role="alert">
		<button class="ink-dismiss">&times;</button>
		<p>
			<strong>Get a better browser, son.</strong>
			( <a href="http://browsehappy.com/">upgrade to a modern  browser</a> to improve your web experience, and make this page look half-decent. )
		</p>
	</div>
	<![endif]-->
<nav class="ink-navigation ink-grid ie7">
        <ul class="menu horizontal black push-left medium-push-center small-push-center tiny-push-center">
            <li class="hide-all show-tiny show-small show-medium"></li>
            <li><a href="#">Welcome to Hyphaelia [username]</a></li>
        </ul>
        <ul class="menu horizontal black push-right hide-small hide-tiny hide-medium">
            <li class="active">
                <a href="#" class="fa fa-rocket"> Username</a>
            </li>                
            <li><span>
                <a href="#"class="fa fa-envelope"> Inbox</a>
              </span>
            </li>
            <li>
                <a href="#" class="fa fa-cog"> Preferences</a>
            </li>           
            <li>
                <a href="#">Login</a>
            </li>                      
        </ul>
    </nav>
	<div class="wrap">
		<div class="ink-grid vertical-space">
			<a href="/index.php">
				<img id="hyph_icon" src="/img/hyph.svg"/></a>
			<h1>Hyphaelia.io</h1>
			<p>Lorem ipsum dolor sit amet, oblique liberavisse et eos, eam no impetus volumus. Nemore epicuri lucilius ad sit.</p>
		</div>
			<!-- content stuff -->
<nav class="ink-navigation ink-grid">
    <ul class="pagination pills black">
        <li class="active"><a href="#">Home</a></li>
        <li><a href="#">Register</a></li>
        <li><a href="#">Browse</a></li>
        <li><a href="#">About</a></li>
    </ul>
</nav>
<div class="ink-grid vertical-space">
	<h3 style="margin-bottom:5px;margin-top:30px;">What is <i>Hyphaelia</i></h3>
	<p>Lorem ipsum dolor sit amet, oblique liberavisse et eos, eam no 
		impetus volumus. Nemore epicuri lucilius ad sit. Id erat mundi 
		suavitate ius, no eum posse zril democritum. Ut rebum possim meliore
	 	nec, qui simul iriure aperiri ne. At mundi definitiones quo, 
	 	vivendum definitiones cu eum. Decore tibique et qui, eos choro 
	 	saepe omittantur ei, volutpat temporibus nam ad.
	 	Lorem ipsum dolor sit amet, oblique liberavisse et eos, eam no 
		impetus volumus. Nemore epicuri lucilius ad sit. Id erat mundi 
		suavitate ius, no eum posse zril democritum. Ut rebum possim meliore
	 	nec, qui simul iriure aperiri ne. At mundi definitiones quo, 
	 	vivendum definitiones cu eum. Decore tibique et qui, eos choro 
	 	saepe omittantur ei, volutpat temporibus nam ad.</p>
</div>
<div class="ink-grid vertical-space">
	<h3 style="margin-bottom:5px;margin-top:30px;">Register | Login</h3>
	<p>To take full advantage of Hyphaelia's diverse intellectual community
		you must first <b>register</b>, or <b>login</b> if you have completed the
		registration proces.
		</p>

	<form class="ink-form">
    <div class="column-group horizontal-gutters">

    	<div class="control-group all-60 small-90 tiny-90">
    		<label for="name">Username</label>
      	<span><input type="text" name="name" id="name" placeholder="Enter a username..."></span>
      	<br><label for="password">Password</label>
      	<span><input type="text" name="password" id="name" placeholder="Enter a password..."></span>
      	<br><label for="email">Email &nbsp; &nbsp; &nbsp; &nbsp; </label>
      	<span><input type="text" name="email" id="name" placeholder="Enter a valid email..."></span>
      	<br><br><label for="keyID">Key ID &nbsp; &nbsp; &nbsp; </label>
      	<span><input type="text" name="keyID" id="name" placeholder="Optional"></span>
      </div>
    
      <div class="control-group all-40 small-100 tiny-100">
      	<label for="name">Already registered?</label>
     		<div class="control">
      		<span><input type="text" name="name" id="name" placeholder="Username"></span>
      		<span><input type="text" name="password" id="password" placeholder="Password"></span>
      	</div>
     	
    </div>
	</form>
</div>
</div>


<div class="push"></div>
</div>

<footer class="clearfix">
	<div class="ink-grid">
		<ul class="unstyled inline half-vertical-space">
			<li class="active"><a href="#">About</a></li>
			<li><a href="#">Sitemap</a></li>
			<li><a href="#">Contacts</a></li>
		</ul>
		<p class="note">&copy; <a href="https://github.com/kurotetsuka">kurotetsuka</a> 2014</p>
	</div>
</footer>
	<!-- page content end -->
</body>
</html>