#[cfg(feature = "server")]
pub(super) mod server;

use serde::{Deserialize, Serialize};

use crate::{
    models::Profile,
    state::{
        websocket::ops::{OpState, OPS},
        ME,
    },
    views::people::listing::OTHERS,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DetailsUpateRes {
    pub authorized: bool,
    pub profile: Profile,
}

pub(super) fn handle_details_update_res(
    op_id: u8,
    DetailsUpateRes {
        authorized,
        profile,
    }: DetailsUpateRes,
) {
    fn do_op(target: &mut Profile, source: Profile) {
        target.education = source.education;
        target.occupation = source.occupation;
        target.location = source.location;
        target.hometown = source.hometown;
        target.seeking = source.seeking;
        target.relationship_type = source.relationship_type;

        target.has_children = source.has_children;
        target.family_plans = source.family_plans;

        target.drinking = source.drinking;
        target.smoking = source.smoking;
        target.marijuana = source.marijuana;
        target.drugs = source.drugs;
    }

    ME.with_mut(|me| {
        if let Some(me) = me.profile.as_mut() {
            if profile.id == me.id {
                OPS.with_mut(|ops| {
                    if authorized {
                        do_op(me, profile);
                        ops.insert(op_id, OpState::Success);
                    } else {
                        ops.insert(op_id, OpState::Failure);
                    }
                })
            } else if authorized {
                OTHERS.with_mut(|profs| {
                    if let Some(prof) = profs.iter_mut().find(|p| p.id == profile.id) {
                        do_op(prof, profile);
                    }
                });
            }
        }
    });
}
