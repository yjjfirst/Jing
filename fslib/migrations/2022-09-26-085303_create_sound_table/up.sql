CREATE TABLE `sound` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `name` VARCHAR(32) NOT NULL UNIQUE,
       `path` VARCHAR(256) NOT NULL,
       `domain_id` INT NOT NULL,
       `description` VARCHAR(1024),
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domain(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
