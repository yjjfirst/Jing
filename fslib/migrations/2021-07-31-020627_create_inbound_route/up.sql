-- Your SQL goes here
CREATE TABLE `inbound_route` (
`id` INT NOT NULL AUTO_INCREMENT,
`context` VARCHAR(64) NOT NULL,
`condition` VARCHAR(512) NOT NULL,
`dest_extension` VARCHAR(64) NOT NULL,
PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
