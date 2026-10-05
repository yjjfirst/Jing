-- Your SQL goes here
CREATE TABLE ivr_attr_helps (
    id SERIAL,
    name VARCHAR(128) NOT NULL,
    range_text VARCHAR(1024) NOT NULL,
    help_text VARCHAR(1024) NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO ivr_attr_helps(name, range_text, help_text) VALUES
('greet-long','sound','Audio or TTS played on the first entry into the menu.'),
('greet-short', 'sound','Audio or TTS played on re-entry (after returning from a submenu or looping).'),
('invalid-sound', 'sound','Audio or TTS played when the caller presses digits that do not match any entry.'),
('exit-sound','sound','Audio or TTS played when the menu exits normally (max failures, max timeouts, or menu-exit).'),
('timeout','digits','Milliseconds to wait for the first digit before incrementing the timeout counter.'),
('inter-digit-timeout','digits','Milliseconds to wait between digits after the first digit has been received.'),
('max-failures','digits','Number of invalid-digit events tolerated before the menu exits.'),
('max-timeouts','digits','Number of timeout events tolerated before the menu exits. If omitted, inherits the value of max-failures.'),
('confirm-attempts', 'digits', 'Number of confirmation attempts allowed.'),
('digit-len', 'digits', 'Maximum number of digits to collect before matching. When set to 0, the engine automatically grows digit-len to the length of the longest literal digits value bound in the menu''s entries.');