-- Add migration script here
CREATE TABLE replies (
	id UUID PRIMARY KEY,
	comment_id UUID NOT NULL REFERENCES comments(id) ON DELETE CASCADE,
	account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
	parent_id UUID REFERENCES replies(id) ON DELETE CASCADE,

	content TEXT NOT NULL
);
