CREATE TABLE sound_files (
       id SERIAL,
       name VARCHAR(256) NOT NULL UNIQUE,
       domain_id INT NOT NULL,
       description VARCHAR(1024),
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id)
);
