-- Add migration script here

CREATE TABLE sub_categories_blogs (
	id							UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	blog_id					UUID REFERENCES blogs(id) NOT NULL,
	sub_category_id UUID REFERENCES sub_categories(id) NOT NULL
);
