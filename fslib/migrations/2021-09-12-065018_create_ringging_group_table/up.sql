CREATE TABLE ringing_groups (
       id SERIAL,
       name VARCHAR(32) NOT NULL,
       group_id VARCHAR(32) NOT NULL UNIQUE,
       domain_id INT NOT NULL,
       description VARCHAR(256),
       ring_time INT NOT NULL DEFAULT 20,
       ring_strategy VARCHAR(32) NOT NULL DEFAULT 'all',
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id)
       );
