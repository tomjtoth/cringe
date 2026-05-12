use dioxus::prelude::*;
use strum::{EnumProperty, IntoEnumIterator};

use crate::{
    components::modal::{TrModal, MODALS},
    models::Gender as EGender,
};

#[component]
pub(super) fn Gender() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {
        div { class: "flex-col items-start!",
            for (val , glow , checked) in EGender::iter()
                .map(|e| (e, e.get_str("glow").unwrap(), fcx.read().gender.contains(&e)))
            {
                if let Some((ico, txt)) = val.to_string().split_once(" ") {
                    label {
                        input {
                            r#type: "checkbox",
                            name: "gender",
                            value: "{val}",
                            checked,
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
                                        MODALS
                                            .build("z-10")
                                            .button("Ok", None)
                                            .title("Filtering genders")
                                            .p(
                                                "Since gender is always defined for every profile, filtering to all equals to having no filter at all.",
                                            );
                                    }
                                })
                            },
                        }

                        span { class: if checked { "text-shadow-[0_0_1px,0_0_2px,0_0_3px,0_0_4px,0_0_5px] {glow}" },
                            "{ico}"
                        }

                        span { class: if checked { "text-shadow-[0_0_2px]" } else { "text-gray-500" },
                            " {txt}"
                        }
                    }
                }
            }
        }
    }
}
