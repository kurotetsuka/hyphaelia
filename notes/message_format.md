# core message format

## examples

### unencrypted message
#### broacast
```json
{
	// dttp version 0.0.1 alpha
	"dttpv": "0.0.1a",

	"meta": "awesome site bros",

	"data": "hey guys, check this out: [cool site](www.cool-site.com)",

	"class": "markdown", //alternatively mimetype:"text/markdown"

	//auth or author?
	"auth": "kurotetsuka <kurotetsuka@gmail.com>::0x1234abcd",

	//2014-09-23 20:30:46 in +0h time zone
	// dt or datetime or date or time?
	"dt": "7de917.",

	//64-bit (is that big enough?) uint
	"salt": "a3f0",

	//pgp signature of [ meta, data, auth, dt, salt]
	// with pgp id key
	// key may or may not match known id key of user ( auth field )
	"sig": "",}

#### broadcast alternate design
```json
{
	"dttpv": "0.0.1a",
	"meta": "re:54fc89a", // reply to short hash of previous message
	"data": "man, that's awesome lol.",
	"class": "markdown",
	"auth": "kurotetsuka <kurotetsuka@gmail.com>",
	"dt": "7de917.", //2014-09-23 20:30:46 in +0h time zone
	"salt": "",
	"sig": "",}

### partially encrypted message
```json
{
	"meta": "",
	"data": "1234aAwofiejfweifOIAIJFE832u92JFO", //b64 encoded byte-block
	"class": "markdown",
	"auth": "kurotetsuka <kurotetsuka@gmail.com>",
	"dt": "7de917.", //2014-09-23 20:30:46
	"salt": "",
	"sig": "",}

