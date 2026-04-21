use dioxus::prelude::*;

use crate::{
    models::Profile as MPerson,
    state::websocket::ops::{OpState, OPS},
    views::people::{
        listing::ListingCtx,
        profile::{button::SkipButton, details::Details},
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
struct ProfileCtx {
    profile: ReadSignal<MPerson>,
}

/// This might be a Prompt, an Image or the whole Details section
pub(crate) struct ResourceCtx {
    state: Signal<bool>,
    pub(crate) op_id: u8,
}

impl Copy for ResourceCtx {}
impl Clone for ResourceCtx {
    fn clone(&self) -> Self {
        *self
    }
}

impl ResourceCtx {
    fn provide(idx: usize) -> Self {
        let state = use_signal(|| false);
        use_context_provider(|| ResourceCtx {
            state,
            op_id: idx as u8,
        })
    }

    fn editing(&self) -> bool {
        (self.state)()
    }

    pub(crate) fn toggle_editing(&mut self) {
        self.state.toggle();
        debug!("rcx.toggle() -> {}", self.editing());
    }

    pub fn await_op(&mut self) {
        if !self.editing() {
            return;
        }

        let id = self.op_id;

        let returned = OPS.with(|ops| {
            debug!(
                "WS op {}({id}) polled {ops:?}",
                match id {
                    0 => "Details",
                    1..7 => "Prompt",
                    _ => "Image",
                }
            );
            ops.get(&id).cloned()
        });

        if let Some(state) = returned {
            OPS.with_mut(|ops| ops.remove(&id));

            match state {
                OpState::Success => self.toggle_editing(),
                OpState::Failure => {
                    // TODO: show a modal or simple toast
                    error!("WS op #{id} failed");
                }
            };
        }
    }
}

#[component]
pub fn Profile(profile: ReadSignal<MPerson>) -> Element {
    let olcx = use_context::<Option<ListingCtx>>();

    use_context_provider(move || ProfileCtx { profile });

    // for the SkipButton and Details
    ResourceCtx::provide(0);

    rsx! {
        div {
            class: format!(
                "{} {}",
                "m-0! mr-0 p-2 sticky z-2 top-0 bg-background",
                "flex justify-between items-center",
            ),

            span { class: "text-2xl", "{profile.read().name}" }

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
