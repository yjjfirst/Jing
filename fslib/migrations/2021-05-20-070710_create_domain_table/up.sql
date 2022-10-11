CREATE TABLE `domains` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `domain_name` VARCHAR(128) NOT NULL,
  PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO `domains` (`domain_name`) VALUE ("$${domain}");
