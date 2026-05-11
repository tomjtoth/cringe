use dioxus::prelude::*;

use crate::{models::Filters, state::AUTH_CTE};

#[put("/api/me/filters")]
pub(crate) async fn update_filters(filters: Filters) -> Result<bool> {
    let mut res = false;

    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
        let db_res = sqlx::query(&format!(
            "
            WITH {AUTH_CTE},

            me AS (
                SELECT u.id
                FROM users u
                INNER JOIN auth a ON u.email = a.email
            )

            INSERT INTO filters (
                user_id,
                gender,
                gender_identity,
                age_min,
                age_max,
                height_min,
                height_max,
                distance,
                zodiac_sign,
                seeking,
                relationship_type,
                has_children,
                family_plans,
                drinking,
                smoking,
                marijuana,
                drugs,
                image_count_min,
                strict_mode
            )
            SELECT
                me.id,
                NULLIF($2, '{{}}'::gender[]),
                NULLIF($3, '{{}}'::gender_identity[]),
                $4,
                $5,
                $6,
                $7,
                $8,
                NULLIF($9, '{{}}'::zodiac_sign[]),
                NULLIF($10, '{{}}'::seeking[]),
                NULLIF($11, '{{}}'::relationship_type[]),
                $12,
                NULLIF($13, '{{}}'::family_plans[]),
                NULLIF($14, '{{}}'::frequency[]),
                NULLIF($15, '{{}}'::frequency[]),
                NULLIF($16, '{{}}'::frequency[]),
                NULLIF($17, '{{}}'::frequency[]),
                $18,
                $19
            FROM me
            ON CONFLICT (user_id)
            DO UPDATE SET
                gender = EXCLUDED.gender,
                gender_identity = EXCLUDED.gender_identity,
                age_min = EXCLUDED.age_min,
                age_max = EXCLUDED.age_max,
                height_min = EXCLUDED.height_min,
                height_max = EXCLUDED.height_max,
                distance = EXCLUDED.distance,
                zodiac_sign = EXCLUDED.zodiac_sign,
                seeking = EXCLUDED.seeking,
                relationship_type = EXCLUDED.relationship_type,
                has_children = EXCLUDED.has_children,
                family_plans = EXCLUDED.family_plans,
                drinking = EXCLUDED.drinking,
                smoking = EXCLUDED.smoking,
                marijuana = EXCLUDED.marijuana,
                drugs = EXCLUDED.drugs,
                image_count_min = EXCLUDED.image_count_min,
                strict_mode = EXCLUDED.strict_mode;
            ",
        ))
        .bind(&sess_id)
        .bind(&filters.gender)
        .bind(&filters.gender_identity)
        .bind(&filters.age_min.map(|n| n as i32))
        .bind(&filters.age_max.map(|n| n as i32))
        .bind(&filters.height_min.map(|n| n as i16))
        .bind(&filters.height_max.map(|n| n as i16))
        .bind(&filters.distance.map(|n| n as i32))
        .bind(&filters.zodiac_sign)
        .bind(&filters.seeking)
        .bind(&filters.relationship_type)
        .bind(&filters.has_children)
        .bind(&filters.family_plans)
        .bind(&filters.drinking)
        .bind(&filters.smoking)
        .bind(&filters.marijuana)
        .bind(&filters.drugs)
        .bind(&filters.image_count_min.map(|n| n as i16))
        .bind(&filters.strict_mode)
        .execute(&pool)
        .await?;

        res = db_res.rows_affected() > 0;
    }

    Ok(res)
}
