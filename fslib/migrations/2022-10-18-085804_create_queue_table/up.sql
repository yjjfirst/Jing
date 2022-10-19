CREATE TABLE `queues` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `name` VARCHAR(128) NOT NULL,
       `exten` VARCHAR(32) NOT NULL,
       `domain_id` INT NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
