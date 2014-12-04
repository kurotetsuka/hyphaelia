
use hyph;

insert into Auths values (
	'kurotetsuka' , 'kurotetsuka@gmail.com', '0a1a20c0', 'password');
insert into Auths values (
	'salij' , 'jojosali@hotmail.com', null, 'password');
insert into Auths values (
	'randy' , 'randy.fortier@uoit.ca', null, 'password');

insert into Motes values (
	0, 'test test :)', 'markdown', 'kurotetsuka', '7de.150.2932e00',
	'cac3f6a6ce8ca2eb', 'dGVzdCB0ZXN0IHlvIHlvIGJybw==', 'AAAAAAAAAAA=')
insert into Motes values (
	1, 'test test 2 :)', 'markdown', 'salij', '7de.150.2933e00',
	'cac3f6a6ce8ca2eb', 'dGVzdCB0ZXN0IHlvIHlvIGJybw==', 'AAAAAAAAAAA=')
insert into Motes values (
	2, 'test test 3 :)', 'markdown', 'randy', '7de.150.2934e00',
	'cac3f6a6ce8ca2eb', 'dGVzdCB0ZXN0IHlvIHlvIGJybw==', 'AAAAAAAAAAA=')

create table Claims (
	sector varchar(20),
	hash bigint,
	claim_dt varchar( 16),
	revok_dt varchar( 16),
	unrevok_dt varchar( 16)
);

insert into Claims values (
	'tech', 0, '7de.151.2932e00', null, null)
insert into Claims values (
	'sci', 1, '7de.152.2932e00', null, null)
insert into Claims values (
	'cult', 2, '7de.153.2932e00', null, null)
