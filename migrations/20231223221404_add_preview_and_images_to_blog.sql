-- Add migration script here
ALTER TABLE blogs ADD COLUMN preview TEXT;
ALTER TABLE blogs ADD COLUMN images TEXT[];
