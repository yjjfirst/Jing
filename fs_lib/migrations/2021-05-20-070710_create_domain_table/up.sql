CREATE TABLE domains (
  id SERIAL,
  domain_name VARCHAR(128) NOT NULL,
  PRIMARY KEY(id)
);

INSERT INTO domains (domain_name) VALUES ('$${domain}');
