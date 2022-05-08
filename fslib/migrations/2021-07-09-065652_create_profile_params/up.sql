-- Your SQL goes here
CREATE TABLE `profile` (
`id` INT NOT NULL AUTO_INCREMENT,
`name` VARCHAR(256) NOT NULL,
PRIMARY KEY (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO `profile` (`name`) VALUE ("internal");
INSERT INTO `profile` (`name`) VALUE ("external");

CREATE TABLE `profile_param` (
`id` INT NOT NULL AUTO_INCREMENT,
`profile_id` INT NOT NULL,
`name` VARCHAR(256) NOT NULL,
`value` VARCHAR(256) NOT NULL,
PRIMARY KEY (id),
FOREIGN KEY (profile_id) REFERENCES profile(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("debug", "0", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("sip-trace", "no", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("context", "internal", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("sip-port", "$${internal_sip_port}", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("dialplan", "XML", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("inbound-codec-prefs", "$${global_codec_prefs}", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("outbound-codec-prefs", "$${global_codec_prefs}", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("rtp-ip", "$${local_ip_v4}", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("sip-ip", "$${local_ip_v4}", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("hold-music", "$${hold_music}", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("apply-nat-acl", "nat.auto", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("apply-inbound-acl", "domains", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("local-network-acl", "localnet.auto", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("auth-calls", "$${internal_auth_calls}", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("ext-rtp-ip", "$${local_ip_v4}", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("ext-sip-ip", "$${local_ip_v4}", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("ws-binding", "5066", 1);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("wss-binding", "7443", 1);

INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("auth-calls", "false", 2);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("debug", "0", 2);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("dialplan", "XML", 2);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("context", "public", 2);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("codec-prefs", "$${global_codec_prefs}", 2);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("rtp-ip", "$${local_ip_v4}", 2);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("sip-ip", "$${local_ip_v4}", 2);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("ext-rtp-ip", "auto-nat", 2);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("ext-sip-ip", "auto-nat", 2);
INSERT INTO `profile_param` (`name`, `value`, `profile_id`) VALUE ("sip-port", "$${external_sip_port}", 2);

CREATE TABLE `gateway` (
`id` INT NOT NULL AUTO_INCREMENT,
`profile_id` INT NOT NULL,
`gateway_name` VARCHAR(256) NOT NULL,
`proxy` VARCHAR(256) NOT NULL,
`register` VARCHAR(256) NOT NULL,
`username` VARCHAR(256),
`password` VARCHAR(256),
PRIMARY KEY(id),
FOREIGN KEY(profile_id) REFERENCES profile(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
