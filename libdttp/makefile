#globals
default: build
freshen: clean build
clean:
	rm -rf bin/*

#vars
version = 0.0.1
warnings = -A dead_code -A unused_variable
options = $(warnings) -g -L bin
lib_opt = --crate-type=rlib
dylib_opt = --crate-type=dylib
#other vars
package_file = pkg/ectocat-$(version).zip

#includes
include deps.mk
include lists.mk

#compilation definitions
$(binaries): bin/% : src/%.rs
	rustc $(options) $< -o $@
$(libs): bin/lib%.rlib : src/%.rs
	rustc $(options) $(lib_opt) $< -o $@
$(dylibs): bin/lib%.so : src/%.rs
	rustc $(options) $(dylib_opt) $< -o $@

#commands
build: $(libs) $(dylibs) $(binaries)

package: $(package_file)
$(package_file): $(binaries)
	zip -FSr $(package_file) \
		bin data legal notes src tool web\
		deps.mk lists.mk makefile license.md readme.md \
		pkg/.gitignore

#tests
test: test-game

test-asdf: bin/asdf
	$<

test-dttp: bin/dttp_test
	$<
