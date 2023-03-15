-- Your SQL goes here
CREATE TABLE cdrs (
  id SERIAL,
  a_caller_id VARCHAR(32) NOT NULL,
  a_dest VARCHAR(32) NOT NULL,
  start_time timestamp NOT NULL,
  duration INT NOT NULL,
  b_caller_id VARCHAR(32),
  b_dest VARCHAR(32),
  uuid VARCHAR(128),
  PRIMARY KEY(id)
);
