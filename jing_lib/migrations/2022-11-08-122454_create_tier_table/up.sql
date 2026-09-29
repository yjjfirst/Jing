CREATE TABLE tiers (
       id SERIAL,
       agent_id INT NOT NULL,
       queue_id INT NOT NULL,
       level INT NOT NULL,
       position INT NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(agent_id) REFERENCES agents(id),
       FOREIGN KEY(queue_id) REFERENCES queues(id)
);
