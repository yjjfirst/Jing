CREATE TABLE `sound` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `exten` VARCHAR(32) NOT NULL,
       `name` VARCHAR(32) NOT NULL,
       `domain_id` INT NOT NULL,
       `sound_file_id` INT NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domain(id),
       FOREIGN KEY(sound_file_id) REFERENCES sound_file(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
