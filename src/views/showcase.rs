use dioxus::prelude::*;

use crate::{
    components::{
        login::Login,
        modal::{TrModal, MODALS},
    },
    views::listing::LCX,
};

#[get("/api/showcase")]
async fn get_bot_pics() -> Result<()> {
    Ok(())
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
