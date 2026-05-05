use dioxus::prelude::*;

use crate::{
    components::{navbar::Navbar, router::Route},
    state::ME,
    views::{me::core::CoreData, showcase::Showcase},
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
