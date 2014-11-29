#!/bin/bash

sizes="16 24 32 64 128 256"

mkdir -p gen/img

for size in $sizes; do
	sizestr="${size}x${size}"
	gm convert res/hyph.png \
		-resize ${sizestr} \
		gen/img/hyph_${sizestr}.png
	echo "done hyph_${sizestr}.png"
done
