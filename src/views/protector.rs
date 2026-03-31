use dioxus::prelude::*;

use crate::{router::Route, state::client::ME, views::login::Login};

#[component]
pub fn NeedsLogin(children: Element) -> Element {
    rsx! {
        if ME.read().authenticated {
            {children}
        } else {
            Login {}
        }
    }
}

#[component]
pub fn NeedsProfile(children: Element) -> Element {
    rsx! {
        if ME.read().profile.is_some() {
            {children}
        } else {
            p { class: "app-center",
                "Create a profile first "
                Link { class: "pre-preflight", to: Route::Me {}, "here" }
                "!"
            }
        }
    }
}

#[component]
pub fn NeedsLoginAndProfile(children: Element) -> Element {
    rsx! {
        NeedsLogin {
            NeedsProfile { children }
        }
    }
}
