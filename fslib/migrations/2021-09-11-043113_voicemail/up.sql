-- Your SQL goes here
CREATE TABLE `voicemail` (
`id` INT NOT NULL AUTO_INCREMENT,
`user_id` INT NOT NULL UNIQUE,
`password` VARCHAR(32) NOT NULL,
`email` VARCHAR(128),
PRIMARY KEY(id),
FOREIGN KEY(user_id) REFERENCES user(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
