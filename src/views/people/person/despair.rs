use dioxus::prelude::*;

#[component]
pub(super) fn Despair() -> Element {
    let cls = "absolute select-none text-shadow-[0_0_5px,0_0_15px] shadow-purple-500 text-red-500";

    rsx! {
        span { class: "{cls} -rotate-49 top-1/6 left-1/5", "DESPAIR" }
        span { class: "{cls} rotate-27 top-1/7 right-1/5", "DESPAIR" }
        span { class: "{cls} -rotate-13 top-4/7 left-3/5", "DESPAIR" }
    }
}
