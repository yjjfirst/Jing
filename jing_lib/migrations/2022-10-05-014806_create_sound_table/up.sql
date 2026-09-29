CREATE TABLE sounds (
       id SERIAL,
       exten VARCHAR(32) NOT NULL,
       name VARCHAR(32) NOT NULL,
       domain_id INT NOT NULL,
       sound_file_id INT NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id),
       FOREIGN KEY(sound_file_id) REFERENCES sound_files(id)
);
