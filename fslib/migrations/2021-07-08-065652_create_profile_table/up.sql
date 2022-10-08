-- Your SQL goes here
CREATE TABLE `profiles` (
`id` INT NOT NULL AUTO_INCREMENT,
`name` VARCHAR(256) NOT NULL,
PRIMARY KEY (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO `profiles` (`name`) VALUE ("internal");
INSERT INTO `profiles` (`name`) VALUE ("external");
