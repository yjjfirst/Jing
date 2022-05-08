CREATE TABLE `ringing_group` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `name` VARCHAR(32) NOT NULL,
       `group_id` VARCHAR(32) NOT NULL UNIQUE,
       `domain_id` INT NOT NULL,
       `description` VARCHAR(256),
       `ring_time` INT NOT NULL DEFAULT 20, 
       `ring_strategy` VARCHAR(32) NOT NULL DEFAULT 'all',
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domain(id)
       ) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

CREATE TABLE `ringing_group_member` (
      `id` INT NOT NULL AUTO_INCREMENT,
      `ringing_group_id` INT NOT NULL,
      `user_id` INT NOT NULL,
      PRIMARY KEY(id),
      FOREIGN KEY(ringing_group_id) REFERENCES ringing_group(id),
      FOREIGN KEY(user_id) REFERENCES user(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
       
            
