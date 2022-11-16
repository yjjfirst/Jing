CREATE TABLE extension_types (
       id SERIAL,
       name VARCHAR(32) NOT NULL UNIQUE,
       PRIMARY KEY(id)
);

INSERT INTO extension_types (name) VALUES ('user');
INSERT INTO extension_types (name) VALUES ('ringgroup');
INSERT INTO extension_types (name) VALUES ('ivr');
INSERT INTO extension_types (name) VALUES ('sound');
INSERT INTO extension_types (name) VALUES ('conference');
INSERT INTO extension_types (name) VALUES ('queue');
