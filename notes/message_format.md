# core message format

## examples

### unencrypted message
#### broacast
```json
{
	dttpv: "0.0.1a", // dttp version 0.0.1 alpha
	meta: "bc:awesome site bros", // bc = broadcast
	data: "hey guys, check this out: [cool site](www.cool-site.com)",
	class: "markdown",
	author: "kurotetsuka <kurotetsuka@gmail.com>",
	datetime: "7de917.", //2014-09-23 20:30:46 in +0h time zone
	salt: "",
	signature: "",}

#### broadcast alternate design
```json
{
	dttpv: "v0.0.1a",
	meta: "re:54fc89,", // reply to short hash of previous message
	data: "hey guys, check this out: [cool site](www.cool-site.com)",
	class: "markdown",
	author: "kurotetsuka <kurotetsuka@gmail.com>",
	datetime: "7de917.", //2014-09-23 20:30:46 in +0h time zone
	salt: "",
	signature: "",}

### partially encrypted message
```json
{
	meta: "",
	data: "hey guys, check this out: [cool site](www.cool-site.com)",
	class: "markdown",
	author: "kurotetsuka <kurotetsuka@gmail.com>",
	datetime: "7de917.", //2014-09-23 20:30:46
	salt: "",
	signature: "",}

