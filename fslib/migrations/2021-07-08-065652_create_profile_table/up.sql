-- Your SQL goes here
CREATE TABLE profiles (
id SERIAL,
name VARCHAR(256) NOT NULL,
PRIMARY KEY (id)
);

INSERT INTO profiles (name) VALUES ('internal');
INSERT INTO profiles (name) VALUES ('external');
