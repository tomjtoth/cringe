use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let class = format!(
        "{} {} {} {}",
        "p-4 flex max-md:flex-col flex-wrap items-stretch gap-2 **:p-2",
        "[&_img]:w-14 [&_img]:inline",
        "[&_a]:flex [&_a]:justify-between [&_a]:items-center",
        "[&_>_li]:min-w-40 [&_>_li]:border [&_>_li]:rounded",
    );

    rsx! {
        div { class: "absolute left-1/2 top-1/2 -translate-1/2 border rounded",

            ul { class,

                li {
                    a { href: "/api/auth/google",
                        "Google"
                        img { src: "https://authjs.dev/img/providers/google.svg" }
                    }
                }

                li {
                    a { href: "/api/auth/discord",
                        "Discord"
                        img { src: "https://authjs.dev/img/providers/discord.svg" }
                    }
                }

                li {

                    a { href: "/api/auth/github",
                        "GitHub"
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
