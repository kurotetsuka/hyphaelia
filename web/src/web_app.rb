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
			set :public_dir, settings.root + '/../static'
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
		get '/login' do
			@head = erb :head
			erb :login
		end

		# main pages
		get '/register' do
			@head = erb :head
			erb :register
		end

		# main pages
		get '/s/:sector' do
			@head = erb :head
			@sector = params[:sector]
			erb :sector
		end

		#error pages
		not_found do
			'This is an amazing 404 page!'
		end
	end
end
