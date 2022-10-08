CREATE TABLE `ringing_group_members` (
      `id` INT NOT NULL AUTO_INCREMENT,
      `ringing_group_id` INT NOT NULL,
      `user_id` INT NOT NULL,
      PRIMARY KEY(id),
      FOREIGN KEY(ringing_group_id) REFERENCES ringing_groups(id),
      FOREIGN KEY(user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
