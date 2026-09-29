CREATE TABLE extensions (
  id SERIAL PRIMARY KEY,
  exten VARCHAR(128) NOT NULL,
  exten_type VARCHAR(64) NOT NULL,
  domain_id INT NOT NULL,
  UNIQUE(exten, domain_id)
);
