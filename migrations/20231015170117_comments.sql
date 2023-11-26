-- Add migration script here
CREATE TABLE comments (
	id UUID PRIMARY KEY,
	blog_id UUID NOT NULL REFERENCES blogs(id) ON DELETE CASCADE,
	account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,

	content TEXT NOT NULL
);
