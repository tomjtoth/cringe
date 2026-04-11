use std::collections::HashMap;

use axum::Extension;
use dioxus::fullstack::FullstackContext;
use dioxus::prelude::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::PgPool;
use tokio::sync::mpsc::{channel, Sender};

use crate::{
    models::image::Image,
    state::websocket::{server::ws_notify, WsResponse},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageConversionResult {
    pub image: Image,
    pub placeholders: HashMap<i32, String>,
}

pub(super) async fn converter_tx() -> anyhow::Result<Sender<i32>> {
    let Extension(tx) = FullstackContext::extract()
        .await
        .expect("failed to extract converter daemon's tx");

    Ok(tx)
}

pub fn init_converter(pool: PgPool) -> Sender<i32> {
    let (tx, mut rx) = channel::<i32>(1000);

    tokio::spawn(async move {
        while let Some(id) = rx.recv().await {
            let pool = pool.clone();

            info!("Starting #{id}");

            let Some(Json(mut from_db)) = sqlx::query_scalar::<_, Json<Image>>(
                "SELECT row_to_json(ui) FROM user_images ui WHERE id = $1",
            )
            .bind(&id)
            .fetch_optional(&pool)
            .await?
            else {
                error!("Failed to fetch #{id}");
                continue;
            };

            let Ok(converted) = tokio::task::spawn_blocking(move || {
                from_db.convert()?;

                anyhow::Ok(from_db)
            })
            .await?
            .map_err(|e| error!("Failed to convert #{id}: {e:?}")) else {
                continue;
            };

            info!("Converted #{id}");

            let Ok(Json(mut res)) = sqlx::query_scalar::<_, Json<ImageConversionResult>>(
                r#"
                    WITH job AS (
                        UPDATE user_images ui
                        SET
                            url = NULL,
                            bytes = $2::bytea
                        WHERE id = $1::int
                        RETURNING id
                    ),

                    queue AS (
                        SELECT
                            id,
                            (
                                -- 1-based indices so -1
                                row_number() OVER (ORDER BY id) - 1
                            ) AS ord
                        FROM user_images ui
                        WHERE user_id > 0 AND NOT EXISTS(
                            SELECT 1 FROM job j WHERE j.id = ui.id
                        ) AND url IS NOT NULL AND bytes IS NOT NULL
                    ),

                    updated_queue AS (
                        UPDATE user_images ui
                        SET url = placeholder_url(ord)
                        FROM queue q
                        WHERE ui.id = q.id
                        RETURNING ui.id, ui.url
                    ),

                    placeholders AS (
                        SELECT jsonb_object_agg(
                            id, url
                        ) AS placeholders
                        FROM updated_queue uq
                    )

                    SELECT
                        row_to_json(p) || 
                        jsonb_build_object('image', NULL)
                    FROM placeholders p
                    "#,
            )
            .bind(&id)
            .bind(&converted.bytes())
            .fetch_one(&pool)
            .await
            .map_err(|e| error!("Failed #{id}: {e:?}")) else {
                continue;
            };

            res.image = converted;

            info!("Reporting #{id}");

            _ = ws_notify(None, WsResponse::ImageConversion(res)).await;

            info!("Finished #{id}");
        }

        anyhow::Ok(())
    });

    tx
}
