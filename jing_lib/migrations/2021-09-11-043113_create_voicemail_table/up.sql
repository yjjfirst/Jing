-- Your SQL goes here
CREATE TABLE voicemails (
id SERIAL,
user_id INT NOT NULL UNIQUE,
password VARCHAR(32) NOT NULL,
email VARCHAR(128),
PRIMARY KEY(id),
FOREIGN KEY(user_id) REFERENCES users(id)
);
