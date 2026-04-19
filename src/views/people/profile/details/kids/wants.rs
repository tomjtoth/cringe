use dioxus::prelude::*;

use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn Wants(already_has_kids: bool) -> Element {
    let mut dcx = use_context::<DetailsCtx>();

    let value = dcx.rw.read().kids.as_ref().map(|k| k.wants).flatten();

    rsx! {
        if (dcx.editing)() {
            li {
                "🍼"
                select {
                    class: if value.is_none() { "text-gray-500" },
                    value,
                    onchange: move |evt| {
                        dcx.rw
                            .with_mut(|p| {
                                let wants = evt.value().parse::<i8>().ok();
                                if let Some(kids) = p.kids.as_mut() {
                                    kids.wants = wants;
                                } else {
                                    #[cfg(not(feature = "server"))]
                                    {
                                        p.kids = Some(crate::models::kids::Kids {
                                            wants,
                                            ..Default::default()
                                        });
                                    }
                                }
                            });
                    },

                    option { value: "",
                        "# of"

                        if already_has_kids {
                            " additional"
                        }

                        " kids you want"
                    }
                    option { value: -1,
                        "I don't know if I want any"

                        if already_has_kids {
                            " more"
                        }

                        " kids"
                    }
                    option { value: 0,
                        "I don't want any"

                        if already_has_kids {
                            " more"
                        }

                        " kids"
                    }

                    for n in 1..i8::MAX {
                        option { value: n,
                            "I want {n}"

                            if already_has_kids {
                                " more"
                            }

                            " kids"
                        }
                    }

                    option { value: i8::MAX, "I want {i8::MAX} **or more** kids" }
                }
            }
        } else {
            if let Some(wants) = dcx.ro.read().kids.as_ref().and_then(|k| k.wants) {
                li {
                    "🍼 "
                    if wants > 0 {
                        "Wants {wants}"
                    } else if wants == 0 {
                        "Doesn't want"
                    } else {
                        "Doesn't know if wants any"
                    }

                    if wants == i8::MAX {
                        b { " or more" }
                    }

                    if already_has_kids {
                        " additional"
                    }

                    " kids"
                }
            }
        }
    }
}
