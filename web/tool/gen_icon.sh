#!/bin/bash

sizes="16 24 32 64 128 256"

for size in $sizes; do
	sizestr="${size}x${size}"
	gm convert res/hyph.png \
		-resize ${sizestr} \
		build/img/hyph_${sizestr}.png
done
