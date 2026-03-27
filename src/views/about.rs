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
            b { "Work-in-Progress & currently insecure" }
            " Hinge clone. Check out the source code "
            a { href: "https://github.com/tomjtoth/cringe", target: "_blank", "here" }
            "."
        }

        // p {
        //     b { "This app is invitation-only. There is no automated verification process. " }
        //     "Instead you invite your friends and moderate their photos. "
        //     "If your direct inviter deletes their account, their inviter will moderate your pics. "
        // }
    }
}
