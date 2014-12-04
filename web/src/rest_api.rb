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
			set :db_host, ENV['HYPH_db_host']
			set :db_name, ENV['HYPH_db_name']
			set :db_user, ENV['HYPH_db_user']
			set :db_pass, ENV['HYPH_db_pass']
			#set sinatra options
			set :server, %w[thin mongrel webrick]
			#set :bind, 'localhost'
			set :bind, '0.0.0.0'
			set :port, 7860
		end

		# sinatra stuff
		get '/mote/:hash' do
			content_type :json
			response['Access-Control-Allow-Origin'] = '*'

			mote = {
				:meta     => "test test :)",
				:class    => "markdown",
				:auth     => "kurotetsuka",
				:datetime => "7de.150.2932e00",
				:salt     => "cac3f6a6ce8ca2eb",
				:data     => "dGVzdCB0ZXN0IHlvIHlvIGJybw==",
				:sig      => "AAAAAAAAAAA="}
			mote.to_json()
		end

		get '/sector/:sec' do
			content_type :json
			response['Access-Control-Allow-Origin'] = '*'

			mote = {
				:meta     => "test test :)",
				:class    => "markdown",
				:auth     => "kurotetsuka",
				:datetime => "7de.150.2932e00",
				:salt     => "cac3f6a6ce8ca2eb",
				:data     => "dGVzdCB0ZXN0IHlvIHlvIGJybw==",
				:sig      => "AAAAAAAAAAA="}
			sector_motes = [ mote, mote, mote, mote, mote, mote, mote ]
			sector_motes.to_json()
		end

		get '/sector/:sec/top' do
			content_type :json
			response['Access-Control-Allow-Origin'] = '*'

			mote = {
				:meta     => "test test :)",
				:class    => "markdown",
				:auth     => "kurotetsuka",
				:datetime => "7de.150.2932e00",
				:salt     => "cac3f6a6ce8ca2eb",
				:data     => "dGVzdCB0ZXN0IHlvIHlvIGJybw==",
				:sig      => "AAAAAAAAAAA="}
			sector_motes = [ mote, mote, mote, mote, mote, mote, mote ]
			sector_motes.to_json()
		end

		get '/sector/:sec/new' do
			content_type :json
			response['Access-Control-Allow-Origin'] = '*'

			mote = {
				:meta     => "test test :)",
				:class    => "markdown",
				:auth     => "kurotetsuka",
				:datetime => "7de.150.2932e00",
				:salt     => "cac3f6a6ce8ca2eb",
				:data     => "dGVzdCB0ZXN0IHlvIHlvIGJybw==",
				:sig      => "AAAAAAAAAAA="}
			sector_motes = [ mote, mote, mote, mote, mote, mote, mote ]
			sector_motes.to_json()
		end
	end
end
