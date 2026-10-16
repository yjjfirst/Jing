-- Your SQL goes here
CREATE TABLE queue_param_helps (
    id SERIAL,
    name VARCHAR(128) NOT NULL,
    range_text VARCHAR(1024) NOT NULL,
    help_text VARCHAR(1024) NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO queue_param_helps (name, range_text, help_text) VALUES
('strategy','enum(longest-idle-agent,round-robin,top-down,agent-with-least-talk-time,agent-with-fewest-calls,sequentially-by-agent-order,ring-all,ring-progressively)','Agent selection algorithm.'),
('moh-sound','string','Hold music played to waiting callers.'),
('time-base-score','enum(system,queue)','Reference point for calculating a caller''s wait-time score used in agent selection. queue counts from when the caller entered the queue; system counts from call origination.'),
('max-wait-time','digits','Maximum seconds a caller waits before being abandoned. 0 means unlimited.'),
('max-wait-time-with-no-agent','digits','Maximum seconds a caller waits when no agents are logged in. 0 means unlimited.'),
('max-wait-time-with-no-agent-time-reached','digits','Grace period in seconds after max-wait-time-with-no-agent is reached before the caller is removed.'),
('tier-rules-apply','bool','Enable tier-based wait-time rules (see below).'),
('tier-rule-wait-second','digits','Seconds a caller must wait before advancing to the next tier level.'),
('tier-rule-wait-multiply-level','bool','When true, the wait time to reach tier level N is N * tier-rule-wait-second. When false, all higher tiers become reachable after a single tier-rule-wait-second interval.'),
('tier-rule-no-agent-no-wait','bool','When true, callers skip tier wait times if no agents are available in the current tier.'),
('discard-abandoned-after','digits', 'Seconds after which an abandoned caller''s record is removed from the database, preventing resume. 0 keeps it indefinitely.'),
('abandoned-resume-allowed','bool','When true, a caller who hung up and calls back within discard-abandoned-after seconds resumes their original position in the queue.');