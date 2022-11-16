
CREATE TABLE ivrs (
  id SERIAL,
  exten VARCHAR(32) NOT NULL,
  name VARCHAR(128) NOT NULL,
  domain_id INT NOT NULL,
  PRIMARY KEY(id),
  FOREIGN KEY(domain_id) REFERENCES domains(id)
);
