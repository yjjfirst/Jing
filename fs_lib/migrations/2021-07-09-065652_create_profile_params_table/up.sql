-- Your SQL goes here
CREATE TABLE profile_params (
id SERIAL,
profile_id INT NOT NULL,
name VARCHAR(256) NOT NULL,
value VARCHAR(256) NOT NULL,
PRIMARY KEY (id),
FOREIGN KEY (profile_id) REFERENCES profiles(id)
);

INSERT INTO profile_params (name, value, profile_id) VALUES ('debug', '0', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('sip-trace', 'no', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('context', 'internal', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('sip-port', '$${internal_sip_port}', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('dialplan', 'XML', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('inbound-codec-prefs', '$${global_codec_prefs}', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('outbound-codec-prefs', '$${global_codec_prefs}', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('rtp-ip', '$${local_ip_v4}', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('sip-ip', '$${local_ip_v4}', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('hold-music', '$${hold_music}', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('apply-nat-acl', 'nat.auto', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('apply-inbound-acl', 'domains', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('local-network-acl', 'localnet.auto', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('auth-calls', '$${internal_auth_calls}', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('ext-rtp-ip', '$${local_ip_v4}', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('ext-sip-ip', '$${local_ip_v4}', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('ws-binding', '5066', 1);
INSERT INTO profile_params (name, value, profile_id) VALUES ('wss-binding', '7443', 1);

INSERT INTO profile_params (name, value, profile_id) VALUES ('auth-calls', 'false', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('debug', '0', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('dialplan', 'XML', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('context', 'public', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('codec-prefs', '$${global_codec_prefs}', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('rtp-ip', '$${local_ip_v4}', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('sip-ip', '$${local_ip_v4}', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('ext-rtp-ip', 'auto-nat', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('ext-sip-ip', 'auto-nat', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('sip-port', '$${external_sip_port}', 2);
INSERT INTO profile_params (name, value, profile_id) VALUES ('apply-inbound-acl', 'providers', 2);
