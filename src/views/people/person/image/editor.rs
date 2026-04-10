use dioxus::prelude::*;

use crate::{
    models::image::Image,
    state::{
        websocket::{WsCtx, WsRequest},
        ME,
    },
    views::people::person::{
        container::Container,
        image::{masterpiece::Masterpiece, ribbon::Ribbon},
        utils::class_canceler_deleter,
        ResourceCtx,
    },
};

#[component]
pub fn ImageEditor(src: Option<Image>) -> Element {
    let rcx = use_context::<ResourceCtx>();
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
        src.unwrap_or(Image::Uploaded {
            id: None,
            bytes: None,
            url: None,
            prompt: None,
            position: Some(max - 1),
        })
    });

    let (existing, new_but_empty, to_be_deleted) = sig.with(|img| {
        let has_id = img.id().is_some();
        (
            has_id,
            !has_id && !img.has_bytes(),
            has_id && img.pos().is_none(),
        )
    });

    let onsubmit = use_callback({
        move |_: Event<FormData>| {
            spawn({
                let mut rcx = rcx.clone();

                async move {
                    rcx.next_state();

                    if new_but_empty {
                        return rcx.next_state();
                    }

                    _ = wscx.send(WsRequest::ImageToDb(sig())).await;

                    rcx.next_state();
                }
            });
        }
    });

    let (class, canceler, deleter) = class_canceler_deleter(new_but_empty, to_be_deleted);

    let to_be_profile_pic = *sig.read().pos() == Some(0) || max == 1;

    rsx! {
        Container { class: "{class} rounded-b-none!", onsubmit,
            div { class: "pt-5 px-2 grid-cols-subgrid grid col-span-2",
                input { name: "id", hidden: true, value: sig.read().id() }

                input {
                    name: "prompt",
                    placeholder: "Prompt if any",
                    class: "w-full min-w-30",
                    value: sig.read().prompt(),
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
                class: if !existing { "cursor-pointer" },
                class: if !existing && !new_but_empty { "border-t" },

                if sig.with(|img| { img.has_url() || img.has_bytes() }) {
                    img { class: "object-cover w-full", src: sig.read().src() }
                } else {
                    Masterpiece {}
                }

                Ribbon { to_be_profile_pic }

                if !existing {
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

            // buttons
            {canceler}
            {deleter}
        }
    }
}
