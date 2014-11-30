#!/usr/bin/env ruby

# library imports
require 'dotenv'
require 'erubis'
require 'sinatra'

# local imports

module Hyph
	class WebApp < Sinatra::Base
		# sinatra config
		configure do
			#load and apply env file
			env = Dotenv::Environment.new( "config/dev.env")
			env.load
			env.apply!
			#set sinatra options
			#set :bind, 'localhost'
			set :bind, '0.0.0.0'
			set :port, 8080
			set :views, settings.root + '/../views'
		end

		# sinatra stuff
		get '/' do
			@name = "jake"
			erb :index
		end
	end
end