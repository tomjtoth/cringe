use dioxus::prelude::*;

use crate::views::people::person::buttons::LikeButton;

#[component]
pub fn Container(children: Element, id: Option<i32>, class: Option<String>) -> Element {
    let class = format!(
        "relative border rounded-2xl w-full overflow-hidden {}",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { class,
            {children}

            if let Some(id) = id {
                LikeButton { id }
            }
        }
    }
}
