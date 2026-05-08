use dioxus::prelude::*;

use crate::views::listing::LCX;

#[get("/api/showcase")]
async fn get_bot_pics() -> Result<()> {
    Ok(())
}

#[component]
pub fn Showcase(children: Element) -> Element {
    use_effect(move || LCX.with_mut(|cx| *cx = None));

    let semver = env!("CARGO_PKG_VERSION");

    rsx! {
        h1 {
            "😬 Cringe "
            sup { "{semver}" }
        }

        {children}
    }
}
