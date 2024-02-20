CREATE TABLE ringing_group_members (
      id SERIAL,
      ringing_group_id INT NOT NULL,
      user_id INT NOT NULL,
      PRIMARY KEY(id),
      FOREIGN KEY(ringing_group_id) REFERENCES ringing_groups(id) ON DELETE CASCADE,
      FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);
