-- Your SQL goes here
CREATE TABLE portal_tokens (
       id SERIAL,
       portal_user_id int not null,
       token VARCHAR(256) not null,
       expire_at TIMESTAMP with time zone not null,
       PRIMARY KEY(id),
       CONSTRAINT fk_portal_user
                  FOREIGN KEY(portal_user_id)
                  REFERENCES portal_users(id)

);
