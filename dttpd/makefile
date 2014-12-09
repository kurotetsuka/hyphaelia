#globals
default: build
freshen: clean build
clean:
	cargo clean

#vars

#commands
build:
	cargo build

#tests
test: test-p1

test-p1: build
	target/dttpd 8960

test-p2: build
	target/dttpd 8961

test-p3: build
	target/dttpd 8962

test-p4: build
	target/dttpd 8963
