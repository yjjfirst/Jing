CREATE TABLE `conference_controls` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `name` VARCHAR(128) NOT NULL,
       `description` VARCHAR(512),
       PRIMARY KEY(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO conference_controls (`name`, `description`) VALUES ("default", "Default control group");
