CREATE TABLE user_variables (
       id SERIAL,
       user_id INT NOT NULL,
       name VARCHAR(128) NOT NULL,
       value VARCHAR(128) NOT NULL,
       UNIQUE(user_id, name),
       PRIMARY KEY(id),
       FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);
