use dioxus::prelude::*;

use crate::models::person::Person;

mod drinking;
mod drugs;
mod marijuana;
mod smoking;

#[component]
pub(super) fn Habits(sig: Signal<Person>, editing: bool) -> Element {
    rsx! {
        drinking::Drinking { sig, editing }
        smoking::Smoking { sig, editing }
        marijuana::Marijuana { sig, editing }
        drugs::Drugs { sig, editing }
    }
}
