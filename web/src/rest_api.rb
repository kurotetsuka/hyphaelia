#!/usr/bin/env ruby

# library imports
require 'dotenv'
require 'json'
require 'sinatra'

# local imports

module Hyph
	class RestApi < Sinatra::Base
		# sinatra config
		configure do
			#load and apply env file
			env = Dotenv::Environment.new( "config/dev.env")
			env.load
			env.apply!
			#set db options
			set :db_user, ENV['HYPH_db_user']
			set :db_pass, ENV['HYPH_db_pass']
			#set sinatra options
			#set :bind, 'localhost'
			set :bind, '0.0.0.0'
			set :port, 7860
		end

		# sinatra stuff
		get '/' do
			content_type :json
			response['Access-Control-Allow-Origin'] = '*'
			asdf = {
				:db_user => "#{settings.db_user}",
				:db_pass => "#{settings.db_pass}",
				:oiu => [ "bubba", "boo"]}
			asdf.to_json()
		end
	end
end
