use std::str::FromStr;

use chrono::{Local, Months, NaiveDate};
use dioxus::prelude::*;

use crate::{
    models::person::{Gender, Person},
    state::client::ME,
};

#[post("/api/me")]
async fn post_basics(
    name: String,
    sex: Gender,
    dob: NaiveDate,
    height: u8,
) -> Result<Option<Person>> {
    if dob.years_since(legal_age()).is_some() {
        #[cfg(feature = "server")]
        {
            use crate::state::server::{get_db, get_session_id};

            if let Some(sess_id) = get_session_id().await {
                let pool = get_db().await;

                let profile = sqlx::query_as::<_, Person>(
                    r#"
                INSERT INTO users (name, email, gender, born, height)
                    SELECT $2, a.email, $3, $4, $5
                    FROM auth_sessions a
                    WHERE id = $1 AND expires_at > NOW() AND csrf_token IS NULL
                RETURNING *
                "#,
                )
                .bind(&sess_id)
                .bind(&name)
                .bind(&sex)
                .bind(&dob)
                .bind(height as i16)
                .fetch_optional(&pool)
                .await?;

                return Ok(profile);
            }
        }
    }

    Ok(None)
}

fn legal_age() -> NaiveDate {
    let today = Local::now().date_naive();
    today.checked_sub_months(Months::new(18 * 12)).unwrap()
}

#[component]
pub fn BasicMe() -> Element {
    let legal = legal_age();

    let mut name = use_signal(|| String::new());
    let mut gender = use_signal(|| Gender::Male);
    let mut bday = use_signal(|| legal.to_string());
    let mut height = use_signal(|| 180u8);

    let onsubmit = move |evt: Event<FormData>| async move {
        // should I actually? this could simply reload the page
        evt.prevent_default();

        if name().len() > 0 {
            if let Ok(dob) = NaiveDate::from_str(&bday()) {
                if let Ok(Some(my_profile)) = post_basics(name(), gender(), dob, height()).await {
                    *ME.write() = Some(Some(my_profile))
                }
            }
        }
    };

    let required = true;

    rsx! {
        form {
            onsubmit,
            class: "absolute top-1/2 left-1/2 -translate-1/2
                    flex flex-col gap-2 items-center",

            h2 { "HEADS UP!!" }
            p { class: "text-center", "These props are not changeable later." }

            input {
                placeholder: "Your Name Here",
                class: "placeholder:text-center w-40 text-center",
                value: name,
                minlength: 2,
                required,

                onchange: move |evt| *name.write() = evt.value(),
            }

            select {
                required,
                value: gender() as u8,
                onchange: move |evt| *gender.write() = Gender::from_numerical_str(evt.value()),

                option { value: "", disabled: true, "Your gender" }
                option { value: Gender::Male as u8, "{Gender::Male}" }
                option { value: Gender::Female as u8, "{Gender::Female}" }
            }

            input {
                required,
                r#type: "date",
                value: bday,
                max: legal.to_string(),
                onchange: move |evt| *bday.write() = evt.value(),
            }

            input {
                required,
                r#type: "number",
                placeholder: "📏 height",
                class: "w-30",
                min: u8::MIN,
                max: u8::MAX,

                value: height,
                onchange: move |evt| {
                    if let Ok(h) = evt.value().parse::<u8>() {
                        *height.write() = h;
                    }
                },
            }

            button { "💾 Save" }
        }

    }
}
