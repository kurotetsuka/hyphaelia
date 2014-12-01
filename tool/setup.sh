#!/bin/bash
## little script for getting the repository all set up

sudo pacman -S \
	mariadb \
	ruby \

gem install \
	erubis \
	dotenv \
	json \
	sinatra \
	sinatra-contrib \
	thin \
