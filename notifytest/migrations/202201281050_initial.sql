CREATE TABLE targets (
	level_1 TEXT,
	level_2 TEXT,
	channel_name TEXT,
	UNIQUE (level_1, level_2)
);

CREATE TABLE commands (
	id SERIAL PRIMARY KEY,
	level_1 TEXT,
	level_2 TEXT,
	payload TEXT,
	channel_name TEXT,
	channel_failure BOOLEAN
);

CREATE OR REPLACE FUNCTION notify_channel()
	RETURNS TRIGGER
	LANGUAGE PLPGSQL
	AS
	$$
	DECLARE
		target_channel TEXT;
	BEGIN
		SELECT channel_name
			INTO target_channel
			FROM targets
			WHERE level_1 = NEW.level_1 
			  AND level_2 = NEW.level_2;

		if found then 
			SELECT pg_notify(target_channel, NEW.playload);
			UPDATE commands 
				SET channel_name = channel_name, channel_failure = false 
				WHERE id = NEW.id;
		else
			UPDATE commands
				SET channel_failure = true
				WHERE id = NEW.id;
		end if;

		RETURN NEW;
	END;
	$$;

CREATE TRIGGER notify_channel
	AFTER INSERT
	ON commands
	FOR EACH ROW
		EXECUTE PROCEDURE notify_channel();
