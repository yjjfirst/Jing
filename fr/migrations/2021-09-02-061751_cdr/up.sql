-- Your SQL goes here
CREATE TABLE `cdr` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `a_caller_id` VARCHAR(32) NOT NULL,
  `a_dest` VARCHAR(32) NOT NULL,
  `start_time` DATETIME NOT NULL,
  `duration` INT NOT NULL,
  `b_caller_id` VARCHAR(32),
  `b_dest` VARCHAR(32),
  `uuid` VARCHAR(128),
  PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
