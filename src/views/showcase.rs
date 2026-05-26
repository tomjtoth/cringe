use dioxus::prelude::*;

use crate::{
    components::{
        login::Login,
        modal::{TrModal, MODALS},
    },
    views::listing::LCX,
};

#[get("/api/showcase")]
async fn get_bots() -> Result<Vec<Profile>> {
    let (_, pool) = crate::state::server::get_ctx().await;
    let res: Vec<Profile> = sqlx::query_as(
        "SELECT
            u.id,
            u.name,
            u.gender,
            u.height,
            age_from_dob(u.born) as age,
            jsonb_build_array(to_jsonb(ui)) AS images
        FROM users u
        INNER JOIN user_images ui ON ui.user_id = u.id
        WHERE ai_personality IS NOT NULL
        AND ui.position = 0
        ",
    )
    .fetch_all(&pool)
    .await?;

    Ok(res)
}

#[component]
pub fn Showcase(children: Element, hide_login: Option<bool>) -> Element {
    use_effect(move || LCX.with_mut(|cx| *cx = None));

    let semver = env!("CARGO_PKG_VERSION");

    rsx! {
        div {
            class: "flex justify-between items-center",
            class: "p-2 lg:p-6 2xl:p-10",
            class: "text-xl lg:text-2xl 2xl:text-3xl",

            span {
                "😬 Cringe "
                sup { "{semver}" }
            }

            if hide_login != Some(true) {

                button {
                    class: "text-[0.6em]",
                    onclick: move |_| MODALS.new("z-10", true, rsx! {
                        Login {}
                    }),

                    "Login ➜]"
                }
            }
        }

        {children}
    }
}
