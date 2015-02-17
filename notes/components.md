## components of this project

### spec stuff
 - dttp-spec : distrubuted trustnet transmission protocols
 - hyphaelia-spec : leveraging of dttp to create a message-thread protocol

### library stuff
 - libdttp : rust library for dealing with dttp

### hub stuff
 - dttpd : hub daemon
 - dttpdctl : hub cli frontend
 - dttpdctl-ui : hub gui frontend

### user (client) stuff
 - dttpc : minimal client cli frontend
 - dttp-shell : client shell frontend
 - name? : fancy client cli frontend
 - hyphic : minimal client gui frontend ( gtk? qt? vala? )
 - hyphaelia-web : web front-end to a hub

### user (client) stuff
 - dttp-rest : rest api for accessing a hub

### rfc stuff
 - dttp-rfcs : rfcs for the design of dttp
 - hyphaelia-rfcs : rfcs for the architecture of hyphaelia

### future ideas and stuff
 - package manager that uses dttp instead of http
	 - dependency information uses votes for the min/max version of a dependency
 - cryptocoin-based marketplace

