CREATE TYPE family_plans AS ENUM ('wants children', 'doesn''t want children', 'not sure yet');

ALTER TABLE users
ADD COLUMN family_plans family_plans,
ADD COLUMN has_children BOOLEAN;

UPDATE users SET
    family_plans = CASE
        WHEN kids_wants > 0 THEN 'wants children'::family_plans
        WHEN kids_wants = 0 THEN 'doesn''t want children'::family_plans
        WHEN kids_wants = -1 THEN 'not sure yet'::family_plans
    END,
    has_children = CASE
        WHEN kids_has > 0 THEN true
        WHEN kids_has = 0 THEN false
    END;

ALTER TABLE users
DROP COLUMN kids_wants,
DROP COLUMN kids_has;
