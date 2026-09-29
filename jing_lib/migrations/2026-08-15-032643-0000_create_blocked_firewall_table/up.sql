-- Your SQL goes here
CREATE TABLE firewall_rules (
    id SERIAL PRIMARY KEY,
    ip_address VARCHAR(45) NOT NULL UNIQUE,
    action VARCHAR(16) NOT NULL DEFAULT 'deny',
    created_at timestamp with time zone not null
);
