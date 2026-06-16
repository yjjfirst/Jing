CREATE TABLE acl_nodes (
       id SERIAL,
       list_id int,
       node_type VARCHAR(128) NOT NULL,
       cidr VARCHAR(128) NOT NULL,
       PRIMARY KEY(id),
       FOREIGN KEY(list_id) REFERENCES acl_lists(id) ON DELETE CASCADE
);
