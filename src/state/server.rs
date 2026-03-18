use dioxus::{
    fullstack::TypedHeader,
    prelude::dioxus_fullstack::{extract::Extension as FullstackExtension, FullstackContext},
};

use crate::models::person::Person;

pub async fn get_db() -> sqlx::PgPool {
    let FullstackExtension(pool) =
        FullstackContext::extract::<FullstackExtension<sqlx::PgPool>, _>()
            .await
            .expect("PgPool extension is missing from server context");

    pool
}

pub async fn get_cookies() -> axum_extra::headers::Cookie {
    let TypedHeader(cookies) =
        FullstackContext::extract::<TypedHeader<axum_extra::headers::Cookie>, _>()
            .await
            .expect("Cookie header missing from server context");

    cookies
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

        let user_id: i32 = sqlx::query_scalar(
            "
            INSERT INTO users (
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
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19
            )
            RETURNING id
            ",
        )
        .bind(&bot.name)
        .bind(&bot.email)
        .bind(bot.gender)
        .bind(bot.born)
        .bind(i16::from(bot.height))
        .bind(bot.education)
        .bind(bot.occupation)
        .bind(bot.location)
        .bind(bot.hometown)
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

        for (position, prompt) in bot.prompts.into_iter().enumerate() {
            sqlx::query(
                "INSERT INTO user_prompts (user_id, position, title, body) VALUES ($1, $2, $3, $4)",
            )
            .bind(user_id)
            .bind(position as i32)
            .bind(prompt.title)
            .bind(prompt.body)
            .execute(&mut *tx)
            .await?;
        }

        for (position, pic) in bot.pictures.into_iter().enumerate() {
            sqlx::query(
                "INSERT INTO user_pictures (user_id, position, url, prompt) VALUES ($1, $2, $3, $4)",
            )
            .bind(user_id)
            .bind(position as i32)
            .bind(pic.src())
            .bind(pic.prompt())
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;
    println!("Loaded and seeded {} people from bots.yaml", bots_len);
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
        for (idx, img) in pp.pictures.iter().enumerate() {
            if matches!(img, crate::models::person::Pic::Uploaded { .. }) {
                continue;
            }

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
