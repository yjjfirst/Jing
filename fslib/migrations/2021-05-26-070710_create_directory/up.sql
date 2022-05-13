-- Your SQL goes here
CREATE TABLE `extension` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `exten` VARCHAR(128) NOT NULL UNIQUE,
  `exten_type` VARCHAR(64),
  PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

CREATE TABLE `domain` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `domain_name` VARCHAR(128) NOT NULL,
  PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO `domain` (`domain_name`) VALUE ("$${domain}");

CREATE TABLE `user` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `domain_id` INT NOT NULL,
  `number_alias` VARCHAR(128),
  `mailbox` VARCHAR(128),
  `cidr` VARCHAR(128),
  `user_id` VARCHAR(128) NOT NULL,
  `password` VARCHAR(128) NOT NULL,
  `toll_allow` VARCHAR(128),
  `user_context` VARCHAR(128),
  `default_gateway` VARCHAR(128),
  `effective_caller_id_name` VARCHAR(128),
  `effective_caller_id_number` VARCHAR(128),
  `outbound_caller_id_name` VARCHAR(128),
  `outbound_caller_id_number` VARCHAR(128),
  `callgroup` VARCHAR(128),
  `uservar1` VARCHAR(128),
  `uservar2` VARCHAR(128),
  `uservar3` VARCHAR(128),
   PRIMARY KEY (id),
   UNIQUE INDEX ind (user_id),
   FOREIGN KEY(domain_id) REFERENCES domain(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
