use dioxus::prelude::*;

use crate::{
    models::person::Pic,
    views::people::person::{container::Container, svg_quote_mark::SvgQuoteMark},
};

#[component]
pub fn Image(src: Option<Pic>, id: String) -> Element {
    rsx! {
        if let Some(pic) = src {
            Container { id,

                if let Pic::Advanced { prompt: Some(prompt), .. } = &pic {
                    SvgQuoteMark { class: "p-1 max-w-8 inline-block fill-foreground" }
                    span { class: "p-2 text-2xl", "{prompt}" }
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
