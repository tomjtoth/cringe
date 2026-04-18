#[cfg(feature = "server")]
pub(super) mod converter;
#[cfg(feature = "server")]
pub(super) mod ops;

use std::collections::HashMap;

use dioxus::prelude::info;
use serde::{Deserialize, Serialize};

use crate::{
    models::{image::Image, person::Person},
    state::{
        crud_query::Sorted,
        websocket::ops::{OpState, OPS},
        ME,
    },
    views::people::listing::OTHERS,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageOpResult {
    pub authorized: bool,

    /// #### Will send this back anyways, so that clients can use .user_id and .id
    pub image: Option<Image>,
    pub sorted: Sorted,
}

pub(super) fn handle_image_crud_res(
    op_id: u128,
    ImageOpResult {
        authorized,
        image,
        sorted,
    }: ImageOpResult,
) {
    fn do_op(profile: &mut Person, image: &Image, sorted: &Sorted) {
        profile.images.retain(|img| img.id() != image.id());

        for img in profile.images.iter_mut() {
            if let Some(id) = img.id() {
                if let Some(pos) = sorted.get(id) {
                    img.set_pos(Some(*pos))
                }
            }
        }

        profile.images.sort_by_key(|img| *img.pos());

        if let Some(pos) = image.pos() {
            profile.images.insert(*pos as usize, image.clone());
        }
    }

    OPS.with_mut(|ops| {
        if let Some(OpState::Pending) = ops.get(&op_id) {
            if let Some(image) = image.as_ref().filter(|_| authorized) {
                if let Some(me) = ME.write().profile.as_mut() {
                    do_op(me, image, &sorted);
                }
                ops.insert(op_id, OpState::Success);
            } else {
                ops.insert(op_id, OpState::Failure);
            }
        } else if let Some(image) = image.as_ref().filter(|_| authorized) {
            OTHERS.with_mut(|profs| {
                if let Some(profile) = profs.iter_mut().find(|prof| prof.id == *image.user_id()) {
                    do_op(profile, image, &sorted);
                }
            });
        }
    });
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageConversionResult {
    pub image: Image,
    pub placeholders: HashMap<i32, String>,
}

pub(super) fn image_cli_converted(
    ImageConversionResult {
        image,
        placeholders,
    }: ImageConversionResult,
) {
    fn do_op(profile: &mut Person, image: &Image, placeholders: &HashMap<i32, String>) {
        if placeholders.len() > 0 {
            for img in profile.images.iter_mut() {
                if let Some(id) = img.id() {
                    if let Some(url) = placeholders.get(&id) {
                        info!("Updating placeholder for {id}");
                        img.set_url(Some(url.clone()));
                    }
                }
            }
        }

        if *image.user_id() == profile.id {
            if let Some(img) = profile.images.iter_mut().find(|i| i.id() == image.id()) {
                *img = image.clone();
                info!("Received #{}", img.id().unwrap());
            }
        }
    }

    if let Some(me) = ME.write().profile.as_mut() {
        do_op(me, &image, &placeholders);
    }
    for profile in OTHERS.write().iter_mut() {
        do_op(profile, &image, &placeholders);
    }
}
