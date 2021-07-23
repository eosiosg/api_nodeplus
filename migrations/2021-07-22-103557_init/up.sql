CREATE TABLE contact_us
(
    id         INTEGER PRIMARY KEY,
    name       VARCHAR(255)  NOT NULL,
    email      VARCHAR(255)  NOT NULL,
    message    VARCHAR(2000) NOT NULL,
    status     INTEGER DEFAULT 0,
    created_at TEXT    DEFAULT CURRENT_TIMESTAMP
);

insert into contact_us (id, name, email, message, status)
values (1, 'test', 'test', 'test', 0);

CREATE TABLE stats
(
    id                 INTEGER PRIMARY KEY,
    total_assets       INTEGER DEFAULT 0 NOT NULL,
    total_rewards      INTEGER DEFAULT 0 NOT NULL,
    total_running_time INTEGER DEFAULT 0 NOT NULL,
    total_node_count   INTEGER DEFAULT 0 NOT NULL,
    updated_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP NOT NULL
);

insert into stats (id, total_assets, total_rewards, total_running_time, total_node_count)
values (1, 5244721382, 891602634, 22776071, 3752);
