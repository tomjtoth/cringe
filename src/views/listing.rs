use dioxus::{html::geometry::euclid::Vector2D, prelude::*};

use crate::{
    components::profile::Profile,
    models::{Decision, Profile as MProfile},
    state::AUTH_CTE,
};

fn strict_multi(col: &str) -> String {
    format!(
        "AND (
            f.{col} IS NULL
            OR u.{col} = ANY(f.{col})
            OR (NOT f.strict_mode AND u.{col} IS NULL)
        )"
    )
}

#[get("/api/profiles?wants")]
async fn get_profiles(wants: Option<Decision>) -> Result<Vec<MProfile>> {
    let mut res = vec![];

    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
        let decision_operator = if wants.is_some() { " = $2" } else { "IS NULL" };
        let gender_identity = strict_multi("gender_identity");
        let seeking = strict_multi("seeking");
        let relationship_type = strict_multi("relationship_type");
        let family_plans = strict_multi("family_plans");

        let drinking = strict_multi("drinking");
        let smoking = strict_multi("smoking");
        let marijuana = strict_multi("marijuana");
        let drugs = strict_multi("drugs");

        res = sqlx::query_as::<_, MProfile>(&format!(
            r#"
            WITH {AUTH_CTE},

            me AS (
                SELECT u.id, gps_lon, gps_lat
                FROM auth a
                JOIN users u on a.email = u.email
            ),

            partially_filtered AS (
                SELECT
                    u.id,
                    u.name,
                    u.gender,
                    u.height,
                    u.education,
                    u.occupation,
                    u.location,
                    u.hometown,

                    age_from_dob(born) as age,
                    zodiac_sign_from_dob(born) as zodiac_sign,
                    distance_km(
                        u.gps_lat, u.gps_lon,
                        me.gps_lat, me.gps_lon
                    ) as distance,

                    u.seeking,
                    u.relationship_type,

                    u.has_children,
                    u.family_plans,

                    u.drinking,
                    u.smoking,
                    u.marijuana,
                    u.drugs,

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
                LEFT JOIN filters f ON f.user_id = me.id
                WHERE u.id <> me.id
                AND d.decision {decision_operator}

                AND (f.gender IS NULL OR u.gender = ANY(f.gender))
                {gender_identity}

                AND (f.height_min IS NULL OR u.height >= f.height_min)
                AND (f.height_max IS NULL OR u.height <= f.height_max)

                {seeking}
                {relationship_type}

                AND (f.has_children IS NULL OR u.has_children = f.has_children)
                {family_plans}

                {drinking}
                {smoking}
                {marijuana}
                {drugs}
            )

            SELECT pf.*
            FROM partially_filtered pf
            CROSS JOIN me
            LEFT JOIN filters f ON f.user_id = me.id

            -- additional filtering on computed columns
            WHERE (f.age_min IS NULL OR pf.age >= f.age_min)
            AND (f.age_max IS NULL OR pf.age <= f.age_max)
            AND (f.distance IS NULL OR pf.distance <= f.distance)
            AND (f.zodiac_sign IS NULL OR pf.zodiac_sign = ANY(f.zodiac_sign))
            AND (f.image_count_min IS NULL OR jsonb_array_length(pf.images) >= f.image_count_min)

            ORDER BY pf.distance
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
