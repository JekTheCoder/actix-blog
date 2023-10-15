-- Add migration script here
CREATE OR REPLACE FUNCTION insert_user(TEXT, TEXT, TEXT, TEXT)
RETURNS UUID 
AS 
$$
DECLARE account_id UUID;
BEGIN 
INSERT INTO accounts (username, password, name, kind) VALUES ($1, $2, $3, 'user') RETURNING id INTO account_id;
INSERT INTO users (account_id, email) VALUES (account_id, $4);
RETURN account_id;
END;
$$
language plpgsql;
