CREATE TABLE conferences (
       id SERIAL,
       exten VARCHAR(32) NOT NULL,
       name VARCHAR(128) NOT NULL,
       domain_id INT NOT NULL,
       conference_profile_id INT NOT NULL,
       description VARCHAR(512) NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id),
       FOREIGN KEY(conference_profile_id) REFERENCES conference_profiles(id)
);
