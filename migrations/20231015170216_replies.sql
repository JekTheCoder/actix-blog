-- Add migration script here
CREATE TABLE replies (
	id UUID PRIMARY KEY,
	comment_id UUID NOT NULL REFERENCES comments(id),
	account_id UUID NOT NULL REFERENCES accounts(id),
	parent_id UUID REFERENCES replies(id),

	content TEXT NOT NULL
);
