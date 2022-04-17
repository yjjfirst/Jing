-- Your SQL goes here
CREATE TABLE `inbound_route` (
`id` INT NOT NULL AUTO_INCREMENT,
`context` VARCHAR(64) NOT NULL,
`condition` VARCHAR(512) NOT NULL,
`dest_type` VARCHAR(64) NOT NULL,
`dest` INT NOT NULL,
PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

CREATE TABLE `dest_type` (
`id` INT NOT NULL AUTO_INCREMENT,
`dest_name` VARCHAR(64) NOT NULL,
PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO `dest_type` (`dest_name`) VALUE ("user");
INSERT INTO `dest_type` (`dest_name`) VALUE ("ringgroup");
INSERT INTO `dest_type` (`dest_name`) VALUE ("ivr");
INSERT INTO `dest_type` (`dest_name`) VALUE ("conference");
INSERT INTO `dest_type` (`dest_name`) VALUE ("queue");
INSERT INTO `dest_type` (`dest_name`) VALUE ("announcement");
