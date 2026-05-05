use dioxus::prelude::*;

#[get("/api/showcase")]
async fn get_bot_pics() -> Result<()> {
    Ok(())
}

#[component]
pub fn Showcase(children: Element) -> Element {
    let semver = env!("CARGO_PKG_VERSION");

    rsx! {
        h1 {
            "Cringe "
            sup { "{semver}" }
        }

        {children}
    }
}
