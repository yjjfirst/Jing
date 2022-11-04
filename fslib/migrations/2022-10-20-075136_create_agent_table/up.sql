CREATE TABLE `agents` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `domain_id` INT NOT NULL,
       `user_id` INT NOT NULL,
       `name` VARCHAR(128) NOT NULL UNIQUE,
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id),
       FOREIGN KEY(user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
