-- Your SQL goes here2
CREATE TABLE users (
  id SERIAL,
  domain_id INT NOT NULL,
  user_id VARCHAR(128) NOT NULL,
  password VARCHAR(128) NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY(domain_id) REFERENCES domains(id)
);
