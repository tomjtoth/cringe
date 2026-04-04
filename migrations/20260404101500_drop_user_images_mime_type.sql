ALTER TABLE user_images
DROP CONSTRAINT IF EXISTS user_images_source_ck,
DROP COLUMN IF EXISTS mime_type,
ADD CONSTRAINT user_images_source_ck
    CHECK (num_nonnulls("url", bytes) = 1);

CREATE OR REPLACE FUNCTION placeholder_url(idx bigint)
RETURNS text
LANGUAGE plpgsql
IMMUTABLE
AS $$
BEGIN
    IF idx IS NULL THEN
        RAISE EXCEPTION 'placeholder_url(idx): idx must be non-null';
    END IF;

    RETURN format(
        'https://placehold.co/600x400@3x?text=This+image+is\n%s.\nPlease+wait!',
        CASE
            WHEN idx = 0 THEN 'being+converted'
            WHEN idx = 1 THEN '1st+in+the\nconversion+queue'
            WHEN idx = 2 THEN '2nd+in+the\nconversion+queue'
            WHEN idx = 3 THEN '3rd+in+the\nconversion+queue'
            ELSE idx || 'th+in+the\nconversion+queue'
        END
    );
END;
$$;
