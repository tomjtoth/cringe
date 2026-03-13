use dioxus::prelude::*;

use crate::{models::person::Pic, views::people::person::container::Container};

#[component]
pub fn Image(src: Option<Pic>) -> Element {
    rsx! {
        if let Some(pic) = src {
            Container {
                img {
                    class: "object-cover max-md:w-full md:h-full",
                    src: match pic {
                        Pic::Url(src) => src.clone(),
                        Pic::Advanced { url, .. } => url.clone(),
                    },
                    alt: "broken link to picture",

                }
            }
        }
    }
}
