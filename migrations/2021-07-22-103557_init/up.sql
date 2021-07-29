CREATE TABLE contact_us
(
    id         INTEGER PRIMARY KEY,
    name       VARCHAR(255)  NOT NULL,
    phone      VARCHAR(255)  NOT NULL,
    email      VARCHAR(255)  NOT NULL,
    message    VARCHAR(2000) NOT NULL,
    status     INTEGER DEFAULT 0,
    created_at TEXT    DEFAULT CURRENT_TIMESTAMP
);

insert into contact_us (id, name, phone, email, message, status)
values (1, 'test', 'test', 'test', 'test', 0);

CREATE TABLE chain_stat
(
    id                      INTEGER PRIMARY KEY,
    chain                   VARCHAR(20)                         NOT NULL,
    k                       NUMERIC   DEFAULT 1                 NOT NULL,
    c                       NUMERIC   DEFAULT 1                 NOT NULL,
    past_total_assets       NUMERIC   DEFAULT 0                 NOT NULL,
    past_total_rewards      NUMERIC   DEFAULT 0                 NOT NULL,
    past_total_running_time NUMERIC   DEFAULT 0                 NOT NULL,
    total_node_count        NUMERIC   DEFAULT 0                 NOT NULL,
    updated_at              TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    daily_reward            NUMERIC   DEFAULT 0                 NOT NULL,
    daily_running_time      NUMERIC   DEFAULT 0                 NOT NULL,
    market_price_usd        NUMERIC   DEFAULT 100               NOT NULL,
    market_price_time       TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);



insert into chain_stat (id, chain, k, c, past_total_assets, past_total_rewards, past_total_running_time,
                        total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd,
                        market_price_time)

values (1, 'ETH2', 1, 1, 25600, 1008, 4032000, 800, '2021-07-26 03:54:18', 0.006, 24, 2286.76,
        '2021-07-26 03:54:18');

insert into chain_stat (id, chain, k, c, past_total_assets, past_total_rewards, past_total_running_time,
                        total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd,
                        market_price_time)
values (2, 'EOS', 1, 1, 220000000, 800000, 24000, 1, '2021-07-26 03:54:18', 800, 24, 3.9,
        '2021-07-26 03:54:18');
insert into chain_stat (id, chain, k, c, past_total_assets, past_total_rewards, past_total_running_time,
                        total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd,
                        market_price_time)
values (3, 'POLKADOT', 1, 1, 1800000, 216000, 6480, 1, '2021-07-26 03:54:18', 800, 0, 14.84,
        '2021-07-26 03:54:18');
insert into chain_stat (id, chain, k, c, past_total_assets, past_total_rewards, past_total_running_time,
                        total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd,
                        market_price_time)
values (4, 'KUSAMA', 1, 1, 15000, 1980, 23760, 3, '2021-07-26 03:54:18', 2, 24, 184.57,
        '2021-07-26 03:54:18');
insert into chain_stat (id, chain, k, c, past_total_assets, past_total_rewards, past_total_running_time,
                        total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd,
                        market_price_time)
values (5, 'CARDANO', 1, 1, 960000000, 16500000, 39600, 15, '2021-07-26 03:54:18', 10000, 0, 1.27,
        '2021-07-26 03:54:18');
insert into chain_stat (id, chain, k, c, past_total_assets, past_total_rewards, past_total_running_time,
                        total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd,
                        market_price_time)
values (6, 'PLATON', 1, 1, 60000000, 5599440, 60480, 12, '2021-07-26 03:54:18', 2222, 24, 0.1514,
        '2021-07-26 03:54:18');
insert into chain_stat (id, chain, k, c, past_total_assets, past_total_rewards, past_total_running_time,
                        total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd,
                        market_price_time)
values (7, 'TERRA', 1, 1, 1800000, 74700, 10800, 1, '2021-07-26 03:54:18', 166, 0, 11.74,
        '2021-07-26 03:54:18');
insert into chain_stat (id, chain, k, c, past_total_assets, past_total_rewards, past_total_running_time,
                        total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd,
                        market_price_time)
values (8, 'DASH', 1, 1, 135000, 9990, 1296000, 135, '2021-07-26 03:54:18', 0.185, 24, 151.44,
        '2021-07-26 03:54:18');
insert into chain_stat (id, chain, k, c, past_total_assets, past_total_rewards, past_total_running_time,
                        total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd,
                        market_price_time)
values (9, 'IRISNET', 1, 1, 10000000, 9000000, 10800, 1, '2021-07-26 03:54:18', 20000, 24, 0.07158,
        '2021-07-26 03:54:18');

