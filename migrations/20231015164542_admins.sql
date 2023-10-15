-- Add migration script here
CREATE TABLE admins (
	id UUID PRIMARY KEY,
	account_id UUID NOT NULL REFERENCES accounts(id),

	email TEXT NOT NULL
);
