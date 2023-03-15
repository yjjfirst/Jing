CREATE TABLE conference_controls (
       id SERIAL,
       name VARCHAR(128) NOT NULL,
       description VARCHAR(512) NOT NULL,
       PRIMARY KEY(id)
);

INSERT INTO conference_controls (name, description) VALUES ('default', 'Default control group');
