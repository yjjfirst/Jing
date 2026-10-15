-- Your SQL goes here
CREATE TABLE queue_param_helps (
    id SERIAL,
    name VARCHAR(128) NOT NULL,
    range_text VARCHAR(1024) NOT NULL,
    help_text VARCHAR(1024) NOT NULL,
    PRIMARY KEY(id)
);