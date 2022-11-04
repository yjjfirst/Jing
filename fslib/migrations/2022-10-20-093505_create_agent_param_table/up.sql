CREATE TABLE `agent_params` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `agent_id` INT NOT NULL,
       `name` VARCHAR(128) NOT NULL,
       `value` VARCHAR(128) NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(agent_id) REFERENCES agents(id) ON DELETE CASCADE
)ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
