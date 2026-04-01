use dioxus::prelude::*;

use crate::{
    models::person::{Frequency, Person, RelationshipType, Seeking},
    state::client::{AUTH_CTE, ME},
    views::people::{
        listing::ListingCtx,
        person::{container::Container, PersonCtx, ResourceCtx},
    },
};

#[cfg(not(feature = "server"))]
use crate::models::person::{Habits, Kids};

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
fn FF(sig: Signal<Person>) -> Element {
    rsx! {}
}

#[component]
pub fn PersonalData() -> Element {
    let olcx = use_context::<Option<ListingCtx>>();
    let person = use_context::<PersonCtx>().person;
    let rcx = use_context::<ResourceCtx>();

    let mut sig = use_signal(|| person());

    use_resource({
        let rcx = rcx.clone();

        move || {
            let mut rcx = rcx.clone();

            async move {
                if rcx.submitting() {
                    if let Ok(true) = update_me({
                        let mut wo_prompts_and_images = sig();
                        wo_prompts_and_images.prompts.truncate(0);
                        wo_prompts_and_images.images.truncate(0);

                        wo_prompts_and_images
                    })
                    .await
                    {
                        rcx.next_state();

                        ME.with_mut(|me| me.profile = Some(sig()));

                        return;
                    }

                    rcx.next_state();
                }
            }
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

    let already_has_kids = sig.read().kids.as_ref().is_some_and(|k| k.has > Some(0));

    let class_container = format!(
        "px-2 [&>*+*]:border-t [&>*+*]:p-2 {} {} {}{}",
        "[&_input]:border-none! [&>div>input]:w-full [&>div]:nth-last-2:mb-20",
        "[&_select]:border-none! [&>div>select]:px-0! [&>div>select]:w-full",
        "[&>div]:flex [&>div]:gap-2 [&>div]:items-center",
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
        Container { class: class_container, wo_button: olcx.is_some(),
            ul { class: class_ul,

                if let Some(age) = &sig.read().age() {
                    li { "🎂 {age}" }
                }

                if let Some(dist) = &sig.read().distance() {
                    li { "{dist}" }
                }

                li { "{sig.read().gender}" }

                li { "📏 {sig.read().height} cm" }

                if editing {
                    li {
                        "📍"
                        input {
                            placeholder: "Location",
                            class: "w-30",
                            value: sig.read().location.clone(),
                            onchange: move |evt| {
                                let loc = evt.value();
                                sig.write().location = if loc.len() > 0 { Some(loc) } else { None };
                            },
                        }
                    }
                } else {
                    if let Some(city) = &sig.read().location {
                        li { "📍 {city}" }
                    }
                }

                if editing {
                    li {
                        "🧑‍🧒‍🧒"
                        input {
                            placeholder: "# of kids you have",
                            r#type: "tel",
                            min: 0,
                            max: i8::MAX,
                            value: sig.read().kids.as_ref().map(|k| k.has),
                            onchange: move |evt| {
                                sig.with_mut(|p| {
                                    let has = evt.value().parse::<i8>().ok();
                                    if let Some(kids) = p.kids.as_mut() {
                                        kids.has = has;
                                    } else {
                                        #[cfg(not(feature = "server"))]
                                        {
                                            p.kids = Some(Kids { has, wants: None });
                                        }
                                    }
                                });
                            },
                        }
                    }

                    li {
                        "🍼"
                        select {
                            class: if sig.read().kids.as_ref().map(|k| k.wants).unwrap_or(None) == None { "text-gray-500" },
                            value: sig.read().kids.as_ref().map(|k| k.wants),
                            onchange: move |evt| {
                                sig.with_mut(|p| {
                                    let wants = evt.value().parse::<i8>().ok();
                                    if let Some(kids) = p.kids.as_mut() {
                                        kids.wants = wants;
                                    } else {
                                        #[cfg(not(feature = "server"))]
                                        {
                                            p.kids = Some(Kids {
                                                wants,
                                                ..Default::default()
                                            });
                                        }
                                    }
                                });
                            },

                            option { value: "",
                                "# of"

                                if already_has_kids {
                                    " additional"
                                }

                                " kids you want"
                            }
                            option { value: -1,
                                "I don't know if I want any"

                                if already_has_kids {
                                    " more"
                                }

                                " kids"
                            }
                            option { value: 0,
                                "I don't want any"

                                if already_has_kids {
                                    " more"
                                }

                                " kids"
                            }

                            for n in 1..i8::MAX {
                                option { value: n,
                                    "I want {n}"

                                    if already_has_kids {
                                        " more"
                                    }

                                    " kids"
                                }
                            }

                            option { value: i8::MAX, "I want {i8::MAX} **or more** kids" }
                        }
                    }
                } else {
                    if let Some(kids) = sig.read().kids.as_ref() {
                        if let Some(has) = kids.has {
                            li {
                                "🧑‍🧒‍🧒 "
                                if has > 0 {
                                    "Has {has}"
                                } else {
                                    "No"
                                }
                                " kids"
                            }
                        }

                        if let Some(wants) = kids.wants {
                            li {
                                "🍼 "
                                if wants > 0 {
                                    "Wants {wants}"
                                } else if wants == 0 {
                                    "Doesn't want"
                                } else {
                                    "Doesn't know if wants any"
                                }

                                if wants == i8::MAX {
                                    b { " or more" }
                                } else {
                                    if already_has_kids {
                                        " more"
                                    }
                                }
                                " kids"
                            }
                        }
                    }
                }

                // TODO: include pets here
                //
                if let Some(sign) = sig.read().zodiac_sign() {
                    li { "{sign}" }
                }

                if editing {
                    li {
                        "🍷"

                        select {
                            class: if sig.read().habits.as_ref().map(|h| h.drinking).unwrap_or(None) == None { "text-gray-500" },
                            value: sig.read()
                                .habits
                                .as_ref()
                                .map(|h| h.drinking.as_ref().map(|d| d.to_string()).unwrap_or("".to_string()))
                                .unwrap_or("".to_string()),

                            onchange: move |evt| {
                                sig.with_mut(|p| {
                                    let drinking = Frequency::from_str(&evt.value());
                                    if let Some(habits) = p.habits.as_mut() {
                                        habits.drinking = drinking;
                                    } else {
                                        #[cfg(not(feature = "server"))]
                                        {
                                            p.habits = Some(Habits {
                                                drinking,
                                                ..Default::default()
                                            });
                                        }
                                    }
                                });
                            },

                            option { value: "", "Alcohol?" }
                            option { value: "{Frequency::Never}", "{Frequency::Never}" }
                            option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                            option { value: "{Frequency::Often}", "{Frequency::Often}" }
                            option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                        }
                    }

                    li {
                        "🚬"

                        select {
                            class: if sig.read().habits.as_ref().map(|h| h.smoking).unwrap_or(None) == None { "text-gray-500" },
                            value: sig.read()
                                .habits
                                .as_ref()
                                .map(|h| h.smoking.as_ref().map(|d| d.to_string()).unwrap_or("".to_string()))
                                .unwrap_or("".to_string()),

                            onchange: move |evt| {
                                sig.with_mut(|p| {
                                    let smoking = Frequency::from_str(&evt.value());
                                    if let Some(habits) = p.habits.as_mut() {
                                        habits.smoking = smoking;
                                    } else {
                                        #[cfg(not(feature = "server"))]
                                        {
                                            p.habits = Some(Habits {
                                                smoking,
                                                ..Default::default()
                                            });
                                        }
                                    }
                                });
                            },

                            option { value: "", "Smokes?" }
                            option { value: "{Frequency::Never}", "{Frequency::Never}" }
                            option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                            option { value: "{Frequency::Often}", "{Frequency::Often}" }
                            option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                        }
                    }

                    li {
                        "🌿🚬"

                        select {
                            class: if sig.read().habits.as_ref().map(|h| h.marijuana).unwrap_or(None) == None { "text-gray-500" },
                            value: sig.read()
                                .habits
                                .as_ref()
                                .map(|h| h.marijuana.as_ref().map(|d| d.to_string()).unwrap_or("".to_string()))
                                .unwrap_or("".to_string()),

                            onchange: move |evt| {
                                sig.with_mut(|p| {
                                    let marijuana = Frequency::from_str(&evt.value());
                                    if let Some(habits) = p.habits.as_mut() {
                                        habits.marijuana = marijuana;
                                    } else {
                                        #[cfg(not(feature = "server"))]
                                        {
                                            p.habits = Some(Habits {
                                                marijuana,
                                                ..Default::default()
                                            });
                                        }
                                    }
                                });
                            },

                            option { value: "", "Marijuana?" }
                            option { value: "{Frequency::Never}", "{Frequency::Never}" }
                            option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                            option { value: "{Frequency::Often}", "{Frequency::Often}" }
                            option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                        }
                    }

                    li {
                        "💊💉"

                        select {
                            class: if sig.read().habits.as_ref().map(|h| h.drugs).unwrap_or(None) == None { "text-gray-500" },
                            value: sig.read()
                                .habits
                                .as_ref()
                                .map(|h| h.drugs.as_ref().map(|d| d.to_string()).unwrap_or("".to_string()))
                                .unwrap_or("".to_string()),

                            onchange: move |evt| {
                                sig.with_mut(|p| {
                                    let drugs = Frequency::from_str(&evt.value());
                                    if let Some(habits) = p.habits.as_mut() {
                                        habits.drugs = drugs;
                                    } else {
                                        #[cfg(not(feature = "server"))]
                                        {
                                            p.habits = Some(Habits {
                                                drugs,
                                                ..Default::default()
                                            });
                                        }
                                    }
                                });
                            },

                            option { value: "", "Drugs?" }
                            option { value: "{Frequency::Never}", "{Frequency::Never}" }
                            option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                            option { value: "{Frequency::Often}", "{Frequency::Often}" }
                            option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                        }
                    }
                } else {
                    if let Some(habits) = sig.read().habits.as_ref() {
                        if let Some(drinking) = habits.drinking {
                            li { "🍷 {drinking}" }
                        }

                        if let Some(smoking) = habits.smoking {
                            li { "🚬 {smoking}" }
                        }

                        if let Some(marijuana) = habits.marijuana {
                            li { title: "marijuana", "🌿🚬 {marijuana}" }
                        }

                        if let Some(drugs) = habits.drugs {
                            li { title: "drugs", "💊💉 {drugs}" }
                        }
                    }
                }

            }

            if editing {
                div {
                    "💼"
                    input {
                        placeholder: "Occupation",
                        value: sig.read().occupation.clone(),
                        onchange: move |evt| {
                            let val = evt.value();
                            sig.write().occupation = if val.len() > 0 { Some(val) } else { None };
                        },
                    }
                }
            } else {
                if let Some(job) = &sig.read().occupation {
                    div {
                        "💼"
                        div { "{job}" }
                    }
                }
            }

            if editing {
                div {
                    "🎓"
                    input {
                        placeholder: "Education",
                        value: sig.read().education.clone(),
                        onchange: move |evt| {
                            let val = evt.value();
                            sig.write().education = if val.len() > 0 { Some(val) } else { None };
                        },
                    }
                }
            } else {
                if let Some(edu) = &sig.read().education {
                    div {
                        "🎓"
                        div { "{edu}" }
                    }
                }
            }

            if editing {
                div {
                    "🏠"
                    input {
                        placeholder: "Hometown",
                        value: sig.read().hometown.clone(),
                        onchange: move |evt| {
                            let val = evt.value();
                            sig.write().hometown = if val.len() > 0 { Some(val) } else { None };
                        },
                    }
                }
            } else {
                if let Some(ht) = &sig.read().hometown {
                    div {
                        "🏠"
                        div { "{ht}" }
                    }
                }
            }

            if editing {
                {
                    let value = sig
                        .read()
                        .seeking
                        .as_ref()
                        .map(|s| s.to_string())
                        .unwrap_or("".to_string());

                    rsx! {
                        div {

                            select {
                                class: if value == "" { "text-gray-500" },

                                value,
                                onchange: move |evt| {
                                    sig.write().seeking = Seeking::from_str(&evt.value());
                                },

                                option { value: "", class: "text-gray-500", "🕵️ You're seeking..." }
                                option { value: "{Seeking::LongTerm}", "{Seeking::LongTerm}" }
                                option { value: "{Seeking::LongTermOpenToShort}", "{Seeking::LongTermOpenToShort}" }
                                option { value: "{Seeking::ShortTermFun}", "{Seeking::ShortTermFun}" }
                                option { value: "{Seeking::ShortTermOpenToLong}", "{Seeking::ShortTermOpenToLong}" }
                                option { value: "{Seeking::StillFiguringItOut}", "{Seeking::StillFiguringItOut}" }
                            }
                        }
                    }
                }
            } else {
                if let Some(seeking) = &sig.read().seeking {
                    {
                        let (emoji, text) = seeking.parts();

                        rsx! {
                            div {
                                "{emoji}"
                                div { "{text}" }
                            }
                        }
                    }
                }
            }

            if editing {
                {
                    let value = sig
                        .read()
                        .relationship_type
                        .map(|s| s.to_string())
                        .unwrap_or("".to_string());

                    rsx! {
                        div {
                            select {
                                class: if value == "" { "text-gray-500" },
                                value,
                                onchange: move |evt| {
                                    sig.write().relationship_type = RelationshipType::from_str(&evt.value());
                                },

                                option { value: "", "👩‍❤️‍👨 Your relationship type..." }
                                option { value: "{RelationshipType::FiguringOutMyRelationshipType}",
                                    "{RelationshipType::FiguringOutMyRelationshipType}"
                                }
                                option { value: "{RelationshipType::Monogamy}", "{RelationshipType::Monogamy}" }
                                option { value: "{RelationshipType::NonMonogamy}", "{RelationshipType::NonMonogamy}" }
                            }
                        }
                    }
                }
            } else {
                if let Some(relationship_type) = &sig.read().relationship_type {
                    {
                        let (emoji, text) = relationship_type.parts();

                        rsx! {
                            div {
                                "{emoji}"
                                div { "{text}" }
                            }
                        }
                    }
                }
            }
        }

    }
}
