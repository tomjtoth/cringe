use dioxus::prelude::info;
use sqlx::types::Json;

use crate::models::Profile;

fn parse_yaml() -> anyhow::Result<Vec<Profile>> {
    let yaml_content = std::fs::read_to_string("public/bots.yaml")?;
    let bots = serde_yaml::from_str::<Vec<Profile>>(&yaml_content)?;
    Ok(bots)
}

pub async fn seed(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let bots = parse_yaml()?;
    let bots_len = bots.len();
    let mut tx = pool.begin().await?;

    for bot in bots {
        let gps = bot.gps.as_ref();

        sqlx::query(
            "
            WITH bot AS (
                SELECT 
                    $1 AS name,
                    $2 AS email,
                    $3 AS gender,
                    $4 AS born,
                    $5 AS height,
                    $6 AS education,
                    $7 AS occupation,
                    $8 AS location,
                    $9 AS hometown,
                    $10 AS gps_lat,
                    $11 AS gps_lon,
                    $12 AS seeking,
                    $13 AS relationship_type,
                    $14 AS has_children,
                    $15 AS family_plans,
                    $16 AS drinking,
                    $17 AS smoking,
                    $18 AS marijuana,
                    $19 AS drugs,
                    $20 AS ai_personality
            ),

            updated_bot AS (
                UPDATE users u SET
                    name = b.name,
                    gender = b.gender,
                    born = b.born,
                    height = b.height,
                    education = b.education,
                    occupation = b.occupation,
                    location = b.location,
                    hometown = b.hometown,
                    gps_lat = b.gps_lat,
                    gps_lon = b.gps_lon,
                    seeking = b.seeking,
                    relationship_type = b.relationship_type,
                    has_children = b.has_children,
                    family_plans = b.family_plans,
                    drinking = b.drinking,
                    smoking = b.smoking,
                    marijuana = b.marijuana,
                    drugs = b.drugs,
                    ai_personality = b.ai_personality
                FROM bot b
                WHERE b.email = u.email
                RETURNING u.id
            ),

            inserted_bot AS (
                INSERT INTO users AS u (
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
                    drugs,
                    ai_personality
                )
                SELECT 
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
                    drugs,
                    ai_personality
                FROM bot b
                WHERE NOT EXISTS (
                    SELECT 1 FROM users u2
                    WHERE u2.email = b.email
                )
                RETURNING u.id
            ),

            user_id AS (
                SELECT id FROM updated_bot
                UNION
                SELECT id FROM inserted_bot
            ),

            prompts AS (
                SELECT
                    uid.id AS user_id,
                    (prompt.ordinality - 1)::smallint AS position,
                    prompt.value->>'title' AS title,
                    prompt.value->>'body' AS body
                FROM user_id uid
                CROSS JOIN LATERAL jsonb_array_elements($21::jsonb)
                    WITH ORDINALITY AS prompt(value, ordinality)
            ),

            updated_prompts AS (
                UPDATE user_prompts up
                SET
                    title = p.title,
                    body = p.body
                FROM prompts p
                WHERE up.user_id = p.user_id
                AND up.position = p.position
                RETURNING up.id
            ),

            inserted_prompts AS (
                INSERT INTO user_prompts AS up 
                    (user_id, position, title, body)
                SELECT p.user_id, p.position, p.title, p.body
                FROM prompts p
                WHERE NOT EXISTS (
                    SELECT 1 FROM user_prompts up2
                    WHERE up2.user_id = p.user_id
                    AND up2.position = p.position
                )
                RETURNING up.id
            ),

            deleted_prompts AS (
                DELETE FROM user_prompts AS up
                USING user_id uid
                WHERE up.user_id = uid.id
                AND NOT EXISTS (
                    SELECT 1 FROM prompts p 
                    WHERE p.position = up.position
                )
                RETURNING up.id
            ),

            images AS (
                SELECT
                    uid.id AS user_id,
                    (image.ordinality - 1)::smallint AS position,
                    CASE
                        WHEN jsonb_typeof(image.value) = 'string' THEN image.value #>> '{}'
                        ELSE image.value->>'url'
                    END AS url,
                    CASE
                        WHEN jsonb_typeof(image.value) = 'object' THEN image.value->>'prompt'
                        ELSE NULL
                    END AS prompt
                FROM user_id uid
                CROSS JOIN LATERAL jsonb_array_elements($22::jsonb)
                    WITH ORDINALITY AS image(value, ordinality)
            ),

            inserted_images AS (
                INSERT INTO user_images AS ui 
                    (user_id, position, url, prompt)
                SELECT i.user_id, i.position, i.url, i.prompt
                FROM images i
                WHERE NOT EXISTS (
                    SELECT 1 FROM user_images ui2
                    WHERE ui2.user_id = i.user_id
                    AND ui2.position = i.position
                )
                RETURNING ui.id
            ),

            updated_images AS (
                UPDATE user_images ui
                SET
                    prompt = i.prompt,
                    url = i.url
                FROM images i
                WHERE ui.user_id = i.user_id
                AND ui.position = i.position
                RETURNING ui.id
            ),

            deleted_images AS (
                DELETE FROM user_images ui
                USING user_id uid
                WHERE ui.user_id = uid.id 
                AND NOT EXISTS (
                    SELECT 1 FROM images i 
                    WHERE i.position = ui.position
                )
                RETURNING ui.id
            )

            -- including all CTEs in the final query
            SELECT id FROM inserted_prompts
            UNION
            SELECT id FROM updated_prompts
            UNION
            SELECT id FROM deleted_prompts
            UNION
            SELECT id FROM inserted_images
            UNION
            SELECT id FROM updated_images
            UNION
            SELECT id FROM deleted_images
            ",
        )
        .bind(&bot.name)
        .bind(&bot.email)
        .bind(&bot.gender)
        .bind(&bot.born)
        .bind(i16::from(bot.height))
        .bind(&bot.education)
        .bind(&bot.occupation)
        .bind(&bot.location)
        .bind(&bot.hometown)
        .bind(&gps.map(|g| g.lat))
        .bind(&gps.map(|g| g.lon))
        .bind(&bot.seeking)
        .bind(&bot.relationship_type)
        .bind(&bot.has_children)
        .bind(&bot.family_plans)
        .bind(&bot.drinking)
        .bind(&bot.smoking)
        .bind(&bot.marijuana)
        .bind(&bot.drugs)
        .bind(&bot.ai_personality)
        .bind(Json(&bot.prompts))
        .bind(Json(&bot.images))
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    info!("Loaded and seeded {} profiles from bots.yaml", bots_len);
    Ok(())
}
