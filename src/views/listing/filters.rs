use dioxus::prelude::*;

use crate::{models::Decision, views::listing::LCX};

#[component]
pub fn Filters() -> Element {
    let wants = LCX.read().flatten();

    rsx! {
        form { class: "flex flex-col items-center gap-2",

            h3 { "Filter profiles" }

            for (val , (ico , msg) , shadow , checked) in [
                (Some(Decision::Skip), "🚫 skipped", "text-shadow-red-500"),
                (None, "😬 cringe", "text-shadow-yellow-500"),
                (Some(Decision::Like), "✅ liked", "text-shadow-green-500"),
            ]
                .map(|(a, b, c)| (a, b.split_once(" ").unwrap(), c, a == wants))
            {
                label {
                    class: "cursor-pointer",
                    class: if !checked { "text-gray-500" },
                    input {
                        tabindex: -1,
                        r#type: "radio",
                        name: "wants",
                        class: "border-none! appearance-none checked:text-sha",
                        checked,
                        onclick: move |_| { LCX.with_mut(|cx| *cx = Some(val)) },
                    }
                    span { class: if checked { "text-shadow-[0_0_1px,0_0_2px,0_0_3px,0_0_4px,0_0_5px] {shadow}" },
                        "{ico}"
                    }
                    " {msg}"
                }
            }

        }
    }
}
