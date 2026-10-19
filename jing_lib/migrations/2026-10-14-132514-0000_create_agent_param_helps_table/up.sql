-- Your SQL goes here
CREATE TABLE agent_param_helps (
    id SERIAL,
    name VARCHAR(128) NOT NULL,
    range_text VARCHAR(1024) NOT NULL,
    help_text VARCHAR(1024) NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO agent_param_helps (name, range_text, help_text) VALUES
('max-no-answer', 'digits', 'Number of consecutive unanswered calls before the agent''s status is changed to agent-no-answer-status. 0 disables the limit.'),
('wrap-up-time', 'digits', 'Seconds the agent is unavailable after a call ends before returning to Available.'),
('reject-delay-time','digits', 'Seconds to wait before offering another call to an agent who rejected the previous one.'),
('busy-delay-time','digits', 'Seconds to wait before offering another call to an agent whose line returned busy.'),
('type','enum(callback)','How the module reaches the agent. Callback originates an outbound call to contact; uuid-standby bridges to an existing channel UUID specified in contact.'),
('status','enum(Available,Available/On Demand,On Break,Logged Out)','Initial status applied to the agent when the XML configuration is loaded. Status controls whether the agent is offered calls.');