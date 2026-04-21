use sqlx::types::Json;

use crate::{
    models::Profile,
    state::{details::DetailsUpateRes, server::ServerCtx, AUTH_CTE},
};

pub async fn update_details(ctx: &ServerCtx, details: Profile) -> anyhow::Result<DetailsUpateRes> {
    let Json(mut res) = sqlx::query_scalar::<_, Json<DetailsUpateRes>>(&format!(
        r#"
        WITH {AUTH_CTE},

        updated AS (
            UPDATE users u
            SET
                name = $2,
                height = $3,
                gender = $4,
                gender_identity = $5,
                education = $6, 
                occupation = $7, 
                location = $8, 
                hometown = $9, 
                seeking = $10, 
                relationship_type = $11, 
                has_children = $12, 
                family_plans = $13, 
                drinking = $14, 
                smoking = $15,
                marijuana = $16,
                drugs = $17
            FROM auth a
            WHERE a.email = u.email
            RETURNING 
                id,
                name,
                height,

                gender,
                gender_identity,
                
                education,
                occupation,
                location,
                hometown,
                seeking,
                relationship_type,

                has_children,
                family_plans,

                drinking,
                smoking,
                marijuana,
                drugs
        )

        SELECT jsonb_build_object(
            'authorized', (SELECT count(*) > 0 FROM auth),
            'profile', (SELECT coalesce(
                (SELECT to_jsonb(u) FROM updated u),
                
                -- falling back to a dummy
                (SELECT jsonb_build_object(
                    'name', '',
                    'height', 0,
                    'gender', 'male'
                ))
            ))
        )
        "#
    ))
    .bind(&ctx.session_id)
    .bind(&details.name)
    .bind(details.height as i16)
    .bind(&details.gender)
    .bind(&details.gender_identity)
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
