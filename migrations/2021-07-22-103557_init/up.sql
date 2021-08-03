CREATE TABLE contact_us
(
    id         INTEGER PRIMARY KEY,
    name       VARCHAR(255)  NOT NULL,
    phone      VARCHAR(255)  NOT NULL,
    email      VARCHAR(255)  NOT NULL,
    message    VARCHAR(2000) NOT NULL,
    status     INTEGER   DEFAULT 0 NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

insert into contact_us (id, name, phone, email, message, status, created_at)
values (1, 'test', 'test', 'test', 'test', 1, '2021-07-26 03:54:18');

CREATE TABLE chain_stat
(
    id                      INTEGER PRIMARY KEY,
    chain                   VARCHAR(20)                         NOT NULL,
    symbol                  VARCHAR(20)                         NOT NULL,
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


INSERT INTO chain_stat (id, chain, symbol, k, c, past_total_assets, past_total_rewards, past_total_running_time, total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd, market_price_time) VALUES (1, 'ETH2', 'ETH', 1, 1, 25600, 1008, 4032000, 800, '2021-07-26 03:54:18', 0.006, 24, 2485.4982803966773, '2021-08-03T08:25:26.000Z');
INSERT INTO chain_stat (id, chain, symbol, k, c, past_total_assets, past_total_rewards, past_total_running_time, total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd, market_price_time) VALUES (2, 'EOS', 'EOS', 1, 1, 220000000, 800000, 24000, 1, '2021-07-26 03:54:18', 800, 24, 3.9568158636678, '2021-08-03T08:22:10.000Z');
INSERT INTO chain_stat (id, chain, symbol, k, c, past_total_assets, past_total_rewards, past_total_running_time, total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd, market_price_time) VALUES (3, 'POLKADOT', 'DOT', 1, 1, 1800000, 216000, 6480, 1, '2021-07-26 03:54:18', 800, 0, 17.27507952670076, '2021-08-03T08:25:25.000Z');
INSERT INTO chain_stat (id, chain, symbol, k, c, past_total_assets, past_total_rewards, past_total_running_time, total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd, market_price_time) VALUES (4, 'KUSAMA', 'KSM', 1, 1, 15000, 1980, 23760, 3, '2021-07-26 03:54:18', 2, 24, 196.63099778740988, '2021-08-03T08:25:28.000Z');
INSERT INTO chain_stat (id, chain, symbol, k, c, past_total_assets, past_total_rewards, past_total_running_time, total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd, market_price_time) VALUES (5, 'CARDANO', 'ADA', 1, 1, 960000000, 16500000, 39600, 15, '2021-07-26 03:54:18', 10000, 0, 1.29858269635747, '2021-08-03T08:23:33.000Z');
INSERT INTO chain_stat (id, chain, symbol, k, c, past_total_assets, past_total_rewards, past_total_running_time, total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd, market_price_time) VALUES (6, 'PLATON', 'LAT', 1, 1, 60000000, 5599440, 60480, 12, '2021-07-26 03:54:18', 2222, 24, 0.19194875831305, '2021-08-03T08:23:29.000Z');
INSERT INTO chain_stat (id, chain, symbol, k, c, past_total_assets, past_total_rewards, past_total_running_time, total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd, market_price_time) VALUES (7, 'TERRA', 'LUNA', 1, 1, 1800000, 74700, 10800, 1, '2021-07-26 03:54:18', 166, 0, 12.33747964779181, '2021-08-03T08:22:09.000Z');
INSERT INTO chain_stat (id, chain, symbol, k, c, past_total_assets, past_total_rewards, past_total_running_time, total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd, market_price_time) VALUES (8, 'DASH', 'DASH', 1, 1, 135000, 9990, 1296000, 135, '2021-07-26 03:54:18', 0.185, 24, 155.5686005141396, '2021-08-03T08:22:19.000Z');
INSERT INTO chain_stat (id, chain, symbol, k, c, past_total_assets, past_total_rewards, past_total_running_time, total_node_count, updated_at, daily_reward, daily_running_time, market_price_usd, market_price_time) VALUES (9, 'IRISNET', 'IRIS', 1, 1, 10000000, 9000000, 10800, 1, '2021-07-26 03:54:18', 20000, 24, 0.07604591293186, '2021-08-03T08:23:32.000Z');
