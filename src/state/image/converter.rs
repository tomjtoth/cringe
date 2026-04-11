use std::sync::{LazyLock, RwLock};

use dioxus::prelude::{error, info};
use sqlx::types::Json;
use tokio::sync::mpsc::{channel, Sender};

use crate::{
    models::image::Image,
    state::{
        image::ImageConversionResult,
        websocket::{server::ws_notify, WsResponse},
    },
};

static CONVERTER_TX: LazyLock<RwLock<Option<Sender<i32>>>> = LazyLock::new(|| RwLock::new(None));

pub(super) async fn enqueue(id: i32) -> anyhow::Result<()> {
    let Some(tx) = CONVERTER_TX
        .read()
        .expect("converter tx lock poisoned")
        .clone()
    else {
        anyhow::bail!("converter queue is not initialized")
    };

    tx.send(id).await?;

    Ok(())
}

pub fn init_converter(pool: sqlx::PgPool) {
    let (tx, mut rx) = channel::<i32>(1000);

    *CONVERTER_TX.write().expect("converter tx lock poisoned") = Some(tx);

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
                        RETURNING id, user_id, position, prompt
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
                        SELECT coalesce(
                            jsonb_object_agg(id, url),
                            '{}'::jsonb
                        ) AS placeholders
                        FROM updated_queue uq
                    )

                    SELECT
                        to_jsonb(p) ||
                        jsonb_build_object('image', to_jsonb(j))
                    FROM placeholders p
                    CROSS JOIN job j
                    "#,
            )
            .bind(&id)
            .bind(&converted.bytes())
            .fetch_one(&pool)
            .await
            .map_err(|e| error!("Failed #{id}: {e:?}")) else {
                continue;
            };

            if let Some(bytes) = converted.bytes() {
                res.image.set_bytes(bytes.clone());
            }

            info!("Reporting #{id}");

            _ = ws_notify(None, WsResponse::ImageConversion(res)).await;

            info!("Finished #{id}");
        }

        anyhow::Ok(())
    });
}
