use sqlx::types::Json;

use crate::{
    models::person::Person,
    state::{details::DetailsUpateRes, server::ServerCtx, AUTH_CTE},
};

pub async fn update_details(ctx: &ServerCtx, details: Person) -> anyhow::Result<DetailsUpateRes> {
    let kids = details.kids.as_ref();
    let habits = details.habits.as_ref();

    let Json(mut res) = sqlx::query_scalar::<_, Json<DetailsUpateRes>>(&format!(
        r#"
        WITH {AUTH_CTE},

        updated AS (
            UPDATE users u
            SET
                education = $2,
                occupation = $3,
                location = $4,
                hometown = $5,
                seeking = $6,
                relationship_type = $7,
                kids_has = $8,
                kids_wants = $9,
                habits_drinking = $10,
                habits_smoking = $11,
                habits_marijuana = $12,
                habits_drugs = $13
            FROM auth a
            WHERE a.email = u.email
            RETURNING 
                id,
                name,
                height,
                gender,
                education,
                occupation,
                location,
                hometown,
                seeking,
                relationship_type,

                jsonb_build_object(
                    'has', kids_has,
                    'wants', kids_wants
                ) AS kids,

                jsonb_build_object(
                    'drinking', habits_drinking,
                    'smoking', habits_smoking,
                    'marijuana', habits_marijuana,
                    'drugs', habits_drugs
                ) AS habits,

                '[]'::jsonb AS images,
                '[]'::jsonb AS prompts
        )

        SELECT jsonb_build_object(
            'authorized', (SELECT count(*) > 0 FROM auth),
            'profile', (SELECT coalesce(
                (SELECT to_jsonb(u) FROM updated u),
                
                -- falling back to a dummy
                (SELECT jsonb_build_object(
                    'name', '',
                    'height', 0,
                    'gender', 'male',
                    'prompts', '[]'::jsonb,
                    'images', '[]'::jsonb
                ))
            ))
        )
        "#
    ))
    .bind(&ctx.session_id)
    .bind(&details.education)
    .bind(&details.occupation)
    .bind(&details.location)
    .bind(&details.hometown)
    .bind(&details.seeking)
    .bind(&details.relationship_type)
    .bind(&kids.map(|k| k.has.map(|n| n as i16)))
    .bind(&kids.map(|k| k.wants.map(|n| n as i16)))
    .bind(&habits.map(|h| h.drinking))
    .bind(&habits.map(|h| h.smoking))
    .bind(&habits.map(|h| h.marijuana))
    .bind(&habits.map(|h| h.drugs))
    .fetch_one(&ctx.pool)
    .await?;

    if !res.authorized {
        res.profile.id = details.id;
    }

    Ok(res)
}
