CREATE TABLE acl_lists (
  id SERIAL,
  acl_name VARCHAR(128) NOT NULL,
  acl_default VARCHAR(128) NOT NULL,
  PRIMARY KEY(id)
);
