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

pub(super) fn class_canceler_deleter(
    new_but_empty: bool,
    to_be_deleted: bool,
) -> (String, Element, Element) {
    let class = format!(
        "grid grid-cols-[1fr_auto] gap-2 {}{}",
        "[&>input]:text-xl",
        if new_but_empty || to_be_deleted {
            // hiding the Container's default LikeButton
            " [&>button]:last:hidden"
        } else {
            ""
        }
    );

    let cb = "z-1 absolute bottom-5 right-5 bg-background select-none";

    let canceler = rsx! {
        if new_but_empty {
            button { class: "{cb} ml-2 border-2!", "Cancel ⚠️" }
        }
    };

    let deleter = rsx! {
        if to_be_deleted {
            Despair {}
            button { class: "{cb} ml-2 border-2!", "Delete 😱" }
        }
    };

    (class, canceler, deleter)
}
