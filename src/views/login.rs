use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        div { class: "app-center border rounded",

            h2 { class: "text-center", "Login" }

            p { class: "p-2 max-w-60", "Use a provider with a verified email!" }

            ul {
                class: format!(
                    "{} {} {}",
                    "p-4 flex flex-col gap-2",
                    "[&_li]:min-w-40 [&_li]:border [&_li]:rounded",
                    "[&_a]:flex [&_a]:items-center [&_a]:justify-between [&_a]:p-2 [&_img]:w-12",
                ),

                li {
                    a { href: "/api/auth/google",
                        span { "Google" }
                        img { src: "https://authjs.dev/img/providers/google.svg" }
                    }
                }

                li {
                    a { href: "/api/auth/discord",
                        span { "Discord" }
                        img { src: "https://authjs.dev/img/providers/discord.svg" }
                    }
                }

                li {
                    a { href: "/api/auth/github",
                        span { "GitHub" }
                        img {
                            class: "dark:invert",
                            src: "https://authjs.dev/img/providers/github.svg",
                        }
                    }
                }

            }
        }
    }
}
