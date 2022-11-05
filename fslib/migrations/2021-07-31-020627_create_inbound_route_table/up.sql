-- Your SQL goes here
CREATE TABLE inbound_routes (
id SERIAL,
context VARCHAR(64) NOT NULL,
condition VARCHAR(512) NOT NULL,
dest_extension VARCHAR(64) NOT NULL,
PRIMARY KEY(id)
);
