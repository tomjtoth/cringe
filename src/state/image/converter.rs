use std::sync::{LazyLock, RwLock};

use dioxus::prelude::{error, info};
use sqlx::{types::Json, PgPool};
use tokio::sync::mpsc::{channel, Sender};

use crate::{
    models::Image,
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

async fn convert_image(pool: PgPool, mut from_db: Image, id: i32) -> anyhow::Result<()> {
    let converted = tokio::task::spawn_blocking(move || {
        from_db.convert()?;

        anyhow::Ok(from_db)
    })
    .await??;

    info!("Converted #{id}");

    let Json(mut res) = sqlx::query_scalar::<_, Json<ImageConversionResult>>(
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
    .await?;

    if let Some(bytes) = converted.bytes() {
        res.image.set_bytes(bytes.clone());
    }

    info!("Reporting #{id}");

    _ = ws_notify(None, WsResponse::ImageConversion(res)).await;

    info!("Finished #{id}");

    Ok(())
}

pub fn init_converter(pool: sqlx::PgPool) {
    let (tx, mut rx) = channel::<i32>(1000);

    *CONVERTER_TX.write().expect("converter tx lock poisoned") = Some(tx.clone());

    tokio::spawn(async move {
        let leftovers = sqlx::query_scalar::<_, Json<Image>>(
            r"
            SELECT to_jsonb(ui) FROM user_images ui
            WHERE bytes IS NOT NULL AND url IS NOT NULL
            ",
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        info!("Found {} unconverted images in DB", leftovers.len());

        for Json(image) in leftovers {
            let id = image.id().unwrap();

            info!("Starting #{id}");

            if let Err(e) = convert_image(pool.clone(), image, id).await {
                error!("Failed #{id}: {e:?}");
            }
        }

        while let Some(id) = rx.recv().await {
            info!("Starting #{id}");

            let pool = pool.clone();
            let Ok(Some(Json(from_db))) = sqlx::query_scalar::<_, Json<Image>>(
                "SELECT to_jsonb(ui) FROM user_images ui WHERE id = $1",
            )
            .bind(&id)
            .fetch_optional(&pool)
            .await
            else {
                error!("Failed to fetch #{id}");
                continue;
            };

            if let Err(e) = convert_image(pool, from_db, id).await {
                error!("Failed #{id}: {e:?}");
            }
        }
    });
}
