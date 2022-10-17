CREATE TABLE `conferences` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `exten` VARCHAR(32) NOT NULL,
       `name` VARCHAR(128) NOT NULL,
       `domain_id` INT NOT NULL,
       `conference_profile_id` INT NOT NULL,
       `description` VARCHAR(512),
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id),
       FOREIGN KEY(conference_profile_id) REFERENCES conference_profiles(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
