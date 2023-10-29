-- Add migration script here

CREATE TABLE sub_categories (
	id          UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	name        TEXT NOT NULL,
	category_id	UUID NOT NULL REFERENCES categories(id)
);
