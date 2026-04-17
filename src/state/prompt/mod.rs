#[cfg(feature = "server")]
pub(super) mod crud;

use dioxus::prelude::debug;
use serde::{Deserialize, Serialize};

use crate::{
    models::person::{Person, Prompt},
    state::{
        crud_query::Sorted,
        websocket::ops::{OpState, OPS},
        ME,
    },
    views::people::listing::OTHERS,
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct PromptOpResult {
    pub authorized: bool,
    pub sorted: Sorted,
    pub prompt: Prompt,
}

pub(super) fn handle_prompt_crud_res(
    oid: u32,
    PromptOpResult {
        authorized,
        prompt,
        sorted,
    }: PromptOpResult,
) {
    OPS.with_mut(|ops| {
        // this user initiated the op
        if let Some(OpState::Pending) = ops.get(&oid) {
            ops.insert(
                oid,
                if authorized {
                    OpState::Success
                } else {
                    OpState::Failure
                },
            );
            debug!("OPS: {ops:?}");

            if authorized {
                if let Some(me) = ME.write().profile.as_mut() {
                    do_op(me, &prompt, &sorted);
                }
            }
        } else {
            if authorized {
                for profile in OTHERS.write().iter_mut() {
                    do_op(profile, &prompt, &sorted);
                }
            }
        }
    });
}

fn do_op(profile: &mut Person, prompt: &Prompt, sorted: &Sorted) {
    if prompt.user_id == profile.id {
        profile.prompts.retain(|p| p.id != prompt.id);
        if let Some(pos) = prompt.position {
            profile.prompts.insert(pos as usize, prompt.clone());
        }

        for p in profile.prompts.iter_mut() {
            if let Some(id) = p.id {
                if let Some(pos) = sorted.get(&id) {
                    p.position = Some(*pos)
                }
            }
        }

        profile.prompts.sort_by_key(|p| p.position);
    }
}
