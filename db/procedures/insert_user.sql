CREATE OR REPLACE FUNCTION insert_user(TEXT, TEXT, TEXT, TEXT)
RETURNS UUID 
AS 
$$
DECLARE agent_id UUID;
BEGIN 
INSERT INTO agents (username, password, name, type) VALUES ($1, $2, $3, 'user') RETURNING id INTO agent_id;
INSERT INTO users (agent_id, email) VALUES (agent_id, $4);
RETURN agent_id;
END;
$$
language plpgsql;
