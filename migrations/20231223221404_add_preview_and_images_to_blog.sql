-- Add migration script here
ALTER TABLE blogs ADD COLUMN preview TEXT NOT NULL DEFAULT '';
ALTER TABLE blogs ADD COLUMN images TEXT[] NOT NULL DEFAULT '{}';
ALTER TABLE blogs ADD COLUMN main_image TEXT;
