use dioxus::prelude::*;

use crate::models::person::Person;

#[component]
pub(super) fn Has(sig: Signal<Person>, editing: bool, already_has_kids: bool) -> Element {
    let value = sig.read().kids.as_ref().map(|k| k.has);

    rsx! {
        if editing {
            li {
                "🧑‍🧒‍🧒"
                input {
                    placeholder: "# of kids you have",
                    class: "w-20",
                    r#type: "tel",
                    min: 0,
                    max: i8::MAX,
                    value,
                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let has = evt.value().parse::<i8>().ok();
                            if let Some(kids) = p.kids.as_mut() {
                                kids.has = has;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.kids = Some(crate::models::person::Kids {
                                        has,
                                        wants: None,
                                    });
                                }
                            }
                        });
                    },
                }
            }
        } else {
            if let Some(has) = sig.read().kids.as_ref().and_then(|k| k.has) {
                li {
                    "🧑‍🧒‍🧒 Has "
                    if has > 0 {
                        if has == i8::MAX {
                            b { "{has} or more" }
                        } else {
                            "{has}"
                        }
                    } else {
                        " no"
                    }
                    " kids"
                }
            }
        }
    }
}
