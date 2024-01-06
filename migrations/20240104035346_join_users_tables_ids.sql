BEGIN;
SET session_replication_role = 'replica';
	CREATE TABLE tmp_blogs (
		id UUID PRIMARY KEY,
		admin_id UUID NOT NULL,
		title TEXT NOT NULL,
		content TEXT NOT NULL,
		html TEXT NOT NULL,
		category_id UUID NOT NULL,
		preview TEXT NOT NULL,
		images TEXT[] NOT NULL,
		main_image TEXT
	);

	INSERT INTO tmp_blogs
	(
		id,
		admin_id,
		title,
		content,
		html,
		category_id,
		preview,
		images,
		main_image
	) SELECT
		b.id,
		a.account_id as admin_id,
		b.title,
		b.content,
		b.html,
		b.category_id,
		b.preview,
		b.images,
		b.main_image
	FROM
		blogs b
		JOIN admins a
		ON b.admin_id = a.id;

	DELETE FROM blogs;
	INSERT INTO blogs
	(
		id,
		admin_id,
		title,
		content,
		html,
		category_id,
		preview,
		images,
		main_image
	)
	SELECT
		id,
		admin_id,
		title,
		content,
		html,
		category_id,
		preview,
		images,
		main_image
	FROM
		tmp_blogs;

	DROP TABLE tmp_blogs;

	UPDATE admins SET id = account_id;
	UPDATE users SET id = account_id;

	ALTER TABLE admins DROP COLUMN account_id;
	ALTER TABLE users DROP COLUMN account_id;
SET session_replication_role = 'origin';
COMMIT;
