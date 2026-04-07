use dioxus::prelude::*;

#[component]
pub(super) fn Ribbon(to_be_profile_pic: bool) -> Element {
    rsx! {
        if to_be_profile_pic {
            div {
                class: "absolute top-0 right-0 text-nowrap",
                class: "transform-[translate(50%,-50%)_rotateZ(45deg)_translateY(250%)]",
                class: "px-15 bg-purple-400 dark:bg-purple-600 border-2",
                "profile picture"
                "profile picture"
            }
        }
    }
}
