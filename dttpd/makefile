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
	target/dttpd 192.168.0.38 8960

test-p2: build
	target/dttpd 192.168.0.38 8961

test-p3: build
	target/dttpd 192.168.0.38 8962

test-p4: build
	target/dttpd 192.168.0.38 8963

test-s1: build
	target/dttpd 104.131.4.184 8960

test-s2: build
	target/dttpd 104.131.4.184 8966
