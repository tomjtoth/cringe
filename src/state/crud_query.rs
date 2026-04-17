use std::collections::HashMap;

use crate::state::AUTH_CTE;

pub(super) type Sorted = HashMap<i32, i16>;

/// produces SQL for image and prompt ops
pub(super) fn crud_query(for_prompt: bool) -> String {
    let tbl = if for_prompt {
        "user_prompts AS tbl"
    } else {
        "user_images AS tbl"
    };

    let arg = if for_prompt {
        r"raw_arg AS (
            SELECT
                $2::int AS id,
                $3::int2 AS position,
                $4::text AS title,
                $5::text AS body
        ),

        arg AS (
            SELECT
                id,
                CASE
                    WHEN title = '' OR BODY = '' THEN NULL
                    ELSE position
                END AS position,
                title,
                body
            FROM raw_arg
        )"
    } else {
        r"queue AS (
            SELECT count(*) AS idx FROM user_images
            WHERE user_id > 0 AND url IS NOT NULL AND bytes IS NOT NULL
        ),

        arg AS (
            SELECT
                $2::int AS id,
                $3::int2 AS position,
                $4::text AS prompt,
                $5::bytea AS bytes,
                placeholder_url(idx) AS url
            FROM queue
        )"
    };

    let deleted_return_clause = if for_prompt {
        "tbl.id, tbl.user_id, tbl.title, tbl.body"
    } else {
        "tbl.id, tbl.user_id"
    };

    let updated_set_cols = if for_prompt {
        "title = arg.title, body = arg.body"
    } else {
        "prompt = arg.prompt"
    };

    let returning_clause = if for_prompt {
        "tbl.*"
    } else {
        "tbl.id, tbl.user_id, tbl.url, tbl.prompt, tbl.position"
    };

    let inserted_cols = if for_prompt {
        "title, body"
    } else {
        "prompt, url, bytes"
    };

    let result_name = if for_prompt { "prompt" } else { "image" };

    format!(
        "WITH

        {AUTH_CTE},

        me AS (
            SELECT u.id FROM users u
            INNER JOIN auth a ON a.email = u.email
        ),

        {arg},

        original AS (
            SELECT tbl.id, tbl.position
            FROM {tbl}
            INNER JOIN arg USING(id)
        ),

        deleted AS (
            DELETE FROM {tbl}
            USING arg
            CROSS JOIN me
            WHERE tbl.user_id = me.id
            AND arg.id = tbl.id AND arg.position IS NULL
            RETURNING {deleted_return_clause}
        ),

        sorter AS (
            SELECT
                tbl.id,
                CASE
                    WHEN arg.position <= tbl.position AND (
                    -- insert a new or shifted an existing below/at this position
                        oa.position IS NULL OR oa.position > tbl.position
                    ) THEN tbl.position + 1

                    WHEN oa.position < tbl.position AND (
                        -- deleted or shifted one above this
                        arg.position IS NULL OR arg.position >= tbl.position
                    ) THEN tbl.position -1

                    ELSE tbl.position
                END AS position
            FROM {tbl}
            CROSS JOIN me
            CROSS JOIN arg
            LEFT JOIN original oa ON TRUE
            WHERE tbl.user_id = me.id
        ),

        sorted AS (
            UPDATE {tbl}
            SET
                position = s.position
            FROM sorter s
            WHERE tbl.id = s.id AND tbl.position != s.position
            RETURNING tbl.id, tbl.position
        ),

        updated AS (
            UPDATE {tbl}
            SET
                position = arg.position,
                {updated_set_cols}
            FROM arg
            CROSS JOIN me
            WHERE tbl.user_id = me.id AND tbl.id = arg.id
            AND arg.position IS NOT NULL
            RETURNING {returning_clause}
        ),

        inserted AS (
            INSERT INTO {tbl}
                (user_id, position, {inserted_cols})
            SELECT
                me.id, position, {inserted_cols}
            FROM arg
            CROSS JOIN me
            WHERE arg.id IS NULL AND position IS NOT NULL
            RETURNING {returning_clause}
        )

        SELECT jsonb_build_object(
            'authorized', (SELECT count(*) > 0 FROM me),
            'sorted', (
                SELECT COALESCE(
                    jsonb_object_agg(id, position),
                    '{{}}'::jsonb
                )
                FROM sorted
            ),
            '{result_name}', (SELECT coalesce(
                (SELECT to_jsonb(d) FROM deleted d),
                (SELECT to_jsonb(u) FROM updated u),
                (SELECT to_jsonb(i) FROM inserted i)
            ))
        )
        ",
    )
}
