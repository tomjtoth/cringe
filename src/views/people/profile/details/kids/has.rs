use dioxus::prelude::*;

use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn Has(already_has_kids: bool) -> Element {
    let mut dcx = use_context::<DetailsCtx>();

    let value = dcx.rw.read().kids.as_ref().map(|k| k.has).flatten();

    rsx! {
        if (dcx.editing)() {
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
                        dcx.rw
                            .with_mut(|p| {
                                let has = evt.value().parse::<i8>().ok();
                                if let Some(kids) = p.kids.as_mut() {
                                    kids.has = has;
                                } else {
                                    #[cfg(not(feature = "server"))]
                                    {
                                        p.kids = Some(crate::models::kids::Kids {
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
            if let Some(has) = dcx.ro.read().kids.as_ref().and_then(|k| k.has) {
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
