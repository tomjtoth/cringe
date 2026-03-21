CREATE TABLE auth_sessions (
	id TEXT PRIMARY KEY,
	csrf_token TEXT,
	email TEXT,
	expires_at TIMESTAMPTZ NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX auth_sessions_expires_idx ON auth_sessions(expires_at);

CREATE TYPE gender AS ENUM (
	'male',
	'female'
);

CREATE TYPE decision AS ENUM (
	'like',
	'skip'
);

CREATE TYPE frequency AS ENUM (
	'never',
	'rarely',
	'often',
	'yes'
);

CREATE TYPE seeking AS ENUM (
	'short-term fun',
	'short-term, open to long',
	'long-term, open to short',
	'long-term',
	'still figuring it out'
);

CREATE TYPE relationship_type AS ENUM (
	'monogamy',
	'non-monogamy',
	'figuring out my relationship type'
);



CREATE FUNCTION age_from_dob(born DATE) RETURNS INTEGER AS $$
BEGIN
	RETURN date_part('year', age(CURRENT_DATE, born))::int;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT;

CREATE FUNCTION distance_km(lat1 DOUBLE PRECISION, lon1 DOUBLE PRECISION, lat2 DOUBLE PRECISION, lon2 DOUBLE PRECISION) RETURNS DOUBLE PRECISION AS $$
DECLARE
	R CONSTANT DOUBLE PRECISION := 6371.0088; -- Earth radius in kilometers
	phi1 DOUBLE PRECISION := radians(lat1);
	phi2 DOUBLE PRECISION := radians(lat2);
	delta_phi DOUBLE PRECISION := radians(lat2 - lat1);
	delta_lambda DOUBLE PRECISION := radians(lon2 - lon1);
	ahalf DOUBLE PRECISION;
	c DOUBLE PRECISION;
BEGIN
	ahalf := sin(delta_phi / 2)^2 + cos(phi1) * cos(phi2) * sin(delta_lambda / 2)^2;
	c := 2 * atan2(sqrt(ahalf), sqrt(1 - ahalf));
	RETURN R * c;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT;



CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	"name" TEXT NOT NULL,
	email TEXT NOT NULL,
	gender gender NOT NULL,
	born DATE NOT NULL,
	height SMALLINT NOT NULL CHECK (height BETWEEN 30 AND 300),
	education TEXT,
	occupation TEXT,

	"location" TEXT,
	hometown TEXT,
	gps_lat DOUBLE PRECISION,
	gps_lon DOUBLE PRECISION,

	seeking seeking,
	relationship_type relationship_type,

	kids_has SMALLINT,
	kids_wants SMALLINT,

	habits_drinking frequency,
	habits_smoking frequency,
	habits_marijuana frequency,
	habits_drugs frequency,

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

	CONSTRAINT users_gps_lat_ck CHECK (
		gps_lat IS NULL OR (gps_lat BETWEEN -90 AND 90)
	),
	CONSTRAINT users_gps_lon_ck CHECK (
		gps_lon IS NULL OR (gps_lon BETWEEN -180 AND 180)
	)
);


CREATE INDEX users_gender_idx ON users(gender);
CREATE INDEX users_born_idx ON users(born);
CREATE INDEX users_height_idx ON users(height);

CREATE INDEX users_seeking_idx ON users(seeking);
CREATE INDEX users_relationship_type_idx ON users(relationship_type);

CREATE INDEX users_kids_has_idx ON users(kids_has);
CREATE INDEX users_kids_wants_idx ON users(kids_wants);

CREATE INDEX users_habits_drinking_idx ON users(habits_drinking);
CREATE INDEX users_habits_smoking_idx ON users(habits_smoking);
CREATE INDEX users_habits_marijuana_idx ON users(habits_marijuana);
CREATE INDEX users_habits_drugs_idx ON users(habits_drugs);


CREATE TABLE user_prompts (
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	position INTEGER NOT NULL CHECK (position >= 0),
	title TEXT NOT NULL,
	body TEXT NOT NULL,
	PRIMARY KEY (user_id, position)
);

CREATE INDEX user_prompts_user_id_idx ON user_prompts(user_id);

CREATE TABLE user_pictures (
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	position SMALLINT NOT NULL CHECK (position >= 0 AND position < 9),
	"url" TEXT,
	bytes BYTEA,
	mime_type TEXT,
	prompt TEXT,
	PRIMARY KEY (user_id, position),
	CONSTRAINT user_pictures_source_ck CHECK (
		("url" IS NOT NULL AND bytes IS NULL AND mime_type IS NULL)
		OR
		("url" IS NULL AND bytes IS NOT NULL AND mime_type IS NOT NULL)
	)
);

CREATE INDEX user_pictures_user_id_idx ON user_pictures(user_id);

CREATE TABLE user_decisions (
	actor_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	target_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	decision decision NOT NULL,
	PRIMARY KEY (actor_user_id, target_user_id),
	CONSTRAINT user_decisions_no_self_ck CHECK (actor_user_id <> target_user_id)
);

CREATE INDEX user_decisions_actor_user_id_idx ON user_decisions(actor_user_id);
CREATE INDEX user_decisions_target_user_id_idx ON user_decisions(target_user_id);
CREATE INDEX user_decisions_decision_idx ON user_decisions(decision);
