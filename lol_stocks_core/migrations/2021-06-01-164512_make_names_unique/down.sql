-- This file should undo anything in `up.sql`
ALTER TABLE users DROP CONSTRAINT unique_user_name;
ALTER TABLE teams DROP CONSTRAINT unique_team_name;