-- Your SQL goes here
CREATE TABLE `gateways` (
`id` INT NOT NULL AUTO_INCREMENT,
`profile_id` INT NOT NULL,
`gateway_name` VARCHAR(256) NOT NULL,
`proxy` VARCHAR(256) NOT NULL,
`register` VARCHAR(256) NOT NULL,
`username` VARCHAR(256),
`password` VARCHAR(256),
PRIMARY KEY(id),
FOREIGN KEY(profile_id) REFERENCES profiles(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
