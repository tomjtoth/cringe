use dioxus::prelude::*;

use crate::views::listing::ListingCtx;

#[get("/api/showcase")]
async fn get_bot_pics() -> Result<()> {
    Ok(())
}

#[component]
pub fn Showcase(children: Element) -> Element {
    let mut lcx = use_context::<ListingCtx>();
    use_effect(move || lcx.set(None));

    let semver = env!("CARGO_PKG_VERSION");

    rsx! {
        h1 {
            "😬 Cringe "
            sup { "{semver}" }
        }

        {children}
    }
}
