-- Your SQL goes here
CREATE TABLE user_param_helps (
    id SERIAL,
    name VARCHAR(128) NOT NULL,
    range_text VARCHAR(1024) NOT NULL,
    help_text VARCHAR(1024) NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO user_param_helps(name, range_text, help_text) VALUES
('password','string','SIP digest authentication password. Used by mod_sofia to verify the user''s credentials via MD5 digest.'),
('vm-password', 'string','PIN used to access the voicemail mailbox for this user. The special value user-choose allows the caller to set their own PIN on first access. If omitted, the password param is used as the PIN.');
