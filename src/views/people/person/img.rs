use dioxus::prelude::*;

use crate::{models::person::Pic, views::people::person::container::Container};

#[component]
pub fn Image(src: Option<Pic>, id: String) -> Element {
    rsx! {
        if let Some(pic) = src {
            Container { id,

                if let Pic::Advanced { prompt: Some(prompt), .. } = &pic {
                    p { class: "p-2 py-4 text-2xl",

                        sub {
                            class: "pr-2 text-4xl",
                            style: "font-family: 'Times New Roman',serif;font-size:36px;",
                            "”"
                        }
                        "{prompt}"
                    }
                }

                img {
                    class: "object-cover w-full",
                    src: match pic {
                        Pic::Url(src) => src,
                        Pic::Advanced { url, .. } => url,
                    },
                }
            }
        }
    }
}
