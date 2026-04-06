use dioxus::{
    fullstack::TypedHeader,
    prelude::{
        dioxus_fullstack::{extract::Extension, FullstackContext},
        *,
    },
};
use sqlx::PgPool;
use tokio::sync::mpsc::{channel, Sender};

use crate::{auth::COOKIE_NAME, models::person::Person};

type Job = (i32, Vec<u8>);

pub fn init_converter(pool: PgPool) -> Sender<Job> {
    let (tx, mut rx) = channel::<Job>(1000);

    tokio::task::spawn_blocking(move || {
        while let Some((id, bytes)) = rx.blocking_recv() {
            info!("Starting #{id}");

            let bytes = match convert(bytes) {
                Ok(converted) => {
                    info!("Converted #{id}");
                    converted
                }
                Err(e) => {
                    error!("Failed #{id}: {e:?}");
                    continue;
                }
            };

            let pool = pool.clone();
            tokio::spawn(async move {
                if let Err(e) = sqlx::query(
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
                            placeholder_url(
                                row_number() OVER (ORDER BY id)
                            ) AS url
                        FROM user_images
                        WHERE user_id > 0 AND bytes IS NULL
                    ),

                    updated_queue AS (
                        UPDATE user_images ui
                        SET url = q.url
                        FROM queue q
                        WHERE ui.id = q.id
                        RETURNING ui.id
                    )

                    SELECT 
                        (SELECT count(*) > 0 FROM job),
                        (SELECT count(*) > 0 FROM updated_queue)
                    "#,
                )
                .bind(&id)
                .bind(bytes)
                .execute(&pool)
                .await
                {
                    error!("Failed #{id}: {e:?}",);
                } else {
                    info!("Finished #{id}");
                }
            });
        }
    });

    tx
}

fn convert(bytes: Vec<u8>) -> Result<Vec<u8>> {
    let img = image::load_from_memory(bytes.as_slice())?;

    let resized = img.thumbnail(1920, 1920);

    let mut preview = Vec::new();
    resized.write_to(
        &mut std::io::Cursor::new(&mut preview),
        image::ImageFormat::Avif,
    )?;

    Ok(preview)
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
    let client = reqwest::blocking::Client::builder()
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
