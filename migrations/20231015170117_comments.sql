-- Add migration script here
CREATE TABLE comments (
	id UUID PRIMARY KEY,
	blog_id UUID NOT NULL REFERENCES blogs(id),
	account_id UUID NOT NULL REFERENCES accounts(id),

	content TEXT NOT NULL
);
