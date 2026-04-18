use dioxus::prelude::info;

use crate::{
    models::image::Image,
    state::{
        crud_query::crud_query,
        image::{converter::enqueue, ImageOpResult},
        server::ServerCtx,
    },
};

pub(in crate::state) async fn image_crud(
    ctx: &ServerCtx,
    img: Image,
) -> anyhow::Result<ImageOpResult> {
    use sqlx::types::Json;

    let Json(mut res) = sqlx::query_scalar::<_, Json<ImageOpResult>>(&crud_query(false))
        .bind(&ctx.session_id)
        .bind(img.id())
        .bind(img.pos())
        .bind(img.prompt())
        .bind(img.bytes())
        .fetch_one(&ctx.pool)
        .await?;

    let inserting = img.id().is_none() && img.pos().is_some();
    let updating = img.id().is_some() && img.pos().is_some();

    if let Some(from_db) = res.image.as_ref().filter(|_| res.authorized) {
        if let Some(id) = from_db.id().filter(|_| inserting) {
            info!("Sending job #{id}");
            enqueue(id).await?;
        }
        // not transferring bytes over network, when we have it here already
        else if updating {
            if let Some(bytes) = img.bytes() {
                if let Some(i) = res.image.as_mut() {
                    i.set_bytes(bytes.clone());
                }
            }
        }
    }

    Ok(res)
}
