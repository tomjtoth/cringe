use dioxus::prelude::*;

#[component]
pub fn Container(children: Element) -> Element {
    rsx! {
        div { class: "border rounded-2xl max-md:w-full overflow-hidden", {children} }
    }
}
