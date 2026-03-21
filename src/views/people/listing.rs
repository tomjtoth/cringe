use dioxus::prelude::*;

use crate::{
    models::person::{Decision, Person},
    views::people::person::Person as VPerson,
};

// keep it as POST, otherwise cannot send pos. args
#[post("/api/profiles")]
async fn get_profiles(wants: Option<Decision>) -> Result<Vec<Person>> {
    #[cfg(feature = "server")]
    {
        use crate::state::server::{get_db, get_session_id};

        if let Some(sess_id) = get_session_id().await {
            let pool = get_db().await;

            let profiles = sqlx::query_as::<_, Person>(&format!(
                r#"
                WITH me AS (
                    SELECT 
                        u.id, gps_lon, gps_lat
                        -- TODO: expand with other settings later for filtering, such as distance, age_min, age_max
                    FROM auth_sessions a
                    JOIN users u on a.email = u.email
                    WHERE a.id = $1 AND expires_at > NOW() AND csrf_token IS NULL
                )

                SELECT
                    u.id,
                    name,
                    gender,
                    born,
                    height,
                    education,
                    occupation,
                    location,
                    hometown,

                    age_from_dob(born) as age,
                    distance_km(
                        u.gps_lat, u.gps_lon, 
                        me.gps_lat, me.gps_lon
                    ) as distance,

                    seeking,
                    relationship_type,

                    json_build_object(
                        'has',      kids_has,
                        'wants',    kids_wants
                    ) AS kids,

                    json_build_object(
                        'drinking',     habits_drinking,
                        'smoking',      habits_smoking,
                        'marijuana',    habits_marijuana,
                        'drugs',        habits_drugs
                    ) AS habits,

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
                            json_agg(row_to_json(up) ORDER BY up.position),
                            '[]'
                        )
                        FROM user_pictures up
                        WHERE up.user_id = u.id
                    ) AS pictures

                FROM users u
                CROSS JOIN me
                LEFT JOIN user_decisions d ON d.actor_user_id = me.id AND d.target_user_id = u.id
                WHERE u.id <> me.id
                AND d.decision {}
                ORDER BY distance DESC
                "#,
                if wants.is_some() { " = $2" } else { "IS NULL" }
            ))
            .bind(&sess_id)
            .bind(&wants)
            .fetch_all(&pool)
            .await?;

            return Ok(profiles);
        }
    }

    Ok(vec![])
}

#[component]
pub fn LikedProfiles() -> Element {
    list(Some(Decision::Like), false)
}

#[component]
pub fn SkippedProfiles() -> Element {
    list(Some(Decision::Skip), false)
}

#[component]
pub fn SwipeProfiles() -> Element {
    list(None, true)
}

fn list(wants: Option<Decision>, swiping: bool) -> Element {
    let profiles = use_server_future(move || async move { get_profiles(wants).await })?;

    rsx! {
        if let Some(Ok(peeps)) = profiles().to_owned() {
            ul {
                class: "h-full overflow-y-scroll p-2 pb-0",
                class: if swiping { "[&_>_*+*]:hidden" },

                for person in peeps {
                    if let Some(id) = person.id {
                        li { key: "{id}",
                            VPerson { person }
                        }
                    }
                }
            }
        }
    }
}
