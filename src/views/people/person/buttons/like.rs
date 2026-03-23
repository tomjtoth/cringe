use dioxus::prelude::*;

use crate::{
    models::person::Decision,
    views::people::{listing::ListingCtx, person::PersonCtx},
};


#[component]
pub fn LikeButton() -> Element {
    let pc = use_context::<PersonCtx>();
    let lc = use_context::<Option<ListingCtx>>();

    rsx! {
        // we are in a listing, but not in the "liked" profiles
        if lc.is_some_and(|lc| !matches!(lc.0, Some(Decision::Like))) {
            if let Some(id) = pc.person.id {
                button { class: "absolute z-2 bottom-5 right-5 p-3!
                        bg-background rounded-full!
                        cursor-pointer select-none",

                    "❤️"
                }
            }
        }
    }
}
