use dioxus::prelude::*;

use crate::{
    components::modal::{TrModal, MODALS},
    models::Decision,
    views::listing::ListingCtx,
};

#[component]
pub fn Filters() -> Element {
    let mut lcx = use_context::<ListingCtx>();
    let wants = lcx.read().flatten();

    rsx! {
        div { class: "flex flex-col gap-2",

            label { class: "flex items-center p-2 text-nowrap",
                "Show me "

                select {
                    class: "border-none! appearance-none px-0",
                    value: match wants {
                        Some(Decision::Like) => "like",
                        Some(Decision::Skip) => "skip",
                        _ => "",
                    },
                    onchange: move |evt| {
                        let new = match evt.value().as_str() {
                            "like" => Some(Decision::Like),
                            "skip" => Some(Decision::Skip),
                            _ => None,
                        };

                        lcx.set(Some(new));

                        MODALS.pop();
                    },

                    option { value: "", selected: wants == None, "something cringe 😬" }
                    option {
                        value: "like",
                        selected: wants == Some(Decision::Like),
                        "profiles I liked ✅"
                    }
                    option {
                        value: "skip",
                        selected: wants == Some(Decision::Skip),
                        "profiles I skipped 🚫"
                    }
                }
            }

            div { class: "border rounded [&_input]:w-15",
                h3 { "age" }

                div { class: "flex gap-2 p-2",
                    label {
                        "min:"
                        input { r#type: "number" }
                    }

                    label {
                        "max:"
                        input { r#type: "number" }
                    }

                }
            }

            label { "min height" }
            label { "max height" }
        }

    }
}
