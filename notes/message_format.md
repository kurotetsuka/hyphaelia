# Mote message format

## Examples

### unencrypted message
```json
{
	// dttp version 0.0.1 plain
	"dttpv": "0.0.1",

	"meta": "awesome site bros",

	"data": "hey guys, check this out: [cool site](www.cool-site.com)",

	"class": "markdown", //alternatively mimetype:"text/markdown"

	//auth or author?
	// name (comment) <address> :: fingerprint
	"auth": "kurotetsuka <kurotetsuka@gmail.com> :: 1234abcd",

	//2014-09-23 20:30:46 in +0h time zone
	// dt or datetime or date or time?
	"dt": "7de917.",

	//64-bit (is that big enough?) uint
	"salt": "a3f0",

	//pgp signature of [ meta, data, dt, salt]
	// with pgp id key
	// key may or may not match known id key of user ( auth field )
	"sig": "",}
```

#### broadcast alternate design
```json
{
	"dttpv": "0.0.1-a",
	"meta": "re:54fc89a", // reply to short hash of previous message's meta field
	"data": "man, that's awesome lol.",
	"class": "markdown",
	"auth": "kurotetsuka <kurotetsuka@gmail.com>",
	"dt": "7de917.", //2014-09-23 20:30:46 in +0h time zone
	"salt": "",
	"sig": "",}
```

### partially encrypted message
`gamma` represents the encryption teir

```json
{
	// dttp version 0.0.1 gamma
	"dttpv": "0.0.1-g",
	"meta": "",
	"data": "1234aAwofiejfweifOIAIJFE832u92JFO", //b64 encoded byte-block
	"class": "markdown",
	"auth": "kurotetsuka <kurotetsuka@gmail.com>",
	"dt": "7de917.", //2014-09-23 20:30:46
	"salt": "",
	"sig": "",}
```
