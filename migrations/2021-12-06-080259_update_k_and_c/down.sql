-- This file should undo anything in `up.sql`
update chain_stat set k = 1, c = 1 where id > 0;