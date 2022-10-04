CREATE TABLE `extension_type` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `name` VARCHAR(32) NOT NULL UNIQUE,
       PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO extension_type (name) VALUES ('user');
INSERT INTO extension_type (name) VALUES ('ringgroup');
INSERT INTO extension_type (name) VALUES ('ivr');
INSERT INTO extension_type (name) VALUES ('sound');
