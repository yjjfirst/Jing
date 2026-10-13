-- Your SQL goes here
CREATE TABLE profile_params (
    id SERIAL,
    profile_id INT NOT NULL,
    name VARCHAR(256) NOT NULL,
    value VARCHAR(256) NOT NULL,
    ignored BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id),
    FOREIGN KEY (profile_id) REFERENCES profiles(id)
);

INSERT INTO profile_params (name, value, profile_id, ignored) VALUES
('debug', '0', 1, FALSE),
('context', 'internal', 1, FALSE),
('sip-port', '$${internal_sip_port}', 1, FALSE),
('dialplan', 'XML', 1, FALSE),
('codec-prefs', '$${global_codec_prefs}', 1, FALSE),
('rtp-ip', '$${local_ip_v4}', 1, FALSE),
('sip-ip', '$${local_ip_v4}', 1, FALSE),
('apply-inbound-acl', 'domains', 1, TRUE),
('auth-calls', '$${internal_auth_calls}', 1, FALSE),
('ext-rtp-ip', '$${local_ip_v4}', 1, FALSE),
('ext-sip-ip', '$${local_ip_v4}', 1, FALSE),
('apply-nat-acl', 'nat.auto', 1, FALSE),
('ws-binding', ':5066', 1, FALSE),
('wss-binding', ':7443', 1, FALSE);

INSERT INTO profile_params (name, value, profile_id, ignored) VALUES
('auth-calls', 'false', 2, FALSE),
('apply-inbound-acl', 'providers', 2, FALSE),
('codec-prefs', '$${global_codec_prefs}', 2, FALSE),
('context', 'public', 2, FALSE),
('debug', '0', 2, FALSE),
('dialplan', 'XML', 2, FALSE),
('ext-rtp-ip', 'auto-nat', 2, FALSE),
('ext-sip-ip', 'auto-nat', 2, FALSE),
('rtp-ip', '$${local_ip_v4}', 2, FALSE),
('sip-ip', '$${local_ip_v4}', 2, FALSE),
('sip-port', '$${external_sip_port}', 2, FALSE),
('apply-nat-acl', 'nat.auto', 2, TRUE),
('ws-binding', ':5066', 2, TRUE),
('wss-binding', ':7443', 2, TRUE);
