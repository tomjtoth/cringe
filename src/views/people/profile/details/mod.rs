use dioxus::prelude::*;

use crate::{
    models::person::Person,
    state::{AUTH_CTE, ME},
    views::people::{
        listing::ListingCtx,
        profile::{container::Container, ProfileCtx, ResourceCtx},
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
    let person = use_context::<ProfileCtx>().profile;
    let mut rcx = use_context::<ResourceCtx>();
    let wscx = use_context::<WsCtx>();
    let mut editing = use_signal(|| false);
    let sig = use_signal(|| person());

    use_context_provider(|| DetailsCtx {
        rw: sig,
        ro: person,
        editing,
    });

    use_effect(move || editing.set(olcx.is_none() && rcx.editing()));

    use_effect(move || rcx.await_op());

    let onsubmit = use_callback(move |_: Event<FormData>| {
        spawn(async move {
            // sending without images or prompts to save bandwidth
            let mut me = sig();
            me.prompts.truncate(0);
            me.images.truncate(0);

            _ = wscx
                .req(crate::state::websocket::WsRequest::DetailsUpdate(
                    rcx.op_id, me,
                ))
                .await;
        });
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

    let class_container = format!(
        "px-2 [&>*+*]:border-t [&>*+*]:p-2 {} {} {}{}{}",
        "[&_input]:border-none! [&>div>input]:w-full",
        "[&_select]:border-none! [&>div>select]:px-0! [&>div>select]:w-full",
        "[&>div]:flex [&>div]:gap-2 [&>div]:items-center",
        if editing() {
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

    let immutables = "cursor-not-allowed select-none text-gray-500";

    rsx! {
        Container { class: class_container, wo_button: olcx.is_some(), onsubmit,
            ul { class: class_ul,

                if let Some(age) = &sig.read().age() {
                    li { class: if editing() { immutables }, "🎂 {age}" }
                }

                if let Some(dist) = &sig.read().distance() {
                    li { class: if editing() { immutables }, "{dist}" }
                }

                li { class: if editing() { immutables }, "{sig.read().gender}" }

                li { class: if editing() { immutables }, "📏 {sig.read().height} cm" }

                location::Location {}

                kids::Kids {}

                // TODO: include pets here
                //
                if let Some(sign) = sig.read().zodiac_sign() {
                    li { class: if editing() { "cursor-not-allowed" }, "{sign}" }
                }

                habits::Habits {}

            }

            occupation::Occupation {}

            education::Education {}

            hometown::Hometown {}

            seeking::Seeking {}

            relationship_type::RelationshipType {}
        }
    }
}
