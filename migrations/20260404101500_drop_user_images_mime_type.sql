ALTER TABLE user_images
DROP CONSTRAINT IF EXISTS user_images_source_ck,
DROP COLUMN IF EXISTS mime_type,
ADD CONSTRAINT user_images_source_ck
    CHECK (num_nonnulls("url", bytes) > 0);

CREATE FUNCTION placeholder_url(idx bigint)
RETURNS text
LANGUAGE plpgsql
STRICT
AS $$
DECLARE
    content text := xmltext(
        'This image is\n' || 
        CASE
            WHEN idx = 0 THEN 'being converted.\n😁'
            WHEN idx = 1 THEN '1st in the\nconversion queue.\n🤩'
            WHEN idx = 2 THEN '2nd in the\nconversion queue.\n🥹'
            WHEN idx = 3 THEN '3rd in the\nconversion queue.\n🥺'
            ELSE idx || 'th in the\nconversion queue.\n😬'
        END || 
        '\nPlease wait!'
    );

    lines text[] := string_to_array(content, '\n');
    tspans text := '';
    i int;
    line_count int := array_length(lines, 1);
BEGIN
    -- generate tspans
    FOR i IN 1..line_count LOOP
        tspans := tspans || format(
            '<tspan x="50%%" dy="%s1.2em">%s</tspan>',
            CASE WHEN i = 1 THEN '-' ELSE '' END,
            lines[i]
        );
    END LOOP;

    content := format(
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 600 400">
            <style>
                text { fill: #111; }

                @media (prefers-color-scheme: dark) {
                    text { fill: #eee; }
                }
            </style>
            <text fill="currentColor" x="50%%" y="50%%" text-anchor="middle" dominant-baseline="middle" font-size="30">
                %s
            </text>
        </svg>',
        tspans
    );

    RETURN
        'data:image/svg+xml;base64,' ||
        encode(convert_to(content, 'UTF8'), 'base64');
END;
$$;
