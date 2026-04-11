mod converter;

use std::collections::HashMap;

use dioxus::prelude::info;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

use crate::{
    models::image::Image,
    state::{
        server::get_ctx,
        websocket::{server::ws_notify, WsResponse},
        AUTH_CTE,
    },
};
pub use converter::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageOpResult {
    pub authorized: bool,
    pub image: Image,
    pub sorted: HashMap<i32, i16>,
    // broadcast to all connections for now
    // session_ids: Vec<String>,
}

pub async fn image_op(img: Image) -> anyhow::Result<()> {
    if let (Some(sess_id), pool) = get_ctx().await {
        let Json(res) = sqlx::query_scalar::<_, Json<ImageOpResult>>(&format!(
            r"
            WITH {AUTH_CTE},

            me AS (
                SELECT u.id FROM users u
                INNER JOIN auth a ON a.email = u.email
            ),

            queue AS (
                SELECT count(*) AS idx FROM user_images
                WHERE user_id > 0 AND bytes IS NULL
            ),

            arg_image AS (
                SELECT
                    $4::int AS id,
                    $5::text AS prompt,
                    $6::int2 AS position,
                    placeholder_url(idx) AS url,
                    $7::bytea AS bytes
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
                WHERE i.id IS NULL
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
                    (SELECT row_to_json(d) FROM deleted d),
                    (SELECT row_to_json(u) FROM updated u),
                    (SELECT row_to_json(i) FROM inserted i)
                )
            )
            "
        ))
        .bind(&sess_id)
        .bind(img.id())
        .bind(img.pos())
        .bind(img.prompt())
        .bind(img.bytes())
        .fetch_one(&pool)
        .await?;

        if res.authorized {
            if let Some(id) = res.image.id() {
                info!("Sending job #{id}");
                converter_tx().await?.send(*id).await?;
            }

            ws_notify(None, WsResponse::ImageOp(res)).await;
        } else {
            // TODO
            // ws_notify(Some(vec![sess_id]), todo!()).await;
        }
    }

    Ok(())
}
