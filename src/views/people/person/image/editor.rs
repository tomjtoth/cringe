use dioxus::prelude::*;

use crate::{
    models::image::Image,
    state::{
        websocket::{
            ops::{OpState, OPS},
            WsCtx, WsRequest,
        },
        ME,
    },
    views::people::person::{
        container::Container,
        image::{masterpiece::Masterpiece, ribbon::Ribbon},
        utils::{container_class, ButtonOverride},
        ResourceCtx,
    },
};

#[component]
pub fn ImageEditor(src: Option<Image>) -> Element {
    let mut rcx = use_context::<ResourceCtx>();
    let wscx = use_context::<WsCtx>();

    let max = ME
        .read()
        .profile
        .as_ref()
        .map(|p| p.images().len() as i16)
        .unwrap_or(0)
        + {
            // only a new image can be added as additional
            if src.is_none() {
                1
            } else {
                0
            }
        };

    let mut sig = use_signal(|| {
        src.clone().unwrap_or(Image::Uploaded {
            id: None,
            user_id: None,
            bytes: None,
            url: None,
            prompt: None,
            position: Some(max - 1),
        })
    });

    use_effect(move || {
        let id = rcx.oid();

        debug!("looking for #{id} in OPS");
        let is_success = OPS.with(|ops| {
            ops.get(&id)
                .map(|state| matches!(state, OpState::Success))
                .unwrap_or(false)
        });

        // must keep this separate, otherwise use_effect wouldn't subscribe to changes in OPS \o/
        if is_success {
            debug!("found #{id} in OPS");

            OPS.with_mut(|ops| {
                ops.remove(&id);
            });
            rcx.toggle_editing();
        }
    });

    let (is_new, is_empty, has_changes) = sig.with(|sig| {
        (
            sig.id().is_none(),
            !sig.has_bytes() && !sig.has_url() || sig.pos().is_none(),
            src.map(|src| {
                src.pos() != sig.pos() || src.prompt() != sig.prompt() || src.src() != sig.src()
            })
            .unwrap_or(true),
        )
    });

    let onsubmit = use_callback(move |_: Event<FormData>| {
        if is_new && is_empty || !is_new && !has_changes {
            return rcx.toggle_editing();
        }

        spawn(async move {
            _ = wscx.req(WsRequest::ImageOp(rcx.oid(), sig())).await;
        });
    });

    let class = container_class(is_empty, has_changes);

    let to_be_profile_pic = *sig.read().pos() == Some(0) || max == 1;

    rsx! {
        Container { class: "{class} rounded-b-none!", onsubmit,
            div { class: "pt-5 px-2 grid-cols-subgrid grid col-span-2",
                input {
                    name: "id",
                    hidden: true,
                    value: sig.read().id().clone(),
                }

                input {
                    name: "prompt",
                    placeholder: "Prompt if any",
                    class: "w-full min-w-30",
                    value: sig.read().prompt().as_deref(),
                    oninput: move |evt| sig.write().set_prompt(evt.value()),
                }

                input {
                    name: "position",
                    placeholder: "#",
                    class: "max-w-20",
                    r#type: "number",
                    min: 1,
                    max,
                    value: sig.read().pos().map(|p| p + 1),
                    oninput: move |evt| {
                        sig
                            .write()
                            .set_pos(
                                evt
                                    .value()
                                    .parse::<i16>()
                                    .ok()
                                    .filter(|&pos| 1 <= pos && pos <= max)
                                    .map(|pos| pos - 1),
                            )
                    },
                }
            }

            label {
                class: "relative col-span-2 overflow-hidden",
                class: if is_new { "cursor-pointer" },
                class: if is_new && !is_empty { "border-t" },

                if sig.with(|img| { img.has_url() || img.has_bytes() }) {
                    img { class: "object-cover w-full", src: sig.read().src() }
                } else {
                    Masterpiece {}
                }

                Ribbon { to_be_profile_pic }

                if is_new {
                    input {
                        name: "image",
                        hidden: true,
                        r#type: "file",
                        accept: ".png,.jpg,.jpeg,.bmp",
                        onchange: move |evt| {
                            spawn(async move {
                                if let Some(file) = evt.files().get(0) {
                                    if let Ok(bytes) = file.read_bytes().await {
                                        sig.write().set_bytes_resized(bytes.to_vec()).await;
                                    }
                                }
                            });
                        },
                    }
                }
            }

            ButtonOverride { has_changes, is_empty, is_new }
        }
    }
}
