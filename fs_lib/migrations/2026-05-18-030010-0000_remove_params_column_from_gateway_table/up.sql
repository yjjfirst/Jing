-- Your SQL goes here
ALTER TABLE gateways
DROP COLUMN proxy,
DROP COLUMN register,
DROP COLUMN username,
DROP COLUMN password;
