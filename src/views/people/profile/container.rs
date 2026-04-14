use dioxus::prelude::*;

use crate::views::people::profile::button::LikeButton;

#[component]
pub fn Container(
    children: Element,
    class: Option<String>,
    wo_button: Option<bool>,
    onsubmit: Option<Callback<Event<FormData>>>,
) -> Element {
    let class = format!(
        "relative border rounded-2xl w-full overflow-hidden {}",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        form {
            class,
            onsubmit: move |evt| {
                evt.prevent_default();

                if let Some(handler) = onsubmit {
                    handler(evt);
                }
            },

            {children}

            if wo_button != Some(true) {
                LikeButton {}
            }
        }
    }
}
