use dioxus::prelude::info;

use crate::models::Profile;

fn parse_yaml() -> anyhow::Result<Vec<Profile>> {
    let yaml_content = std::fs::read_to_string("public/bots.yaml")?;
    let mut bots = serde_yaml::from_str::<Vec<Profile>>(&yaml_content)?;

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
                has_children,
                family_plans,
                drinking,
                smoking,
                marijuana,
                drugs
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
        .bind(bot.has_children)
        .bind(bot.family_plans)
        .bind(bot.drinking)
        .bind(bot.smoking)
        .bind(bot.marijuana)
        .bind(bot.drugs)
        .fetch_one(&mut *tx)
        .await?;

        for (position, prompt) in bot.prompts.iter().enumerate() {
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
