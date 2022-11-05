CREATE TABLE queue_params (
       id SERIAL,
       queue_id INT NOT NULL,
       name VARCHAR(128) NOT NULL,
       value VARCHAR(128) NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(queue_id) REFERENCES queues(id) ON DELETE CASCADE
);
