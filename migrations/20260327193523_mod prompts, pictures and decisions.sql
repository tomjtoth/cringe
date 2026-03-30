ALTER TABLE user_prompts RENAME TO old_user_prompts;

CREATE TABLE user_prompts (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	position SMALLINT NOT NULL CHECK (position >= 0 AND position < 6),
	title TEXT NOT NULL,
	body TEXT NOT NULL,
	CONSTRAINT user_prompts_user_id_position_key
		UNIQUE (user_id, position) DEFERRABLE INITIALLY DEFERRED
);

INSERT INTO user_prompts(user_id, position, title, body)
SELECT user_id, position, title, body
FROM old_user_prompts;

DROP TABLE old_user_prompts;

CREATE INDEX user_prompts_user_id_idx ON user_prompts(user_id);

-- images

CREATE TABLE user_images (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	position SMALLINT NOT NULL CHECK (position >= 0 AND position < 6),
	"url" TEXT,
	bytes BYTEA,
	mime_type TEXT,
	prompt TEXT,
	CONSTRAINT user_images_source_ck CHECK (
		("url" IS NOT NULL AND bytes IS NULL AND mime_type IS NULL)
		OR
		("url" IS NULL AND bytes IS NOT NULL AND mime_type IS NOT NULL)
	),
	CONSTRAINT user_images_user_id_position_key
		UNIQUE (user_id, position) DEFERRABLE INITIALLY DEFERRED
);
CREATE INDEX user_images_user_id_idx ON user_images(user_id);

INSERT INTO user_images (user_id, position, "url", bytes, mime_type, prompt)
SELECT user_id, position, "url", bytes, mime_type, prompt
FROM user_pictures;

DROP TABLE user_pictures;



-- decisions


ALTER TABLE user_decisions
ADD COLUMN liked_prompt_id INTEGER REFERENCES user_prompts(id) ON DELETE SET NULL,
ADD COLUMN liked_image_id INTEGER REFERENCES user_images(id) ON DELETE SET NULL,
ADD COLUMN liked_obj_was_prompt BOOLEAN,
ADD CONSTRAINT liked_obj_consistency_ck CHECK (
  (liked_obj_was_prompt = TRUE  AND liked_prompt_id IS NOT NULL AND liked_image_id IS NULL)
  OR
  (liked_obj_was_prompt = FALSE AND liked_prompt_id IS NULL AND liked_image_id IS NOT NULL)
  OR
  liked_obj_was_prompt IS NULL
  -- allowing for boolean to be set, but img or prompt to be deleted
  -- showing a placeholder img or text instead (Y)
);
