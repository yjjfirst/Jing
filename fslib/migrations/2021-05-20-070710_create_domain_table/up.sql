CREATE TABLE `domains` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `domain_name` VARCHAR(128) NOT NULL,
  `active` BOOLEAN NOT NULL DEFAULT false,
  PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO `domains` (`domain_name`, `active`) VALUE ("$${domain}", 1);
