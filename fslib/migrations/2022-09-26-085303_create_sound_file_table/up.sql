CREATE TABLE `sound_files` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `name` VARCHAR(32) NOT NULL UNIQUE,
       `domain_id` INT NOT NULL,
       `description` VARCHAR(1024),
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
