use sqlx::types::Json;

use crate::{
    models::person::Person,
    state::{details::DetailsUpateRes, server::ServerCtx, AUTH_CTE},
};

pub async fn update_details(ctx: &ServerCtx, details: Person) -> anyhow::Result<DetailsUpateRes> {
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
                has_children = $8,
                family_plans = $9,
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

                has_children,
                family_plans,

                habits_drinking,
                habits_smoking,
                habits_marijuana,
                habits_drugs,

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
    .bind(&details.has_children)
    .bind(&details.family_plans)
    .bind(&details.drinking)
    .bind(&details.smoking)
    .bind(&details.marijuana)
    .bind(&details.drugs)
    .fetch_one(&ctx.pool)
    .await?;

    if !res.authorized {
        res.profile.id = details.id;
    }

    Ok(res)
}
