use dioxus::prelude::*;

use crate::{
    components::{
        login::Login,
        modal::{TrModal, MODALS},
    },
    models::Profile,
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

    let bots = use_server_future(get_bots)?;

    let shadows = "text-shadow-background text-shadow-[0_0_5px,0_0_4px,0_0_3px,0_0_2px,0_0_1px]/90";

    rsx! {
        div {
            class: "flex justify-between items-center relative",
            class: "p-2 lg:p-6 2xl:p-10",
            class: "text-xl lg:text-2xl 2xl:text-3xl",

            span { class: "z-1 {shadows}",
                "😬 Cringe "
                sup { "{semver}" }
            }

            if hide_login != Some(true) {

                button {
                    class: "z-1 text-[0.6em] bg-background",
                    onclick: move |_| MODALS.new("z-10", true, rsx! {
                        Login {}
                    }),

                    b { "Login ➜]" }
                }
            }

            if let Some(Ok(bots)) = bots() {
                div {
                    class: "absolute top-0 left-0 h-full {shadows}",
                    class: "w-16/10 transform-[rotate(20deg)_translate(3%,-50%)]",
                    class: "lg:w-12/10 lg:transform-[rotate(20deg)_translate(0%,-200%)]",

                    ul {
                        class: "columns-3 lg:columns-4 2xl:columns-5",
                        class: "[&>*:not(:first-child)]:mt-4 select-none",

                        for bot in bots {
                            if let Some(img) = bot.images.get(0) {
                                li { class: "border rounded-2xl overflow-hidden relative",
                                    img {
                                        class: "select-none object-cover w-full",
                                        src: img.src(),
                                    }

                                    span { class: "absolute bottom-2 left-2",
                                        b { "{bot.name}" }
                                        span { class: "text-[0.8em]", " {bot.age.unwrap()}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        div { class: "app-center z-1 text-center max-h-4/5 max-w-9/10 overflow-y-scroll {shadows} text-[1.5em]",
            {children}
        }
    }
}
