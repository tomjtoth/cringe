use dioxus::prelude::*;

#[component]
pub(super) fn Ribbon(to_be_profile_pic: bool) -> Element {
    rsx! {
        if to_be_profile_pic {
            div {
                class: "absolute top-0 right-0 origin-bottom-right",
                class: "rotate-45 translate-y-27 text-nowrap",
                class: "px-10 bg-purple-400 dark:bg-purple-600 border-2",
                "profile picture"
            }
        }
    }
}
