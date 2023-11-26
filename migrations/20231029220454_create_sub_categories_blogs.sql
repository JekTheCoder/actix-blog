-- Add migration script here

CREATE TABLE sub_categories_blogs (
	id							UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	blog_id					UUID NOT NULL REFERENCES blogs(id) ON DELETE CASCADE,
	sub_category_id UUID NOT NULL REFERENCES sub_categories(id) ON DELETE CASCADE
);
