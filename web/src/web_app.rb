#!/usr/bin/env ruby

# library imports
require 'dotenv'
require 'erubis'
require 'sinatra'
require "sinatra/reloader"

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
			set :server, %w[thin mongrel webrick]
			#set :bind, 'localhost'
			set :bind, '0.0.0.0'
			set :port, 8080
			set :views, settings.root + '/../views'
			set :public_dir, settings.root + '/../build'
		end
		configure :development do
			register Sinatra::Reloader
		end

		# main pages
		get '/' do
			@head = erb :head
			erb :index
		end


		# main pages
		get '/s/:sector' do
			@sector = params[:sector]
			if @sector == "null"
				erb :motes
			end
		end

		#error pages
		not_found do
			'This is nowhere to be found.'
		end
	end
end
