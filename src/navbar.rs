use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "grow overflow-scroll", Outlet::<Route> {} }

        ul { class: "bg-background border-t flex justify-around w-full
                     [&_a]:flex [&_a]:flex-col text-center",
            li {
                Link { to: Route::About {},
                    "⁉️"
                    span { "about" }
                }
            }
        }
    }
}
