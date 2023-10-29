-- Add migration script here

CREATE TABLE categories (
	id          UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	name        TEXT NOT NULL
);
