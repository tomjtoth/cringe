use dioxus::prelude::info;

use crate::{
    models::image::Image,
    state::{
        image::{converter::enqueue, ImageOpResult},
        server::ServerCtx,
        websocket::WsResponse,
        AUTH_CTE,
    },
};

pub async fn image_crud_ops(ctx: &ServerCtx, img: Image) -> anyhow::Result<()> {
    use crate::state::websocket::server::ws_notify;
    use sqlx::types::Json;

    info!("ImageOp: {img:?}");

    let Json(mut res) = sqlx::query_scalar::<_, Json<ImageOpResult>>(&format!(
        r"
        WITH {AUTH_CTE},

        me AS (
            SELECT u.id FROM users u
            INNER JOIN auth a ON a.email = u.email
        ),

        queue AS (
            SELECT count(*) AS idx FROM user_images
            WHERE user_id > 0 AND url IS NOT NULL AND bytes IS NOT NULL
        ),

        arg_image AS (
            SELECT
                $2::int AS id,
                $3::int2 AS position,
                $4::text AS prompt,
                $5::bytea AS bytes,
                placeholder_url(idx) AS url
            FROM queue
        ),

        original AS (
            SELECT ui.id, ui.position
            FROM user_images ui
            INNER JOIN arg_image USING(id)
        ),

        deleted AS (
            DELETE FROM user_images AS ui
            USING arg_image i
            CROSS JOIN me
            WHERE ui.user_id = me.id
            AND i.id = ui.id AND i.position IS NULL
            RETURNING ui.id, ui.user_id
        ),

        sorter AS (
            SELECT ui.id,
                CASE
                    WHEN i.position <= ui.position AND (
                    -- insert a new or shifted an existing below/at this position
                        o.position IS NULL OR o.position > ui.position
                    ) THEN ui.position + 1

                    WHEN o.position < ui.position AND (
                        -- deleted or shifted one above this
                        i.position IS NULL OR i.position >= ui.position
                    ) THEN ui.position -1

                    ELSE ui.position
                END AS position
            FROM user_images ui
            CROSS JOIN me
            CROSS JOIN arg_image i
            LEFT JOIN original o ON TRUE
            WHERE ui.user_id = me.id
        ),

        sorted AS (
            UPDATE user_images ui
            SET
                position = s.position
            FROM sorter s
            WHERE ui.id = s.id AND ui.position != s.position
            RETURNING ui.id, ui.position
        ),

        updated AS (
            UPDATE user_images AS ui
            SET
                position = i.position,
                prompt = i.prompt
            FROM arg_image i
            CROSS JOIN me
            WHERE ui.user_id = me.id
            AND ui.id = i.id AND i.position IS NOT NULL
            RETURNING ui.id, ui.user_id, ui.url, ui.prompt, ui.position
        ),

        inserted AS (
            INSERT INTO user_images AS ui
                (user_id, position, prompt, url, bytes)
            SELECT
                me.id, position, prompt, url, bytes
            FROM arg_image i
            CROSS JOIN me
            WHERE i.id IS NULL AND position IS NOT NULL
            RETURNING ui.id, ui.user_id, ui.url, ui.prompt, ui.position
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
            'image', (SELECT coalesce(
                (SELECT to_jsonb(d) FROM deleted d),
                (SELECT to_jsonb(u) FROM updated u),
                (SELECT to_jsonb(i) FROM inserted i)
            ))
        )
        "
    ))
    .bind(&ctx.session_id)
    .bind(img.id())
    .bind(img.pos())
    .bind(img.prompt())
    .bind(img.bytes())
    .fetch_one(&ctx.pool)
    .await?;

    info!("{res:?}");

    if res.authorized {
        let updating = img.id().is_some() && img.pos().is_some();
        let inserting = img.id().is_none() && img.pos().is_some();

        if let Some(id) = res.image.id().filter(|_| inserting) {
            info!("Sending job #{id}");
            enqueue(id).await?;
        }

        if updating {
            if let Some(bytes) = img.bytes() {
                res.image.set_bytes(bytes.clone());
            }
        }

        ws_notify(None, WsResponse::ImageOp(res)).await;
    } else {
        // TODO
        // ws_notify(Some(vec![sess_id]), todo!()).await;
    }

    Ok(())
}
