CREATE TABLE `conference_profile_params` (
       `id` INT NOT NULL AUTO_INCREMENT,
       `conference_profile_id` INT NOT NULL,
       `name` VARCHAR(128) NOT NULL,
       `value` VARCHAR(128) NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(conference_profile_id) REFERENCES conference_profiles(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "domain", "$${domain}");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "rate", "8000");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "interval", "20");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "energy-level", "100");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "muted-sound", "conference/conf-muted.wav");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "unmuted-sound", "conference/conf-unmuted.wav");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "alone-sound", "conference/conf-alone.wav");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "moh-sound", "$${hold_music}");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "enter-sound", "tone_stream://%(200,0,500,600,700)");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "exit-sound", "tone_stream://%(500,0,300,200,100,50,25)");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "kicked-sound", "conference/conf-kicked.wav");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "locked-sound", "conference/conf-locked.wav");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "is-locked-sound", "conference/conf-is-locked.wav");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "is-unlocked-sound", "conference/conf-is-unlocked.wav");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "pin-sound", "conference/conf-pin.wav");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "bad-pin-sound", "conference/conf-bad-pin.wav");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "caller-id-name", "$${outbound_caller_name}");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "caller-id-number", "$${outbound_caller_id}");

INSERT INTO conference_profile_params (`conference_profile_id`, `name`, `value`)
       VALUES (1, "comfort-noise", "true");
