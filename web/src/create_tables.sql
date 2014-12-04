
create database if not exists hyph;

use hyph;

create table if not exists Auths (
	name varchar( 20),
	email varchar( 40),
	key_id varchar( 8),
	pass varchar( 64)
);

create table if not exists Motes (
	hash bigint,
	meta varchar( 40),
	class varchar( 20),
	auth varchar( 20),
	datetime varchar( 16),
	salt varchar( 16),
	data varchar( 120),
	sig varchar( 120)
);

create table if not exists Claims (
	sector varchar( 20),
	hash bigint,
	claim_dt varchar( 16),
	revok_dt varchar( 16),
	unrevok_dt varchar( 16)
);
