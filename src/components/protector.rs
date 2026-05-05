use dioxus::prelude::*;

use crate::{
    components::{core_data::CoreData, navbar::Navbar, router::Route},
    state::ME,
    views::showcase::Showcase,
};

#[component]
pub fn Protector() -> Element {
    rsx! {
        if ME.read().authenticated {
            if ME.read().profile.is_some() {
                div { class: "grow overflow-hidden", Outlet::<Route> {} }
                Navbar {}
            } else {
                CoreData {}
            }
        } else {
            Showcase {}
        }

    }
}
