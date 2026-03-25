use dioxus::prelude::*;

use crate::views::people::person::button::LikeButton;

#[component]
pub fn Container(children: Element, class: Option<String>, wo_button: Option<bool>) -> Element {
    let class = format!(
        "relative border rounded-2xl w-full overflow-hidden {}",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { class,
            {children}

            if wo_button != Some(true) {
                LikeButton {}
            }
        }
    }
}
