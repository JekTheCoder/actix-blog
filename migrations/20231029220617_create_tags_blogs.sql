-- Add migration script here

CREATE TABLE tags_blogs (
	id							UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	blog_id					UUID NOT NULL REFERENCES blogs(id) ON DELETE CASCADE,
	tag_id					UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE
);
