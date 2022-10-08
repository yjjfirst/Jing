CREATE TABLE `extensions` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `exten` VARCHAR(128) NOT NULL,
  `exten_type` VARCHAR(64) NOT NULL,
  `domain_id` INT NOT NULL,
  UNIQUE(exten, domain_id),
  PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
