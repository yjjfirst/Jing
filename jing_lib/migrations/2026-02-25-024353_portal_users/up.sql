-- Your SQL goes here
CREATE TABLE portal_users (
       id SERIAL not null,
       username VARCHAR(128) not null,
       password VARCHAR(128) not null,
       PRIMARY KEY(id)
);

INSERT INTO portal_users (username, password) VALUES ('martin', 'ymzh_2008');