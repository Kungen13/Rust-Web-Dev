-- Add down migration script here
--command: sqlx migrate revert
DROP TABLE IF EXISTS questions;