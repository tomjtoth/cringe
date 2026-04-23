use std::str::FromStr;

use chrono::{Local, Months, NaiveDate};
use dioxus::prelude::*;

use crate::{
    models::{Gender, Profile},
    state::{AUTH_CTE, ME},
    views::people::profile::details::{GenderSelect, HeightInput, NameInput},
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

const BOROMIR: Asset = asset!("./Boromir.jpg", ImageAssetOptions::new().with_avif());

#[component]
pub fn CoreData() -> Element {
    let legal = legal_dob();

    let mut name = use_signal(String::new);
    let mut gender = use_signal(|| Gender::Male);
    let mut birthday = use_signal(|| Some(legal));
    let mut height = use_signal(|| None::<u8>);
    let mut age_confirmed = use_signal(|| false);

    let on_name_change = use_callback(move |s| name.set(s));
    let on_height_change = use_callback(move |s: String| height.set(s.parse::<u8>().ok()));
    let on_gender_change = use_callback(move |s: String| {
        if let Some(g) = Gender::from_str(&s) {
            gender.set(g);
        }
    });

    let required = true;
    let age = birthday.with(|opt| {
        opt.map(|bd| legal.years_since(bd).map(|y| y + 18))
            .flatten()
    });
    let too_young = age.filter(|age| *age >= 18).is_none();

    let span_cls = "absolute left-1/2 -translate-x-1/2";

    rsx! {
        div { class: "relative h-full overflow-scroll",
            form {
                class: "app-center flex flex-col gap-2 items-center",

                onsubmit: move |evt| async move {
                    evt.prevent_default();

                    if name.read().len() > 0 {
                        if let (Some(bd), Some(h)) = (birthday(), height()) {
                            if let Ok(Some(me)) = post_basics(name(), gender(), bd, h)
                                .await
                            {
                                ME.write().profile = Some(me);
                            }
                        }
                    }
                },

                h2 { "Your core data" }

                NameInput { value: name(), onchange: on_name_change }

                GenderSelect { value: gender(), onchange: on_gender_change }

                input {
                    required,
                    r#type: "date",
                    value: birthday.with(|bd| bd.map(|bd| bd.to_string())),
                    max: legal.to_string(),
                    onchange: move |evt| {
                        birthday.set(NaiveDate::from_str(&evt.value()).ok());
                        age_confirmed.set(false);
                    },
                }

                HeightInput { value: height(), onchange: on_height_change }

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
                        div { class: "relative overflow-hidden uppercase text-nowrap text-sm sm:text-xl md:text-2xl lg:text-3xl xl:text-4xl 2xl:text-5xl 3xl:text-6xl text-shadow-[0_0_1px,0_0_2px,0_0_3px,0_0_4px,0_0_5px] text-amber-100 text-shadow-amber-900 max-sm:tracking-tighter max-md:tracking-tight",
                            img { class: "min-w-50 object-cover", src: BOROMIR }
                            span { class: "{span_cls} top-1/30", "one does not simply" }
                            span { class: "{span_cls} bottom-1/30", "cringe without a birthday" }
                        }
                    }

                    input {
                        r#type: "checkbox",
                        hidden: too_young,
                        disabled: too_young,
                        required,
                        checked: age_confirmed(),
                        onchange: move |evt| age_confirmed.set(evt.checked()),
                    }
                }

                button { class: if too_young { "text-gray-500 cursor-not-allowed!" }, "💾 Save" }
            }
        }
    }
}
