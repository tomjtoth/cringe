use dioxus::prelude::*;

use crate::{
    models::person::Person as MPerson,
    views::people::{
        listing::ListingCtx,
        person::{button::SkipButton, details::Details},
    },
};

mod button;
mod container;
mod details;
mod image;
mod prompt;
mod utils;

use image::Image;
use prompt::Prompt;

#[derive(Clone)]
struct PersonCtx {
    person: ReadSignal<MPerson>,
}

/// This might be a Prompt, an Image or the whole Personal data section
#[derive(Clone)]
struct ResourceCtx {
    state: Signal<u8>,
}

impl ResourceCtx {
    fn provide() -> Self {
        let state = use_signal(|| 0);
        use_context_provider(|| ResourceCtx { state })
    }

    fn editing(&self) -> bool {
        *(self.state).read() >= 1
    }

    fn next_state(&mut self) {
        let curr = (self.state)();
        *(self.state).write() = if curr == 2 { 0 } else { curr + 1 };
    }
}

#[component]
pub fn Person(person: ReadSignal<MPerson>) -> Element {
    let olcx = use_context::<Option<ListingCtx>>();

    use_context_provider(move || PersonCtx { person });

    // for the SkipButton and PersonalData
    let _rcx = ResourceCtx::provide();

    rsx! {
        div {
            class: format!(
                "{} {}",
                "m-0! mr-0 p-2 sticky z-2 top-0 bg-background",
                "flex justify-between items-center",
            ),

            span { class: "text-2xl", "{person().name}" }

            if olcx.is_none() {
                a {
                    class: "border rounded p-2 cursor-pointer select-none",
                    href: "/logout",
                    "logout [➜"
                }
            }
        }

        div { class: "relative md:columns-2 lg:columns-3 *:mb-2 text-lg",

            Image { idx: 0 }
            Prompt { idx: 0 }

            Details {}

            Image { idx: 1 }
            Prompt { idx: 1 }

            Image { idx: 2 }
            Prompt { idx: 2 }

            Image { idx: 3 }
            Prompt { idx: 3 }

            Image { idx: 4 }
            Prompt { idx: 4 }

            Image { idx: 5 }
            Prompt { idx: 5 }

        }

        if olcx.is_some() {
            SkipButton {}
        }

    }
}
