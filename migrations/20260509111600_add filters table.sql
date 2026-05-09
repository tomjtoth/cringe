CREATE TABLE filters (
    user_id INTEGER PRIMARY KEY REFERENCES users(id)
        ON DELETE CASCADE ON UPDATE CASCADE,

    gender gender[],
    gender_identity gender_identity[],

    age_min INTEGER CHECK (age_min >= 18),
    age_max INTEGER CHECK (age_max >= 18 AND (
        age_min IS NULL
        OR age_max IS NULL
        OR age_max >= age_min
    )),

    height_min SMALLINT CHECK (height_min BETWEEN 0 AND 255),
    height_max SMALLINT CHECK (height_max BETWEEN 0 AND 255),

    distance DOUBLE PRECISION CHECK (distance > 0.0),

    zodiac_sign zodiac_sign[],
    seeking seeking[],
    relationship_type relationship_type[],

    has_children BOOLEAN,
    family_plans family_plans[],

    drinking frequency[],
    smoking frequency[],
    marijuana frequency[],
    drugs frequency[],

    image_count_min SMALLINT CHECK (image_count_min BETWEEN 0 AND 6)
);
