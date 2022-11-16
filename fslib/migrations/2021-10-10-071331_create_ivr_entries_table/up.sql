-- Your SQL goes here
CREATE TABLE ivr_entries (
  id SERIAL,
  ivr_id INT,
  digits VARCHAR(8) NOT NULL,
  dest_type VARCHAR(64) NOT NULL,
  dest_exten VARCHAR(64) NOT NULL,
  PRIMARY KEY(id),
  FOREIGN KEY(ivr_id) REFERENCES ivrs(id)
);
