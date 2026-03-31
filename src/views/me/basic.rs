use std::str::FromStr;

use chrono::{Local, Months, NaiveDate};
use dioxus::prelude::*;

use crate::{
    models::person::{Gender, Person},
    state::client::{AUTH_CTE, ME},
};

#[post("/api/me")]
async fn post_basics(
    name: String,
    sex: Gender,
    dob: NaiveDate,
    height: u8,
) -> Result<Option<Person>> {
    info!(
        r#"ℹ️  Attempting to insert "{}", "{}", "{}", "{}" into users"#,
        name, sex, dob, height
    );

    let mut res = None;

    if let Some(age) = legal_dob().years_since(dob) {
        debug!(r#"✅ Is above legal age ({} years old)"#, 18 + age);
        if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
            res = sqlx::query_as::<_, Person>(&format!(
                r#"
                WITH {AUTH_CTE}

                INSERT INTO users (email, name, gender, born, height)
                    SELECT a.email, $2, $3, $4, $5
                    FROM auth a
                RETURNING *
                "#
            ))
            .bind(&sess_id)
            .bind(&name)
            .bind(&sex)
            .bind(&dob)
            .bind(height as i16)
            .fetch_optional(&pool)
            .await?;
        }
    }

    let [emoji, txt] = if res.is_some() {
        ["✅", "succeeded"]
    } else {
        ["🚫", "failed"]
    };

    info!(r#"{emoji} INSERT {txt}!"#,);

    Ok(res)
}

fn legal_dob() -> NaiveDate {
    let today = Local::now().date_naive();
    today.checked_sub_months(Months::new(18 * 12)).unwrap()
}

#[component]
pub fn BasicMe() -> Element {
    let legal = legal_dob();

    let mut name = use_signal(String::new);
    let mut gender = use_signal(String::new);
    let mut bday = use_signal(|| legal.to_string());
    let mut height = use_signal(|| 180u8);

    let required = true;

    rsx! {
        form {
            class: "app-center flex flex-col gap-2 items-center",

            onsubmit: move |evt| async move {
                evt.prevent_default();

                if let Some(sex) = Gender::from_label(&gender()) {
                    if name().len() > 0 {
                        if let Ok(dob) = NaiveDate::from_str(&bday()) {
                            if let Ok(Some(my_profile)) = post_basics(name(), sex, dob, height())
                                .await
                            {
                                ME.write().profile = Some(my_profile);
                            }
                        }
                    }
                }
            },

            h2 { "HEADS UP!!" }
            p { class: "text-center", "These are not changeable later." }

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
                value: gender(),
                onchange: move |evt| *gender.write() = evt.value(),

                option { value: "", disabled: true, "Your gender" }
                option { value: "{Gender::Male}", "{Gender::Male}" }
                option { value: "{Gender::Female}", "{Gender::Female}" }
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
