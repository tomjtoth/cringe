use dioxus::prelude::*;

#[component]
fn Despair() -> Element {
    let cls = "absolute select-none text-shadow-[0_0_5px,0_0_15px] shadow-purple-500 text-red-500";

    rsx! {
        span { class: "{cls} -rotate-49 top-1/6 left-1/5", "DESPAIR" }
        span { class: "{cls} rotate-27 top-1/7 right-1/5", "DESPAIR" }
        span { class: "{cls} -rotate-13 top-4/7 left-3/5", "DESPAIR" }
    }
}

#[component]
pub(super) fn ButtonOverride(is_new: bool, is_empty: bool, has_changes: bool) -> Element {
    let class = "z-1 absolute bottom-5 right-5 bg-background select-none ml-2 border-2!";

    rsx! {
        if is_new && is_empty || !is_new && !has_changes {
            button { class, "Cancel 🤷" }
        } else if !is_new && is_empty {
            Despair {}
            button { class, "Delete 😱" }
        }
    }
}

pub(super) fn container_class(is_empty: bool, has_changes: bool) -> String {
    format!(
        "grid grid-cols-[1fr_auto] gap-2 {}{}",
        "[&>input]:text-xl",
        if is_empty || !has_changes {
            // hiding the Container's default LikeButton
            " [&>button]:last:hidden"
        } else {
            ""
        }
    )
}
