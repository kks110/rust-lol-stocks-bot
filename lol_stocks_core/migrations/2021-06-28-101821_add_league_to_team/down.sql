-- This file should undo anything in `up.sql`
ALTER TABLE teams DROP COLUMN league_id;
DROP TABLE leagues;