#[cfg(feature = "server")]
pub(super) mod crud;

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
    pub prompt: Option<Prompt>,
    pub sorted: Sorted,
}

pub(super) fn handle_prompt_crud_res(
    op_id: u128,
    PromptOpResult {
        authorized,
        prompt,
        sorted,
    }: PromptOpResult,
) {
    fn do_op(profile: &mut Person, prompt: &Prompt, sorted: &Sorted) {
        profile.prompts.retain(|p| p.id != prompt.id);

        for p in profile.prompts.iter_mut() {
            if let Some(id) = p.id {
                if let Some(pos) = sorted.get(&id) {
                    p.position = Some(*pos);
                }
            }
        }

        profile.prompts.sort_by_key(|p| p.position);

        if let Some(pos) = prompt.position {
            profile.prompts.insert(pos as usize, prompt.clone());
        }
    }

    OPS.with_mut(|ops| {
        if let Some(OpState::Pending) = ops.get(&op_id) {
            if let Some(prompt) = prompt.as_ref().filter(|_| authorized) {
                if let Some(me) = ME.write().profile.as_mut() {
                    do_op(me, prompt, &sorted);
                }
                ops.insert(op_id, OpState::Success);
            } else {
                ops.insert(op_id, OpState::Failure);
            }
        } else if let Some(prompt) = prompt.as_ref().filter(|_| authorized) {
            OTHERS.with_mut(|profs| {
                if let Some(profile) = profs.iter_mut().find(|prof| prof.id == prompt.user_id) {
                    do_op(profile, prompt, &sorted);
                }
            });
        }
    });
}
