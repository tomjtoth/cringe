use dioxus::{html::geometry::euclid::Vector2D, prelude::*};

use crate::{
    components::profile::Profile,
    models::{Decision, Profile as MProfile},
    state::AUTH_CTE,
};

#[get("/api/profiles?wants")]
async fn get_profiles(wants: Option<Decision>) -> Result<Vec<MProfile>> {
    let mut res = vec![];

    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
        let decision_operator = if wants.is_some() { " = $2" } else { "IS NULL" };

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
                            jsonb_agg(row_to_json(pp) ORDER BY pp.position),
                            '[]'::jsonb
                        )
                        FROM user_prompts pp
                        WHERE pp.user_id = u.id
                    ) as prompts,

                    (
                        WITH ui AS (
                            SELECT
                                ui.id,
                                ui.user_id,
                                ui.position,
                                ui.prompt,
                                ui.url,

                                CASE
                                    WHEN $2 IN ('like', 'skip')
                                    AND ui.position > 0
                                    THEN NULL
                                    ELSE ui.bytes
                                END AS bytes

                            FROM user_images ui
                            WHERE ui.user_id = u.id
                        )

                        SELECT coalesce(
                            jsonb_agg(
                                row_to_json(ui)
                                ORDER BY position
                            ),
                            '[]'::jsonb
                        )
                        FROM ui
                    ) AS images

                FROM users u
                CROSS JOIN me
                LEFT JOIN user_decisions d 
                    ON d.actor_user_id = me.id 
                AND d.target_user_id = u.id
                WHERE u.id <> me.id
                AND d.decision {decision_operator}
                ORDER BY distance
                "#,
            ))
            .bind(&sess_id)
            .bind(&wants)
            .fetch_all(&pool)
            .await?;
    }

    Ok(res)
}

pub static OTHERS: GlobalSignal<Vec<MProfile>> = GlobalSignal::new(|| vec![]);

/// ### ListingContext
pub(crate) const LCX: GlobalSignal<Option<Option<Decision>>> = GlobalSignal::new(|| None);

#[component]
pub fn Listing() -> Element {
    use_effect(|| LCX.with_mut(|lcx| *lcx = Some(None)));

    let from_server = use_resource(move || async move { get_profiles(LCX().flatten()).await });

    let mut ul_ref = use_signal(|| None::<std::rc::Rc<MountedData>>);

    let ul_len = use_memo(|| OTHERS.read().len());

    use_effect(move || {
        if let Some(Ok(profiles)) = from_server().to_owned() {
            *OTHERS.write() = profiles;
        }
    });

    use_effect(move || {
        let _subscribe_here = ul_len();

        if let Some(ul) = ul_ref() {
            spawn(async move {
                _ = ul.scroll(Vector2D::new(0.0, 0.0), ScrollBehavior::Smooth);
            });
        }
    });

    rsx! {
        if OTHERS.read().len() > 0 {
            div {
                class: "p-2 pt-0 h-full overflow-y-scroll",
                onmounted: move |cx| ul_ref.set(Some(cx.data())),
                div {
                    class: "sm:columns-2 lg:columns-3",

                    // we're swiping, hide everything but the 1st child
                    class: if LCX.read().flatten().is_none() { "[&_>_*+*]:hidden" },

                    for profile in OTHERS().into_iter() {
                        Profile {
                            key: r#"{profile.id.expect("missing ID on profile")}"#,
                            profile,
                        }
                    }
                }
            }
        } else {
            div { class: "app-center text-center",
                h3 { "You have seen everyone!" }
                p { "Adjust your filters in the bottom left corner." }
            }
        }
    }
}
