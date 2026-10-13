-- Your SQL goes here
CREATE TABLE profile_param_helps (
    id SERIAL,
    name VARCHAR(128) NOT NULL,
    range_text VARCHAR(1024) NOT NULL,
    help_text VARCHAR(1024) NOT NULL,
    readonly BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(id)
);

INSERT INTO profile_param_helps(name, readonly, range_text, help_text) VALUES
('context', FALSE, 'string', 'The dialplan context inbound calls are delivered to. Can be overridden per-call by the user_context channel variable or ACL pass/fail contexts.'),
('sip-ip', FALSE, 'domain', 'Local IP address for the SIP transport socket. Use only IP addresses, not hostnames. Accepts auto (use the system-guessed IP), interface:NAME (resolve from a named network interface), or an explicit dotted-decimal address.'),
('rtp-ip', FALSE, 'domain', 'Local IP address used as the RTP source/destination in SDP. Accepts the same forms as sip-ip. Multiple rtp-ip entries are supported and the profile will select an appropriate address per call.'),
('ext-sip-ip', FALSE, 'domain', 'Public IP address advertised in SIP Contact and Via headers for the SIP transport. Accepts the same forms as ext-rtp-ip.'),
('ext-rtp-ip', FALSE, 'domain', 'Public IP address advertised in SDP for RTP. Used when the FreeSWITCH host is behind NAT. Accepts a literal IP, auto (system-guessed external IP), auto-nat (IP learned via NAT-PMP or UPnP), stun:host (STUN query), or host:hostname (DNS lookup).'),
('sip-port',FALSE, 'string','UDP/TCP port the SIP transport listens on. Accepts auto to let the OS assign a port.'),
('dialplan', TRUE, 'string','The dialplan module used to process inbound calls.'),
('debug', TRUE, 'digits', 'Sofia module debug level for this profile.'),
('auth-calls', FALSE, 'bool','Require digest authentication on inbound INVITE requests.'),
('apply-inbound-acl', FALSE, 'string','acl	ACL name applied to inbound call source IPs. Calls passing the ACL may be delivered to an optional context specified with acl-name:pass-context:fail-context syntax. Multiple instances of this parameter are supported. Use none to clear.'),
('codec-prefs', FALSE, 'string','Comma-separated codec names, e.g. OPUS,G722,PCMU,PCMA'),
('apply-nat-acl', TRUE, 'string','Checked against the source IP to detect NAT'),
('ws-binding', FALSE, 'string','IP address and port for the WebSocket (WS) listener. Format is [ip]:port. Omitting the IP binds all interfaces.'),
('wss-binding', FALSE, 'string','IP address and port for the secure WebSocket (WSS) listener. Requires a wss.pem certificate in $${certs_dir}, which is auto-generated if absent. When tls-cert-dir is set, FreeSWITCH also looks for wss.key and wss.crt in that directory.');