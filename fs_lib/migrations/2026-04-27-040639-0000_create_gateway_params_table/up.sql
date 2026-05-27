-- Your SQL goes here
CREATE TABLE gateway_params (
	       id SERIAL,
	       gateway_id INT NOT NULL,
	       name VARCHAR(128) NOT NULL,
	       value VARCHAR(128) NOT NULL,
	       UNIQUE(gateway_id, name),
	       PRIMARY KEY(id),
	       FOREIGN KEY(gateway_id) REFERENCES gateways(id) ON DELETE CASCADE
);
