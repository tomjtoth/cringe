#[cfg(feature = "server")]
pub(super) mod converter;
#[cfg(feature = "server")]
pub(super) mod ops;

use std::collections::HashMap;

use dioxus::prelude::{debug, info};
use serde::{Deserialize, Serialize};

use crate::{
    models::{image::Image, person::Person},
    state::{
        websocket::ops::{OpState, OPS},
        ME,
    },
    views::people::listing::OTHERS,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageOpResult {
    pub authorized: bool,
    pub image: Image,
    pub sorted: HashMap<i32, i16>,
    // broadcast to all connections for now
    // session_ids: Vec<String>,
}

pub(super) fn image_cli_ops(
    oid: u32,
    ImageOpResult {
        authorized,
        image,
        sorted,
    }: ImageOpResult,
) {
    fn do_op(profile: &mut Person, image: &Image, sorted: &HashMap<i32, i16>) {
        if *image.user_id() == profile.id {
            profile.images.retain(|img| img.id() != image.id());
            if let Some(pos) = image.pos() {
                profile.images.insert(*pos as usize, image.clone());
            }

            for img in profile.images.iter_mut() {
                if let Some(id) = img.id() {
                    if let Some(pos) = sorted.get(id) {
                        img.set_pos(Some(*pos))
                    }
                }
            }

            profile.images.sort_by_key(|img| *img.pos());
        }
    }

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

            if let Some(me) = ME.write().profile.as_mut() {
                do_op(me, &image, &sorted);
            }
        } else {
            for profile in OTHERS.write().iter_mut() {
                do_op(profile, &image, &sorted);
            }
        }
    });
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageConversionResult {
    pub image: Image,
    pub placeholders: HashMap<i32, String>,
}

pub fn image_cli_converted(
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

    if let Some(p) = ME.write().profile.as_mut() {
        do_op(p, &image, &placeholders);
    }
    for profile in OTHERS.write().iter_mut() {
        do_op(profile, &image, &placeholders);
    }
}
