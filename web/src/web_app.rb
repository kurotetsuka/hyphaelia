#!/usr/bin/env ruby

# library imports
require 'dotenv'
require 'erubis'
require 'mysql2'
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
			#set db options
			set :db_name, ENV['HYPH_db_name']
			set :db_user, ENV['HYPH_db_user']
			set :db_pass, ENV['HYPH_db_pass']
			#set sinatra options
			set :server, %w[thin mongrel webrick]
			enable :sessions
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

		post '/login' do
			client = Mysql2::Client.new(
				:database => settings.db_name,
				:username => settings.db_user,
				:password => settings.db_pass)

			@name = params[ :username]
			name_esc = client.escape( @name)
			pass = params[ :password]

			results = client.query(
				"select pass from Auths where name='#{name_esc}'")
			result = results.first

			if result[ "pass"] == pass
				session[ :user] = @name
				printf( "login success!\n")
				redirect '/s/null'
			else
				printf(
					"login error!: result.pass: %s, pass: %s\n",
					result[ "pass"], pass)
				@error = true
			end

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

		# main pages
		get '/post' do
			@head = erb :head
			erb :post
		end

		#error pages
		not_found do
			'This is an amazing 404 page!'
		end
	end
end
