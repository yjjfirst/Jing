-- Your SQL goes here
CREATE TYPE firewall_action AS ENUM ('allow', 'deny');

CREATE TABLE firewall (
    id SERIAL PRIMARY KEY,
    ip_address VARCHAR(45) NOT NULL,
    action firewall_action NOT NULL DEFAULT 'deny',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
