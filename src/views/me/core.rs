use std::str::FromStr;

use chrono::{Local, Months, NaiveDate};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    models::{Gender, Profile},
    state::{AUTH_CTE, ME},
};

// TODO: move this into WS, so that we can notify users of a new profile
#[post("/api/me")]
async fn post_basics(
    name: String,
    sex: Gender,
    dob: NaiveDate,
    height: u8,
) -> Result<Option<Profile>> {
    info!(
        r#"ℹ️  Attempting to insert "{}", "{}", "{}", "{}" into users"#,
        name, sex, dob, height
    );

    let mut res = None;

    if let Some(age) = legal_dob().years_since(dob) {
        debug!(r#"✅ Is above legal age ({} years old)"#, 18 + age);
        if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
            res = sqlx::query_as::<_, Profile>(&format!(
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
pub fn CoreData() -> Element {
    let legal = legal_dob();

    let mut name = use_signal(String::new);
    let mut gender = use_signal(|| Gender::Male);
    let mut birthday = use_signal(|| Some(legal));
    let mut height = use_signal(|| 180u8);
    let mut age_confirmed = use_signal(|| false);

    let required = true;
    let age = birthday.with(|opt| {
        opt.map(|bd| legal.years_since(bd).map(|y| y + 18))
            .flatten()
    });
    let too_young = age.filter(|age| *age >= 18).is_none();

    rsx! {
        form {
            class: "app-center flex flex-col gap-2 items-center",

            onsubmit: move |evt| async move {
                evt.prevent_default();

                if name.read().len() > 0 {
                    if let Some(bd) = birthday() {
                        if let Ok(Some(me)) = post_basics(name(), gender(), bd, height())
                            .await
                        {
                            ME.write().profile = Some(me);
                        }
                    }
                }
            },

            h2 { "Your core data" }

            input {
                placeholder: "Your firstname",
                class: "placeholder:text-center w-40 text-center",
                value: name,
                minlength: 2,
                required,

                onchange: move |evt| *name.write() = evt.value(),
            }

            select {
                required,
                onchange: move |evt| {
                    if let Some(g) = Gender::from_str(&evt.value()) {
                        *gender.write() = g;
                    }
                },

                option { value: "", disabled: true, "Your gender" }
                for gender in Gender::iter() {
                    option { value: "{gender}", "{gender}" }
                }
            }

            input {
                required,
                r#type: "date",
                value: birthday.with(|bd| bd.map(|bd| bd.to_string())),
                max: legal.to_string(),
                onchange: move |evt| {
                    *birthday.write() = NaiveDate::from_str(&evt.value()).ok();
                    age_confirmed.set(false);
                },
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

            p { class: "text-center",
                "These must be always defined, "
                "you can change them later, "
                "with the exception of your "
                b { "date of birth" }
                ", which you "
                b { "cannot change, ever!" }
            }

            label { class: "flex items-center gap-2 text-center",

                if let Some(age) = age {
                    if age < 18 {
                        "I'm too young for this.."
                    } else {
                        "I am {age} years old"
                    }
                } else {
                    "I don't remember when I was born"
                }

                input {
                    r#type: "checkbox",
                    hidden: too_young,
                    disabled: too_young,
                    required,
                    checked: age_confirmed(),
                }
            }

            button {
                class: if too_young { "text-gray-500 cursor-not-allowed!" },
                disabled: too_young,
                "💾 Save"
            }
        }
    }
}
