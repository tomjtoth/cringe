use dioxus::prelude::*;

use crate::{models::person::Liked, state::client::PEEPS, views::people::person::Person};

#[derive(Clone)]
struct PersonFilter(fn(&crate::models::person::Person) -> bool);

impl PartialEq for PersonFilter {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[component]
pub fn ListOfLikedProfiles() -> Element {
    let filter = PersonFilter(|x| matches!(x.liked, Some(Liked::Yes)));

    rsx! {
        Listing { filter }
    }
}

#[component]
pub fn ListOfDislikedProfiles() -> Element {
    let filter = PersonFilter(|x| matches!(x.liked, Some(Liked::No)));

    rsx! {
        Listing { filter }
    }
}

#[component]
pub fn ListOfUncheckedProfiles() -> Element {
    let filter = PersonFilter(|x| matches!(x.liked, None));

    rsx! {
        Listing { filter }
    }
}

#[component]
fn Listing(filter: PersonFilter) -> Element {
    rsx! {
        ul { class: "h-full overflow-scroll",
            for person in PEEPS.iter().filter(|person| (filter.0)(person)) {
                li { key: "{person.id}",
                    Person { id: &person.id }
                }
            }
        }
    }
}
