use dioxus::prelude::*;

use crate::{
    models::person::Person,
    state::client::{AUTH_CTE, ME},
    views::people::{
        listing::ListingCtx,
        person::{container::Container, PersonCtx, ResourceCtx},
    },
};

mod education;
mod habits;
mod hometown;
mod kids;
mod location;
mod occupation;
mod relationship_type;
mod seeking;

#[put("/api/me")]
async fn update_me(me: Person) -> Result<bool> {
    let mut res = false;

    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
        let kids = me.kids.as_ref();
        let habits = me.habits.as_ref();

        let db_res = sqlx::query(&format!(
            r#"
            WITH {AUTH_CTE}

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
            "#
        ))
        .bind(sess_id)
        .bind(&me.education)
        .bind(&me.occupation)
        .bind(&me.location)
        .bind(&me.hometown)
        .bind(&me.seeking)
        .bind(&me.relationship_type)
        .bind(&kids.map(|k| k.has.map(|n| n as i16)))
        .bind(&kids.map(|k| k.wants.map(|n| n as i16)))
        .bind(&habits.map(|h| h.drinking))
        .bind(&habits.map(|h| h.smoking))
        .bind(&habits.map(|h| h.marijuana))
        .bind(&habits.map(|h| h.drugs))
        .execute(&pool)
        .await?;

        res = db_res.rows_affected() > 0;
    }

    Ok(res)
}

#[component]
pub fn Details() -> Element {
    let olcx = use_context::<Option<ListingCtx>>();
    let person = use_context::<PersonCtx>().person;
    let rcx = use_context::<ResourceCtx>();

    let sig = use_signal(|| person());

    let onsubmit = use_callback({
        let rcx = rcx.clone();

        move |_: Event<FormData>| {
            spawn({
                let mut rcx = rcx.clone();

                async move {
                    rcx.next_state();

                    if let Ok(true) = update_me({
                        let mut wo_prompts_and_images = sig();
                        wo_prompts_and_images.prompts.truncate(0);
                        wo_prompts_and_images.images.truncate(0);

                        wo_prompts_and_images
                    })
                    .await
                    {
                        ME.with_mut(|me| me.profile = Some(sig()));
                    }

                    rcx.next_state();
                }
            });
        }
    });

    let values_under_ul = [
        sig.read().occupation.is_some(),
        sig.read().education.is_some(),
        sig.read().hometown.is_some(),
        sig.read().seeking.is_some(),
        sig.read().relationship_type.is_some(),
    ]
    .into_iter()
    .filter(|&x| x)
    .count();

    let editing = olcx.is_none() && rcx.editing();

    let class_container = format!(
        "px-2 [&>*+*]:border-t [&>*+*]:p-2 {} {} {}{}{}",
        "[&_input]:border-none! [&>div>input]:w-full",
        "[&_select]:border-none! [&>div>select]:px-0! [&>div>select]:w-full",
        "[&>div]:flex [&>div]:gap-2 [&>div]:items-center",
        if editing {
            " [&>div]:nth-last-2:mb-20"
        } else {
            ""
        },
        if olcx.is_none() && values_under_ul == 0 {
            // the edit button has bottom-5 and its top border is not even visible,
            // overriding from here to complicate things less (?)
            " [&>button]:nth-2:bottom-2!"
        } else {
            ""
        }
    );

    let class_ul = format!(
        "p-2 flex overflow-x-scroll text-nowrap {} {}{}",
        "[&>*+*]:ml-2 [&>*+*]:border-l *:p-2",
        "[&>li]:flex [&>li]:gap-2 [&>li]:items-center",
        if olcx.is_none() && !rcx.editing() && values_under_ul == 0 {
            " [&>li:last-child]:mr-15"
        } else {
            ""
        }
    );

    rsx! {
        Container { class: class_container, wo_button: olcx.is_some(), onsubmit,
            ul { class: class_ul,

                if let Some(age) = &sig.read().age() {
                    li { "🎂 {age}" }
                }

                if let Some(dist) = &sig.read().distance() {
                    li { "{dist}" }
                }

                li { "{sig.read().gender}" }

                li { "📏 {sig.read().height} cm" }

                location::Location { sig, editing }

                kids::Kids { sig, editing }

                // TODO: include pets here
                //
                if let Some(sign) = sig.read().zodiac_sign() {
                    li { "{sign}" }
                }

                habits::Habits { sig, editing }

            }

            occupation::Occupation { sig, editing }

            education::Education { sig, editing }

            hometown::Hometown { sig, editing }

            seeking::Seeking { sig, editing }

            relationship_type::RelationshipType { sig, editing }
        }
    }
}
