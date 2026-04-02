use dioxus::prelude::*;

use crate::views::people::{
    listing::ListingCtx,
    person::{container::Container, PersonCtx, ResourceCtx},
};

#[component]
pub fn Image(idx: usize) -> Element {
    let _olcx = use_context::<Option<ListingCtx>>();

    let (src, _show_adder) = {
        let pcx = use_context::<PersonCtx>();
        let person = (pcx.person)();
        let pics = person.images();
        let op = pics.get(idx);

        (op.cloned(), idx == pics.len())
    };

    let _rcx = ResourceCtx::provide();

    rsx! {
        if let Some(image) = src {
            Container {
                if let Some(prompt) = image.prompt() {
                    p { class: "p-2 py-4 text-2xl",

                        sub {
                            class: "pr-2 text-4xl select-none",
                            style: "font-family: 'Times New Roman',serif;font-size:36px;",
                            "”"
                        }

                        "{prompt}"
                    }
                }

                img { class: "object-cover w-full", src: image.src() }
            }
        }
    }
}
