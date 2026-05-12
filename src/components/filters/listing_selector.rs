use dioxus::prelude::*;

use crate::models::Decision;

#[component]
pub fn ListingSelector(mut wants: Signal<Option<Decision>>) -> Element {
    rsx! {
        div { class: "flex-col items-end! [&_input]:hidden",

            for (val , (ico , msg) , shadow , checked) in [
                (Some(Decision::Skip), "🚫 skipped", "text-shadow-red-500"),
                (None, "😬 cringe", "text-shadow-yellow-500"),
                (Some(Decision::Like), "✅ liked", "text-shadow-green-500"),
            ]
                .map(|(a, b, c)| (a, b.split_once(" ").unwrap(), c, a == wants()))
            {
                label {
                    input {
                        r#type: "radio",
                        required: true,
                        name: "wants",
                        checked,
                        onclick: move |_| wants.set(val),
                    }

                    span { class: if checked { "text-shadow-[0_0_2px]" } else { "text-gray-500" },
                        "{msg} "
                    }

                    span { class: if checked { "text-shadow-[0_0_1px,0_0_2px,0_0_3px,0_0_4px,0_0_5px] {shadow}" },
                        "{ico}"
                    }
                }
            }
        }
    }
}
