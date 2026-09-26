-- Your SQL goes here
CREATE TABLE user_variable_helps (
    id SERIAL,
    name VARCHAR(128) NOT NULL,
    range_text VARCHAR(1024) NOT NULL,
    help_text VARCHAR(1024) NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO user_variable_helps(name, range_text, help_text) VALUES
('effective_caller_id_name','string','Caller ID name presented on internal calls originating from this user.'),
('effective_caller_id_number', 'string','Caller ID number presented on internal calls originating from this user.'),
('outbound_caller_id_name', 'string','Caller ID name used when this user places calls through an external gateway.'),
('Outoubnd Caller Id Number', 'string','Caller ID number used when this user places calls through an external gateway.');