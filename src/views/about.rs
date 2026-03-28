use dioxus::prelude::*;

#[get("/api/semver")]
async fn get_semver() -> Result<String> {
    Ok(std::env::var("APP_VER").unwrap_or("D.E.V".to_string()))
}

#[component]
pub fn About() -> Element {
    let semver = use_server_future(get_semver)?;

    rsx! {
        h1 {
            "Cringe "

            if let Some(Ok(triplet)) = semver() {
                sup { "{triplet}" }
            }
        }

        p { class: "p-2",
            "This is a "
            b { "Work-in-Progress" }
            " Hinge clone. "
            b { "Expect data loss" }
            " below version 1.0.0! Check out the source code "
            a {
                class: "pre-preflight",
                href: "https://github.com/tomjtoth/cringe",
                target: "_blank",
                "here"
            }
            "."
        }

    }
}
