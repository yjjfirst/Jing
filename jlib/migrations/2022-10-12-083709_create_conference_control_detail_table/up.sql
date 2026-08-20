CREATE TABLE conference_control_details (
       id SERIAL,
       conference_control_id INT NOT NULL,
       action VARCHAR(32) NOT NULL,
       digits VARCHAR(8) NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(conference_control_id) REFERENCES conference_controls(id)
);

INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'mute', '0');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'deaf mute', '*');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'energy up', '9');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'energy equ', '8');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'energy dn', '7');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'vol talk up', '3');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'vol talk zero', '2');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'vol talk dn', '1');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'vol listen up', '6');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'vol listen zero', '5');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'vol listen dn', '4');
INSERT INTO conference_control_details (conference_control_id, action, digits) VALUES (1, 'hangup', '#');
