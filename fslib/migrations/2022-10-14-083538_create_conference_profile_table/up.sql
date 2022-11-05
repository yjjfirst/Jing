CREATE TABLE conference_profiles (
       id SERIAL,
       name VARCHAR(128) NOT NULL,
       description VARCHAR(512),
       PRIMARY KEY(id)
);

INSERT INTO conference_profiles (name, description) VALUES ('default', 'Default profile');
