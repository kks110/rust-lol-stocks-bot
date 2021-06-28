-- This file should undo anything in `up.sql`
ALTER TABLE teams ALTER COLUMN league_id DROP NOT NULL;