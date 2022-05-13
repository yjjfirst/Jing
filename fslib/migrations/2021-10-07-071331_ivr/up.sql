-- Your SQL goes here
CREATE TABLE `ivr` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `exten` VARCHAR(32) NOT NULL,
  `name` VARCHAR(128) NOT NULL,
  `domain_id` INT NOT NULL,
  `greet_long` VARCHAR(256),
  `greet_short` VARCHAR(256),
  `invalid_sound` VARCHAR(256),
  `exit_sound` VARCHAR(256),
  `confirm_attempts` INT DEFAULT 3,
  `timeout` INT DEFAULT 10000,
  `inter_digit_timeout` INT DEFAULT 2000,
  `max_failures` INT DEFAULT 3,
  `max_timeouts` INT DEFAULT 3,
  `digit_len` INT DEFAULT 4,
  PRIMARY KEY(id),
  FOREIGN KEY(domain_id) REFERENCES domain(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

CREATE TABLE `ivr_option` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `ivr_id` INT,
  `digits` VARCHAR(8) NOT NULL,
  `dest_type` INT,
  `dest_id` INT,
  PRIMARY KEY(id),
  FOREIGN KEY(ivr_id) REFERENCES ivr(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
