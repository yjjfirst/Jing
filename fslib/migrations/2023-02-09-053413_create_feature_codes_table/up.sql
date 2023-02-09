CREATE TABLE feature_codes (
       id SERIAL,
       digits VARCHAR(8) NOT NULL,
       action VARCHAR(128) NOT NULL,
       UNIQUE(digits),
       PRIMARY KEY(id)
);
