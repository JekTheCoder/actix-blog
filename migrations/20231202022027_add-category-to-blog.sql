-- Add migration script here
ALTER TABLE blogs ADD COLUMN category_id UUID REFERENCES categories(id);
