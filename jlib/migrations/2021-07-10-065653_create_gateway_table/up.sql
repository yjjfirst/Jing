-- Your SQL goes here
CREATE TABLE gateways (
id SERIAL,
profile_id INT NOT NULL,
gateway_name VARCHAR(256) NOT NULL,
PRIMARY KEY(id),
FOREIGN KEY(profile_id) REFERENCES profiles(id)
);
