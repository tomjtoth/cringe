use dioxus::prelude::*;

#[cfg(not(feature = "server"))]
use crate::models::person::Kids;

use crate::models::person::Person;

#[component]
pub(super) fn Kids(sig: Signal<Person>, editing: bool) -> Element {
    let already_has_kids = sig.read().kids.as_ref().is_some_and(|k| k.has > Some(0));

    rsx! {
        if editing {
            li {
                "🧑‍🧒‍🧒"
                input {
                    placeholder: "# of kids you have",
                    r#type: "tel",
                    min: 0,
                    max: i8::MAX,
                    value: sig.read().kids.as_ref().map(|k| k.has),
                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let has = evt.value().parse::<i8>().ok();
                            if let Some(kids) = p.kids.as_mut() {
                                kids.has = has;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.kids = Some(Kids { has, wants: None });
                                }
                            }
                        });
                    },
                }
            }

            li {
                "🍼"
                select {
                    class: if sig.read().kids.as_ref().map(|k| k.wants).unwrap_or(None) == None { "text-gray-500" },
                    value: sig.read().kids.as_ref().map(|k| k.wants),
                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let wants = evt.value().parse::<i8>().ok();
                            if let Some(kids) = p.kids.as_mut() {
                                kids.wants = wants;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.kids = Some(Kids {
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
            if let Some(kids) = sig.read().kids.as_ref() {
                if let Some(has) = kids.has {
                    li {
                        "🧑‍🧒‍🧒 "
                        if has > 0 {
                            "Has {has}"
                        } else {
                            "No"
                        }
                        " kids"
                    }
                }

                if let Some(wants) = kids.wants {
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
                        } else {
                            if already_has_kids {
                                " more"
                            }
                        }
                        " kids"
                    }
                }
            }
        }
    }
}
