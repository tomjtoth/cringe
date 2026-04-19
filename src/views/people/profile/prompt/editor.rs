use dioxus::prelude::*;

use crate::models::person::Prompt;
use crate::state::websocket::ops::ws_clear_op_id;
use crate::state::websocket::{WsCtx, WsRequest};
use crate::state::ME;
use crate::views::people::profile::{
    container::Container,
    utils::{container_class, ButtonOverride},
    ResourceCtx,
};

#[component]
pub(super) fn PromptEditor(src: Option<Prompt>) -> Element {
    let mut rcx = use_context::<ResourceCtx>();
    let wscx = use_context::<WsCtx>();

    let max = ME
        .read()
        .profile
        .as_ref()
        .map(|p| p.prompts().len() as i16)
        .unwrap_or(0)
        + {
            // only a new prompt can be added as additional
            if src.is_none() {
                1
            } else {
                0
            }
        };

    let mut sig = use_signal(|| {
        src.clone().unwrap_or(Prompt {
            position: Some(max - 1),
            ..Default::default()
        })
    });

    use_effect(move || ws_clear_op_id(&mut rcx));

    let (is_new, is_empty, has_changes) = sig.with(|p| {
        (
            p.id.is_none(),
            p.title == "" || p.body == "" || p.position == None,
            src.map(|src| src.title != p.title || src.body != p.body || src.position != p.position)
                .unwrap_or(true),
        )
    });

    let onsubmit = use_callback(move |_: Event<FormData>| {
        if is_new && is_empty || !is_new && !has_changes {
            return rcx.toggle_editing();
        }

        spawn(async move {
            _ = wscx.req(WsRequest::PromptOp(rcx.op_id, sig())).await;
        });
    });

    let disabled = !rcx.editing();

    let class = container_class(is_empty, has_changes);

    rsx! {
        Container { class: "{class} px-2 pt-10 pb-20", onsubmit,

            input {
                class: "min-w-20 w-full",
                placeholder: "Title",
                disabled,
                value: sig.read().title.as_ref(),
                oninput: move |evt| sig.write().title = evt.value(),
            }

            input {
                placeholder: "#",
                disabled,
                min: 1,
                max,
                r#type: "number",
                class: "max-w-20",
                value: sig.read().position.map(|pos| pos + 1),
                oninput: move |evt| {
                    sig.write().position = evt
                        .value()
                        .parse::<i16>()
                        .ok()
                        .filter(|&pos| 1 <= pos && pos <= max)
                        .map(|pos| pos - 1);
                },
            }

            textarea {
                class: "col-span-2 text-2xl",
                placeholder: "Body",
                disabled,
                value: sig.read().body.as_ref(),
                oninput: move |evt| sig.write().body = evt.value(),
            }

            ButtonOverride { has_changes, is_empty, is_new }
        }
    }
}
