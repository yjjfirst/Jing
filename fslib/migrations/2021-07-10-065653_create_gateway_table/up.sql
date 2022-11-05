-- Your SQL goes here
CREATE TABLE gateways (
id SERIAL,
profile_id INT NOT NULL,
gateway_name VARCHAR(256) NOT NULL,
proxy VARCHAR(256) NOT NULL,
register VARCHAR(256) NOT NULL,
username VARCHAR(256),
password VARCHAR(256),
PRIMARY KEY(id),
FOREIGN KEY(profile_id) REFERENCES profiles(id)
);
