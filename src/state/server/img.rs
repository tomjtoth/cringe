use std::collections::HashMap;

use axum::Extension;
use dioxus::fullstack::FullstackContext;
use dioxus::prelude::{error, info};
use serde::Deserialize;
use sqlx::types::Json;
use sqlx::PgPool;
use tokio::sync::mpsc::{channel, Sender};

use crate::state::server::get_ctx;
use crate::state::server::websocket::ws_notify;
use crate::state::websocket::WsResponse;
use crate::{models::image::Image, state::AUTH_CTE};

#[derive(Deserialize, Default, Debug, sqlx::FromRow)]
struct Result {
    authorized: bool,
    sorted: HashMap<i32, i16>,
    updated: Option<Image>,
    deleted_id: Option<i32>,
    inserted: Option<Image>,
}

pub async fn image_to_db(img: Image) -> anyhow::Result<()> {
    if let (Some(sess_id), pool) = get_ctx().await {
        let bytes = img.bytes();

        let Json(res) = sqlx::query_scalar::<_, Json<Result>>(&format!(
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
                RETURNING ui.id
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
                    prompt = i.prompt,
                    position = i.position
                FROM arg_image i
                CROSS JOIN me
                WHERE ui.user_id = me.id
                AND ui.id = i.id AND i.position IS NOT NULL
                RETURNING ui.*
            ),

            inserted AS (
                INSERT INTO user_images AS ui
                    (user_id, url, prompt, position)
                SELECT
                    me.id, url, prompt, position
                FROM arg_image i
                CROSS JOIN me
                WHERE i.id IS NULL
                RETURNING ui.*
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
                'deleted_id', (SELECT id FROM deleted),
                'updated', (SELECT row_to_json(u) FROM updated u),
                'inserted', (SELECT row_to_json(i) FROM inserted i)
            )
            "
        ))
        .bind(&sess_id)
        .bind(&img.id())
        .bind(&img.pos())
        .bind(&img.prompt())
        .bind(bytes)
        .fetch_one(&pool)
        .await?;

        if let Some(img) = res.inserted {
            info!("Sending job #{}", img.id().unwrap());
            converter_tx().await?.send(img).await?;
        }
    }

    Ok(())
}

#[derive(Deserialize)]
struct UpdateResult {
    needs_bytes: bool,
    session_ids: Vec<String>,
    placeholders: Vec<(i32, String)>,
}

pub fn init_converter(pool: PgPool) -> Sender<Image> {
    let (tx, mut rx) = channel::<Image>(1000);

    tokio::task::spawn_blocking(move || {
        while let Some(mut img) = rx.blocking_recv() {
            let image_id = img.id().unwrap();

            if let Err(e) = img.convert() {
                error!("Failed #{image_id}: {e:?}");
                continue;
            };

            let pool = pool.clone();
            tokio::spawn(async move {
                let Ok(res) = sqlx::query_scalar::<_, Json<UpdateResult>>(
                    r#"
                    WITH job AS (
                        UPDATE user_images ui
                        SET
                            url = NULL,
                            bytes = $2::bytea
                        WHERE id = $1::int
                        RETURNING id, user_id
                    ),

                    user_sessions AS (
                        SELECT u.id AS user_id, jsonb_agg(a.id) AS session_ids
                        FROM users u
                        INNER JOIN auth_sessions a ON a.email = u.email
                        WHERE a.csrf_token IS NULL
                        AND a.email IS NOT NULL
                        AND a.expires_at > NOW()
                        GROUP BY user_id
                    ),

                    queue AS (
                        SELECT
                            id,
                            (
                                -- 1-based indices so -1
                                row_number() OVER (ORDER BY id) - 1
                            ) AS ord
                        FROM user_images ui
                        WHERE user_id > 0 AND bytes IS NULL
                        AND NOT EXISTS(
                            SELECT 1 FROM job j WHERE j.id = ui.id
                        )
                    ),

                    updated_queue AS (
                        UPDATE user_images ui
                        SET url = placeholder_url(ord)
                        FROM queue q
                        WHERE ui.id = q.id
                        RETURNING ui.id, ui.user_id, url
                    ),

                    grouped_changes AS (
                        SELECT 
                            user_id, 
                            jsonb_agg(jsonb_build_array(
                                id, url
                            )) AS placeholders
                        FROM updated_queue uq
                        GROUP BY user_id
                    )

                    SELECT
                        row_to_json(us)::jsonb || 
                        row_to_json(c)::jsonb || 
                        jsonb_build_object(
                            'needs_bytes', j.id IS NOT NULL
                        )
                    FROM user_sessions us
                    INNER JOIN grouped_changes c USING (user_id)
                    LEFT JOIN job j USING (user_id)
                    "#,
                )
                .bind(&image_id)
                .bind(&img.bytes())
                .fetch_all(&pool)
                .await
                .map_err(|e| error!("Failed #{image_id}: {e:?}")) else {
                    return;
                };

                info!("Reporting #{image_id}");

                for Json(UpdateResult {
                    needs_bytes,
                    session_ids,
                    placeholders,
                }) in res
                {
                    for session_id in session_ids {
                        ws_notify(
                            &session_id,
                            WsResponse::ImageUpdate(
                                if needs_bytes { Some(img.clone()) } else { None },
                                placeholders.clone(),
                            ),
                        )
                        .await;
                    }
                }

                info!("Finished #{image_id}");
            });
        }
    });

    tx
}

async fn converter_tx() -> anyhow::Result<Sender<Image>> {
    let Extension(tx) = FullstackContext::extract()
        .await
        .expect("failed to extract converter daemon's tx");

    Ok(tx)
}
