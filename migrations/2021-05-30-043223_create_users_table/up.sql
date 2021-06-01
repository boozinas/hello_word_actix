-- Your SQL goes here
-- public.users definition

-- Drop table

-- DROP TABLE public.users;

CREATE TABLE users (
	"name" varchar(128) NULL,
	geopoints varchar(128) NULL,
	id serial NOT NULL,
	CONSTRAINT users_pkey PRIMARY KEY (id)
);