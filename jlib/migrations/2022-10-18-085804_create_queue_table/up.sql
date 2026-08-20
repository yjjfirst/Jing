CREATE TABLE queues (
       id SERIAL,
       name VARCHAR(128) NOT NULL,
       exten VARCHAR(32) NOT NULL,
       domain_id INT NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id)
);
