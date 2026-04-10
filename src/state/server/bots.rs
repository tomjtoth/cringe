use dioxus::prelude::info;

use crate::models::person::Person;

fn parse_yaml() -> anyhow::Result<Vec<Person>> {
    let yaml_content = std::fs::read_to_string("public/bots.yaml")?;
    let mut bots = serde_yaml::from_str::<Vec<Person>>(&yaml_content)?;

    // assign negative IDs to bots
    for (idx, bot) in bots.iter_mut().enumerate() {
        bot.id = Some(-((idx as i32) + 1));
    }

    Ok(bots)
}

pub async fn seed(pool: &sqlx::PgPool) -> anyhow::Result<()> {
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

    let profiles = parse_yaml().unwrap();

    for profile in profiles {
        for (idx, img) in profile.images.iter().enumerate() {
            let src = &img.src();

            // check only 3rd party links
            if src.starts_with("https://") {
                let response = client.get(src).send().unwrap_or_else(|err| {
                    panic!(
                        "\n\t{}'s image #{idx} request failed: '{src}'\n\tError: {err}\n",
                        profile.name
                    )
                });
                assert!(
                    response.status().is_success(),
                    "\n\t{}'s image #{idx} is not success: '{}'\n\tStatus: {}\n",
                    profile.name,
                    src,
                    response.status()
                );
            }
        }
    }
}
