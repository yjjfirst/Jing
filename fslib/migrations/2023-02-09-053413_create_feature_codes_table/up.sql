CREATE TABLE feature_codes (
       id SERIAL,
       domain_id INT NOT NULL,
       digits VARCHAR(8) NOT NULL,
       action VARCHAR(128) NOT NULL,
       UNIQUE(digits),
       PRIMARY KEY(id),
       FOREIGN KEY(domain_id) REFERENCES domains(id)
);
