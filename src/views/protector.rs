use dioxus::prelude::*;

use crate::{router::Route, state::client::ME, views::login::Login};

#[component]
pub fn NeedsLogin(children: Element) -> Element {
    rsx! {
        if ME().is_some() {
            {children}
        } else {
            Login {}
        }
    }
}

#[component]
pub fn NeedsProfile(children: Element) -> Element {
    rsx! {
        if let Some(Some(_)) = ME() {
            {children}
        } else {
            p { class: "app-center",
                "Create a profile first "
                Link { to: Route::Me {}, "here" }
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
