use dioxus::{
    fullstack::TypedHeader,
    prelude::{
        dioxus_fullstack::{extract::Extension, FullstackContext},
        *,
    },
};
use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::{types::Json, PgPool};
use std::collections::HashMap;
use tokio::sync::{
    mpsc::{channel, Sender},
    Mutex as AsyncMutex,
};

use crate::{auth::COOKIE_NAME, models::person::Person, state::websocket::WsResponse};

// Registry of websocket senders keyed by session id.
static WS_REG: Lazy<AsyncMutex<HashMap<String, Sender<WsResponse>>>> =
    Lazy::new(|| AsyncMutex::new(HashMap::new()));

pub async fn register_ws(session_id: String, tx: Sender<WsResponse>) {
    let mut reg = WS_REG.lock().await;
    info!("WS registering: {session_id}");
    reg.insert(session_id, tx);
}

pub async fn unregister_ws(session_id: &str) {
    let mut reg = WS_REG.lock().await;
    info!("WS unregistering: {session_id}");
    reg.remove(session_id);
}

type Job = (i32, Vec<u8>);

#[derive(Deserialize)]
struct UpdateResult {
    needs_bytes: bool,
    session_ids: Vec<String>,
    placeholders: Vec<(i32, String)>,
}

pub fn init_converter(pool: PgPool) -> Sender<Job> {
    let (tx, mut rx) = channel::<Job>(1000);

    tokio::task::spawn_blocking(move || {
        while let Some((image_id, bytes)) = rx.blocking_recv() {
            info!("Starting #{image_id}");

            let Ok(converted) = convert(bytes).map_err(|e| {
                error!("Failed #{image_id}: {e:?}");
            }) else {
                continue;
            };

            info!("Converted #{image_id}");

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
                .bind(&converted)
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
                        if let Some(tx) = WS_REG.lock().await.get(&session_id) {
                            // ignore send errors (receiver may have dropped)
                            let _ = tx
                                .send(WsResponse::ImageUpdate(
                                    if needs_bytes {
                                        Some((image_id, converted.clone()))
                                    } else {
                                        None
                                    },
                                    placeholders.clone(),
                                ))
                                .await;
                        }
                    }
                }

                info!("Finished #{image_id}");
            });
        }
    });

    tx
}

fn convert(bytes: Vec<u8>) -> Result<Vec<u8>> {
    let img = image::load_from_memory(bytes.as_slice())?;

    let resized = img.thumbnail(1920, 1920);

    let mut converted = Vec::new();
    resized.write_to(
        &mut std::io::Cursor::new(&mut converted),
        image::ImageFormat::Avif,
    )?;

    Ok(converted)
}

pub async fn converter_tx() -> Result<Sender<Job>> {
    let Extension(tx) = FullstackContext::extract()
        .await
        .expect("failed to extract converter daemon's tx");

    Ok(tx)
}

async fn get_db() -> sqlx::PgPool {
    let Extension(pool) = FullstackContext::extract()
        .await
        .expect("PgPool extension is missing from server context");

    pool
}

#[get("/api/healthz")]
async fn healthz() -> Result<()> {
    let pool = get_db().await;
    sqlx::query("SELECT 1").execute(&pool).await?;

    Ok(())
}

pub async fn get_ctx() -> (Option<String>, sqlx::PgPool) {
    futures::join!(get_session_id(), get_db())
}

async fn get_cookies() -> Option<axum_extra::headers::Cookie> {
    let TypedHeader(cookies) =
        FullstackContext::extract::<TypedHeader<axum_extra::headers::Cookie>, _>()
            .await
            .ok()?;

    return Some(cookies);
}

async fn get_session_id() -> Option<String> {
    let cookies = get_cookies().await?;
    let id = cookies.get(COOKIE_NAME)?;

    Some(String::from(id))
}

fn parse_yaml() -> anyhow::Result<Vec<Person>> {
    let yaml_content = std::fs::read_to_string("public/bots.yaml")?;
    let mut bots = serde_yaml::from_str::<Vec<Person>>(&yaml_content)?;

    // assign negative IDs to bots
    for (idx, bot) in bots.iter_mut().enumerate() {
        bot.id = Some(-((idx as i32) + 1));
    }

    Ok(bots)
}

pub async fn seed_bots(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let bots = parse_yaml()?;
    let bots_len = bots.len();
    let mut tx = pool.begin().await?;

    // drop all bots on server startup
    sqlx::query("DELETE FROM users WHERE id < 0")
        .execute(&mut *tx)
        .await?;

    for bot in bots {
        let habits = bot.habits.as_ref();
        let kids = bot.kids.as_ref();
        let gps = bot.gps.as_ref();

        sqlx::query(
            "
            INSERT INTO users (
                id,
                name,
                email,
                gender,
                born,
                height,
                education,
                occupation,
                location,
                hometown,
                gps_lat,
                gps_lon,
                seeking,
                relationship_type,
                kids_has,
                kids_wants,
                habits_drinking,
                habits_smoking,
                habits_marijuana,
                habits_drugs
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
                $12, $13, $14, $15, $16, $17, $18, $19, $20
            )
            RETURNING id
            ",
        )
        .bind(&bot.id)
        .bind(&bot.name)
        .bind(&bot.email)
        .bind(bot.gender)
        .bind(bot.born)
        .bind(i16::from(bot.height))
        .bind(&bot.education)
        .bind(&bot.occupation)
        .bind(&bot.location)
        .bind(&bot.hometown)
        .bind(gps.map(|g| g.lat))
        .bind(gps.map(|g| g.lon))
        .bind(bot.seeking)
        .bind(bot.relationship_type)
        .bind(kids.and_then(|k| k.has).map(i16::from))
        .bind(kids.and_then(|k| k.wants).map(i16::from))
        .bind(habits.and_then(|h| h.drinking))
        .bind(habits.and_then(|h| h.smoking))
        .bind(habits.and_then(|h| h.marijuana))
        .bind(habits.and_then(|h| h.drugs))
        .fetch_one(&mut *tx)
        .await?;

        for (position, prompt) in bot.prompts().iter().enumerate() {
            sqlx::query(
                "INSERT INTO user_prompts (user_id, position, title, body) VALUES ($1, $2, $3, $4)",
            )
            .bind(&bot.id)
            .bind(position as i32)
            .bind(&prompt.title)
            .bind(&prompt.body)
            .execute(&mut *tx)
            .await?;
        }

        for (position, img) in bot.images.iter().enumerate() {
            sqlx::query(
                "INSERT INTO user_images (user_id, position, url, prompt) VALUES ($1, $2, $3, $4)",
            )
            .bind(&bot.id)
            .bind(position as i32)
            .bind(img.src())
            .bind(&img.prompt())
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;
    info!("Loaded and seeded {} profiles from bots.yaml", bots_len);
    Ok(())
}

#[test]
fn no_broken_bot_img_urls() {
    let client = oauth2::reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; cringe-bot/1.0)")
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();

    let pps = parse_yaml().unwrap();

    for pp in pps {
        for (idx, img) in pp.images.iter().enumerate() {
            let src = &img.src();

            // check only 3rd party links
            if src.starts_with("https://") {
                let response = client.get(src).send().unwrap_or_else(|err| {
                    panic!(
                        "\n\t{}'s image #{idx} request failed: '{src}'\n\tError: {err}\n",
                        pp.name
                    )
                });
                assert!(
                    response.status().is_success(),
                    "\n\t{}'s image #{idx} is not success: '{}'\n\tStatus: {}\n",
                    pp.name,
                    src,
                    response.status()
                );
            }
        }
    }
}
