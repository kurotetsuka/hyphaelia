#globals
default: build
freshen: clean build
clean:
	rm -rf bin/*

#vars
warnings = -A dead_code -A unused_variable
options = $(warnings) -g -L bin
lib_opt = --crate-type=rlib

#includes
include deps.mk
include lists.mk

#compilation definitions
$(binaries): bin/% : src/%.rs
	rustc $(options) $< -o $@
$(libraries): bin/%.rlib : src/%.rs
	rustc $(options) $(lib_opt) $< -o $@

#commands
build: $(libraries) $(binaries)

#tests
test: test-game

test-asdf: bin/asdf
	$<

test-game: bin/go_game
	$<
