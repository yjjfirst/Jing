CREATE TABLE `conference_profiles` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `name` VARCHAR(128) NOT NULL,
       `description` VARCHAR(512),
       PRIMARY KEY(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO conference_profiles (`name`, `description`) VALUES("default", "Default profile");
