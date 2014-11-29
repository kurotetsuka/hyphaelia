#!/usr/bin/env ruby

# library imports
require 'sinatra'
require 'json'

# local imports

module Hyph
	class RestApi < Sinatra::Base
		extend self

		# sinatra config
		#set :bind, 'localhost'
		set :bind, '0.0.0.0'
		set :port, 7860

		#methods and stuff
		def set_dbcreds( user, pass)
			@user = user
			@pass = pass
		end

		def db_test
			printf( "user: %s, pass: %s\n")
		end

		# sinatra stuff
		get '/' do
			content_type :json
			response['Access-Control-Allow-Origin'] = '*'
			#RestApi.test_miner
			asdf = {
				:asdf => "useful data yo",
				:fdsa => "there",
				:oiu => [ "bubba", "boo"]}
			asdf.to_json()
		end
	end
end