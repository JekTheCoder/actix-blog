-- Add migration script here
CREATE TABLE blogs (
	id UUID PRIMARY KEY,
	admin_id UUID NOT NULL REFERENCES admins(id),
	title TEXT NOT NULL,
	content TEXT NOT NULL
);
