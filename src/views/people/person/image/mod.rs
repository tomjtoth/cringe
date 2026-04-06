use dioxus::prelude::*;

use crate::views::people::{
    listing::ListingCtx,
    person::{container::Container, image::editor::ImageEditor, PersonCtx, ResourceCtx},
};

mod editor;

#[component]
pub fn Image(idx: usize) -> Element {
    let olcx = use_context::<Option<ListingCtx>>();
    let rcx = ResourceCtx::provide();

    let (src, show_adder) = {
        let pcx = use_context::<PersonCtx>();
        let person = (pcx.person)();
        let pics = person.images();
        let op = pics.get(idx);

        (op.cloned(), idx == pics.len())
    };

    let ordinal_idx = vec!["🥳 1st", "🎉 2nd", "🎊 3rd"]
        .get(idx)
        .map(|s| s.to_string())
        .unwrap_or(format!("📸 {}th", idx + 1));

    rsx! {
        if let Some(image) = src {
            if rcx.editing() {
                ImageEditor { src: image }
            } else {
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
        } else {
            if olcx.is_none() && show_adder {
                if rcx.editing() {
                    ImageEditor {}
                } else {
                    Container {
                        div { class: "py-20 text-center select-none text-3xl",
                            p { "Add your" }
                            p { "{ordinal_idx} image" }
                            p { class: "mt-5", "😏 YOLO" }
                        }
                    }
                }
            }
        }
    }
}
