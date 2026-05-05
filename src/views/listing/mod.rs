mod filters;

use dioxus::prelude::*;

use crate::{
    components::profile::Profile,
    models::{Decision, Profile as MProfile},
    state::AUTH_CTE,
};

pub use filters::Filters;

#[get("/api/profiles?wants")]
async fn get_profiles(wants: Option<Decision>) -> Result<Vec<MProfile>> {
    let mut res = vec![];

    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
        res = sqlx::query_as::<_, MProfile>(&format!(
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

                    drinking,
                    smoking,
                    marijuana,
                    drugs,

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

pub static OTHERS: GlobalSignal<Vec<MProfile>> = GlobalSignal::new(|| vec![]);

/// every other main view sets this to None
pub type ListingCtx = Signal<Option<Option<Decision>>>;

#[component]
pub fn Listing() -> Element {
    let mut lcx = use_context::<ListingCtx>();
    use_effect(move || lcx.set(Some(None)));

    let from_server = use_resource(move || async move { get_profiles(lcx().flatten()).await });

    use_effect(move || {
        if let Some(Ok(profiles)) = from_server().to_owned() {
            *OTHERS.write() = profiles;
        }
    });

    rsx! {
        if OTHERS.read().len() > 0 {
            ul {
                class: "h-full overflow-y-scroll px-2 [&_>_*+*]:mt-2",

                // we're swiping, hide everything but the 1st child
                class: if lcx.read().flatten().is_none() { "[&_>_*+*]:hidden" },

                for profile in OTHERS().into_iter() {
                    li { key: r#"{profile.id.expect("missing ID on profile")}"#,
                        Profile { profile }
                    }
                }
            }
        } else {
            div { class: "app-center text-center",

                h3 { "You have seen everyone!" }

                Filters {}
            }
        }
    }
}
