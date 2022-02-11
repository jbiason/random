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
			SELECT pg_notify(target_channel, NEW.payload);
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

