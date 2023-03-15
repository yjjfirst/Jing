-- Your SQL goes here
CREATE TABLE outbound_routes (
id SERIAL,
gateway_id INT NOT NULL,
priority INT NOT NULL,
condition VARCHAR(512) NOT NULL,
PRIMARY KEY(id),
FOREIGN KEY(gateway_id) REFERENCES gateways(id)
);
