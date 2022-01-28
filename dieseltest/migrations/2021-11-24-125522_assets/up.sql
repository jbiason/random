CREATE TABLE assets (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	source TEXT NOT NULL
	-- extras?
);
CREATE INDEX source_idx on assets (source);

SELECT diesel_manage_updated_at('assets');
