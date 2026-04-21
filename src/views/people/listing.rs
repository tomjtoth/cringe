use dioxus::prelude::*;

use crate::{
    models::{Decision, Person},
    state::AUTH_CTE,
    views::{people::profile::Profile as VPerson, protector::NeedsLoginAndProfile},
};

#[get("/api/profiles?wants")]
async fn get_profiles(wants: Option<Decision>) -> Result<Vec<Person>> {
    let mut res = vec![];

    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
        res = sqlx::query_as::<_, Person>(&format!(
                r#"
                WITH {AUTH_CTE},

                me AS (
                    SELECT
                        u.id, gps_lon, gps_lat
                        -- TODO: expand later with other filters, such as distance, age_min, age_max, gender
                    FROM auth a
                    JOIN users u on a.email = u.email
                )

                SELECT
                    u.id,
                    name,
                    gender,
                    height,
                    education,
                    occupation,
                    location,
                    hometown,

                    age_from_dob(born) as age,
                    zodiac_sign_from_dob(born) as zodiac_sign,
                    distance_km(
                        u.gps_lat, u.gps_lon,
                        me.gps_lat, me.gps_lon
                    ) as distance,

                    seeking,
                    relationship_type,

                    has_children,
                    family_plans,

                    habits_drinking,
                    habits_smoking,
                    habits_marijuana,
                    habits_drugs,

                    (
                        SELECT coalesce(
                            json_agg(row_to_json(pp) ORDER BY pp.position),
                            '[]'
                        )
                        FROM user_prompts pp
                        WHERE pp.user_id = u.id
                    ) as prompts,

                    (
                        SELECT coalesce(
                            json_agg(row_to_json(ui) ORDER BY ui.position),
                            '[]'
                        )
                        FROM user_images ui
                        WHERE ui.user_id = u.id
                    ) AS images

                FROM users u
                CROSS JOIN me
                LEFT JOIN user_decisions d ON d.actor_user_id = me.id AND d.target_user_id = u.id
                WHERE u.id <> me.id
                AND d.decision {}
                ORDER BY distance
                "#,
                if wants.is_some() { " = $2" } else { "IS NULL" }
            ))
            .bind(&sess_id)
            .bind(&wants)
            .fetch_all(&pool)
            .await?;
    }

    Ok(res)
}

#[component]
pub fn LikedProfiles() -> Element {
    rsx! {
        ListProfiles { wants: Decision::Like }
    }
}

#[component]
pub fn SkippedProfiles() -> Element {
    rsx! {
        ListProfiles { wants: Decision::Skip }
    }
}

#[component]
pub fn SwipeProfiles() -> Element {
    rsx! {
        ListProfiles {}
    }
}

pub static OTHERS: GlobalSignal<Vec<Person>> = GlobalSignal::new(|| vec![]);

pub type ListingCtx = Option<Decision>;

#[component]
fn ListProfiles(wants: Option<Decision>) -> Element {
    let from_server = use_server_future(move || async move { get_profiles(wants).await })?;

    use_effect(move || {
        if let Some(Ok(peeps)) = from_server().to_owned() {
            *OTHERS.write() = peeps;
        }
    });

    use_context_provider(|| Some(wants));

    rsx! {
        NeedsLoginAndProfile {
            if OTHERS.read().len() > 0 {
                ul {
                    class: "h-full overflow-y-scroll px-2 [&_>_*+*]:mt-2",

                    // we're swiping, hide everything but the 1st child
                    class: if wants.is_none() { "[&_>_*+*]:hidden" },

                    for profile in OTHERS().into_iter() {
                        li { key: r#"{profile.id.expect("missing ID on profile")}"#,
                            VPerson { profile }
                        }
                    }
                }
            } else {
                p { class: "app-center", "Nobody here!" }
            }
        }
    }
}
