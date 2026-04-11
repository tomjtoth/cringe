#[cfg(feature = "server")]
pub(super) mod converter;
#[cfg(feature = "server")]
pub(super) mod ops;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{models::image::Image, state::ME};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageOpResult {
    pub authorized: bool,
    pub image: Image,
    pub sorted: HashMap<i32, i16>,
    // broadcast to all connections for now
    // session_ids: Vec<String>,
}

pub(super) fn image_cli_ops(ImageOpResult { image, sorted, .. }: ImageOpResult) {
    if let Some(me) = ME.write().profile.as_mut() {
        if *image.user_id() == me.id {
            me.images.retain(|img| img.id() != image.id());
            if let Some(pos) = image.pos() {
                me.images.insert(*pos as usize, image);
            }

            for img in me.images.iter_mut() {
                if let Some(id) = img.id() {
                    if let Some(pos) = sorted.get(id) {
                        img.set_pos(Some(*pos))
                    }
                }
            }

            me.images.sort_by_key(|img| *img.pos());
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageConversionResult {
    pub image: Image,
    pub placeholders: HashMap<i32, String>,
}

pub async fn image_cli_converted(res: ImageConversionResult) {}
