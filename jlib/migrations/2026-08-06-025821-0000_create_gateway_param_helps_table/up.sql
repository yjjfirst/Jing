-- Your SQL goes here
CREATE TABLE gateway_param_helps (
           id SERIAL,
           name VARCHAR(128) NOT NULL,
           range_text VARCHAR(1024) NOT NULL,
           help_text VARCHAR(1024) NOT NULL,
           PRIMARY KEY(id)
);

INSERT INTO gateway_param_helps (name, range_text, help_text) VALUES
('username', 'string', 'Username for gateway authentication.'),
('password', 'string', 'Password for gateway authentication.'),
('register', 'bool', 'Enables or disables registration.'),
('proxy', 'domain', 'Proxy IP address'),
('from-domain', 'domain', 'From domain'),
('realm', 'domain', 'Realms for gateway authentication.'),
('caller-id-in-from', 'bool', 'Enables or disables caller ID in from header.');