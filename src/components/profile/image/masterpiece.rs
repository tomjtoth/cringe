use dioxus::prelude::*;

#[component]
pub(super) fn Masterpiece() -> Element {
    let a = "absolute";

    rsx! {

        div {
            class: "border-15 border-amber-300 dark:border-amber-700 border-ridge",
            class: "px-5 pt-30 pb-5 relative select-none text-center",

            span { class: "{a} left-2/30 top-3/30 text-5xl", "🌞" }

            span { class: "{a} right-3/30 top-1/30 text-7xl", "☁️" }
            span { class: "{a} left-4/30 top-3/30 text-4xl", "☁️" }
            span { class: "{a} left-14/30 top-2/30 text-3xl", "☁️" }

            span { class: "{a} left-10/30 top-5/30", "🕊️" }
            span { class: "{a} left-23/30 top-3/30 -scale-x-100", "🕊️" }

            span { class: "{a} left-4/30 top-16/30", "🌳" }
            span { class: "{a} left-2/30 top-16/30 text-2xl", "🌳" }
            span { class: "{a} left-1/30 top-17/30 text-3xl", "🌳" }
            span { class: "{a} left-3/30 bottom-8/30 text-xs", "🦔" }
            span { class: "{a} left-2/30 top-23/30 text-4xl", "🏌️" }

            span { class: "{a} right-2/30 bottom-8/30 text-7xl", "🌳" }
            span { class: "{a} right-5/30 top-20/30 text-2xl", "🐕" }

            span { class: "{a} top-14/30 left-13/30 text-6xl -z-1 rotate-25 animate-peeking-fox",
                "🦊"
            }
            span { class: "text-9xl ", "🙎" }
        }
    }
}
