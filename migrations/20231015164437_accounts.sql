-- Add migration script here
CREATE TABLE accounts (
	id UUID PRIMARY KEY,
	username TEXT NOT NULL,
	password TEXT NOT NULL,
	name TEXT NOT NULL,
	kind account_kind NOT NULL
);
