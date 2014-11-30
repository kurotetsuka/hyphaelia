#!/bin/bash
## little script for getting the repository all set up

pacman -S \
	httpd \
	mariadb \
	ruby \

gem install \
	dotenv \
	sinatra \
	thin \
	json \
