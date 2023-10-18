-- Add migration script here
ALTER TABLE blogs ADD COLUMN html TEXT NOT NULL DEFAULT '';
