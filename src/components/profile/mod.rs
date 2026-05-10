use std::collections::HashMap;
use std::ops::Deref;

use dioxus::prelude::*;

use crate::state::AUTH_CTE;
use crate::views::listing::OTHERS;
use crate::{
    components::profile::{button::SkipButton, details::Details, image::masterpiece::Masterpiece},
    models::Profile as MPerson,
    state::{
        websocket::ops::{OpState, OPS},
        ME,
    },
    views::listing::LCX,
};

mod button;
mod container;
pub(super) mod details;
mod image;
mod prompt;
mod utils;

use image::Image;
use prompt::Prompt;

#[derive(Clone)]
struct ProfileCtx {
    profile: ReadSignal<MPerson>,
}

impl Deref for ProfileCtx {
    type Target = ReadSignal<MPerson>;

    fn deref(&self) -> &Self::Target {
        &self.profile
    }
}

/// This might be a Prompt, an Image or the whole Details section
pub(crate) struct ResourceCtx {
    state: Signal<bool>,
    pub(crate) op_id: u8,
}

impl Copy for ResourceCtx {}
impl Clone for ResourceCtx {
    fn clone(&self) -> Self {
        *self
    }
}

impl ResourceCtx {
    fn provide(idx: usize) -> Self {
        let state = use_signal(|| false);
        use_context_provider(|| ResourceCtx {
            state,
            op_id: idx as u8,
        })
    }

    fn editing(&self) -> bool {
        (self.state)()
    }

    pub(crate) fn toggle_editing(&mut self) {
        // before switching to editing mode, initialize draft
        if self.op_id == 0 && !self.editing() {
            ME.with_mut(|me| {
                // after successful submit this is None
                if me.draft.is_none() {
                    me.draft = me.profile.clone().map(|me| Box::new(me));

                    // not sending images, nor prompts
                    me.draft.as_mut().map(|d| {
                        d.images.truncate(0);
                        d.prompts.truncate(0);
                    });
                }
            })
        }

        self.state.toggle();
        debug!("rcx.toggle_editing() -> {}", self.editing());
    }

    pub fn await_op(&mut self) {
        if !self.editing() {
            return;
        }

        let id = self.op_id;

        let returned = OPS.with(|ops| {
            debug!(
                "WS op {}({id}) polled {ops:?}",
                match id {
                    0 => "Details",
                    1..7 => "Prompt",
                    _ => "Image",
                }
            );
            ops.get(&id).cloned()
        });

        if let Some(state) = returned {
            OPS.with_mut(|ops| ops.remove(&id));

            match state {
                OpState::Success => {
                    self.toggle_editing();
                    if id == 0 {
                        ME.with_mut(|me| me.draft = None)
                    }
                }

                OpState::Failure => {
                    // TODO: show a modal or simple toast
                    error!("WS op #{id} failed");
                }
            };
        }
    }
}

type ImagesRetVal = (i32, Vec<u8>);

#[get("/api/images?ids")]
async fn get_images(ids: Option<Vec<i32>>) -> Result<Vec<ImagesRetVal>> {
    let mut res = vec![];

    if let (Some(ids), (Some(sess_id), pool)) = (ids, crate::state::server::get_ctx().await) {
        res = sqlx::query_as::<_, ImagesRetVal>(&format!(
            "
            WITH {AUTH_CTE}

            SELECT ui.id, bytes
            FROM user_images ui
            CROSS JOIN auth
            WHERE ui.id = ANY($2) AND bytes IS NOT NULL
            "
        ))
        .bind(&sess_id)
        .bind(&ids)
        .fetch_all(&pool)
        .await?;
    }

    Ok(res)
}

#[component]
pub fn Profile(profile: ReadSignal<MPerson>) -> Element {
    let mut div_ref = use_signal(|| None::<std::rc::Rc<MountedData>>);
    let mut collapsed = use_signal(|| LCX.read().flatten().is_some());
    let collapsible = LCX.with(|lcx| lcx.is_some() && lcx != &Some(None));

    use_context_provider(move || ProfileCtx { profile });

    use_effect(move || {
        if collapsible && !collapsed() {
            let ids: Vec<i32> = profile
                .read()
                .images
                .iter()
                .filter_map(|i| i.id().filter(|_| !i.has_bytes() && !i.has_url()))
                .collect();

            spawn(async move {
                if let Some(r) = div_ref() {
                    _ = r.scroll_to(ScrollBehavior::Smooth).await;
                }

                if let Ok(pairs) = get_images(Some(ids)).await {
                    let bytes: HashMap<i32, Vec<u8>> = pairs.into_iter().collect();

                    OTHERS.with_mut(|ppp| {
                        if let Some(p) = ppp.iter_mut().find(|p| p.id == profile.read().id) {
                            for i in p.images.iter_mut() {
                                if let Some(id) = i.id() {
                                    if let Some(bytes) = bytes.get(id) {
                                        i.set_bytes(bytes.clone());
                                    }
                                }
                            }
                        }
                    });
                }
            });
        }
    });

    // for the SkipButton and Details
    ResourceCtx::provide(0);

    rsx! {
        div {
            class: "relative break-inside-avoid overflow-clip",
            style: if !collapsed() { "column-span: all;" },
            onmounted: move |evt| div_ref.set(Some(evt.data())),

            div {
                class: "m-0! mr-0 p-2 bg-background",
                class: "flex justify-between items-center",
                class: if collapsible { "cursor-pointer" },
                class: if !collapsed() { "sticky z-2 top-0" },

                onclick: move |_| {
                    if collapsible {
                        collapsed.toggle()
                    }
                },

                div { class: "text-2xl",
                    "{profile.read().name}"

                    if collapsible {
                        if let Some(age) = profile.read().age().map(|n| n.to_string()).filter(|_| collapsed()) {
                            ", {age}"
                        }

                        div {
                            class: "ml-2 inline-block font-bold transition duration-200",
                            class: if collapsed() { "-rotate-90" } else { "rotate-90" },
                            "<"
                        }
                    }
                }

                if LCX.read().is_none() {
                    a {
                        class: "border rounded p-2 cursor-pointer select-none",
                        href: "/logout",
                        "logout [➜"
                    }
                }
            }

            if LCX.read().is_some() && collapsed() {
                // images[0]
                div { class: "relative overflow-hidden border rounded-2xl",
                    if let Some(img) = profile.read().images.get(0) {
                        img { class: "object-cover w-full", src: img.src() }
                    } else {
                        Masterpiece {}
                    }
                }
            } else {
                div {
                    style: "column-span: all;",
                    class: "relative sm:columns-2 lg:columns-3 *:mb-2 text-lg",

                    Image { idx: 0 }
                    Prompt { idx: 0 }

                    Details {}

                    Image { idx: 1 }
                    Prompt { idx: 1 }

                    Image { idx: 2 }
                    Prompt { idx: 2 }

                    Image { idx: 3 }
                    Prompt { idx: 3 }

                    Image { idx: 4 }
                    Prompt { idx: 4 }

                    Image { idx: 5 }
                    Prompt { idx: 5 }
                }

                if LCX.read().is_some() {
                    div { class: "sticky h-0 bottom-0 overflow-visible", SkipButton {} }
                }
            }
        }
    }
}
