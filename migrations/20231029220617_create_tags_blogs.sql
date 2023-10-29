-- Add migration script here

CREATE TABLE tags_blogs (
	id							UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	blog_id					UUID REFERENCES blogs(id) NOT NULL,
	tag_id					UUID REFERENCES tags(id) NOT NULL
);
