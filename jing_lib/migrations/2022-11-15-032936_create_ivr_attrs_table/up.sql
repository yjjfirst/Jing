CREATE TABLE ivr_attrs (
       id SERIAL,
       ivr_id INT NOT NULL,
       name VARCHAR(128) NOT NULL,
       value VARCHAR(128) NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(ivr_id) REFERENCES ivrs(id) ON DELETE CASCADE
);
