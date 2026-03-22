use dioxus::prelude::*;

use crate::{router::Route, state::client::ME};

static CLASS: &str = "absolute top-1/2 left-1/2 -translate-1/2";

#[component]
pub fn NeedsLogin(children: Element) -> Element {
    rsx! {
        if ME().is_some() {
            {children}
        } else {
            p { class: CLASS,
                "You must log in first "
                a { href: "/auth/discord", "here" }
                ", your Discord profile "
                b { "must have a verified email" }
                "!"
            }
        }
    }
}

#[component]
pub fn NeedsProfile(children: Element) -> Element {
    rsx! {
        if let Some(Some(_)) = ME() {
            {children}
        } else {
            p { class: CLASS,
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
