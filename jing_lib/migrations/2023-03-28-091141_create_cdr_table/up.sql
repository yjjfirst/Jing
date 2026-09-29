-- Your SQL goes here
create table cdr (
	    id                        serial primary key,
	    caller_id_name            varchar,
	    caller_id_number          varchar,
	    destination_number        varchar not null,
	    start_stamp               timestamp with time zone not null,
	    answer_stamp              timestamp with time zone,
	    end_stamp                 timestamp with time zone not null,
	    duration                  int not null,
	    billsec                   int not null,
	    hangup_cause              varchar not null
);
