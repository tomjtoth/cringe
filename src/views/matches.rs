use dioxus::prelude::*;

use crate::views::listing::LCX;

#[component]
pub fn Matches() -> Element {
    use_effect(move || LCX.with_mut(|cx| *cx = None));

    rsx! {
        div { class: "app-center",
            h1 { "TODO" }
        }
    }
}
