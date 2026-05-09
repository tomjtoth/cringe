use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::Gender as EGender;

#[component]
pub(super) fn Gender() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {

        div { class: "flex-col",
            "Gender:"

            div { class: "flex gap-2 text-nowrap",

                for val in EGender::iter() {
                    label {
                        input {
                            r#type: "checkbox",
                            name: "gender",
                            value: "{val}",
                            checked: fcx.read().gender.contains(&val),
                            onclick: move |evt| {
                                fcx
                                    .with_mut(|ff| {
                                    if ff.gender.contains(&val) {
                                        ff.gender.retain(|f| f != &val);
                                    } else if ff.gender.len() != EGender::iter().len() - 1 {
                                        ff.gender.push(val.clone());
                                    } else {
                                        evt.prevent_default();
                                        ff.gender.clear();
                                    }
                                })
                            },
                        }
                        " {val}"
                    }
                }
            }
        }
    }
}
