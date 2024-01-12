ALTER TABLE blogs
	ADD created_at timestamp NOT NULL DEFAULT now();

ALTER TABLE comments
	ADD created_at timestamp NOT NULL DEFAULT now();

ALTER TABLE replies
	ADD created_at timestamp NOT NULL DEFAULT now();

ALTER TABLE accounts
	ADD created_at timestamp NOT NULL DEFAULT now();
