use dioxus::prelude::info;

use crate::{
    models::image::Image,
    state::{
        crud_query::crud_query,
        image::{converter::enqueue, ImageOpResult},
        server::ServerCtx,
    },
};

pub async fn image_crud(ctx: &ServerCtx, img: Image) -> anyhow::Result<ImageOpResult> {
    use sqlx::types::Json;

    let Json(mut res) = sqlx::query_scalar::<_, Json<ImageOpResult>>(&crud_query(false))
        .bind(&ctx.session_id)
        .bind(img.id())
        .bind(img.pos())
        .bind(img.prompt())
        .bind(img.bytes())
        .fetch_one(&ctx.pool)
        .await?;

    if res.authorized {
        let updating = img.id().is_some() && img.pos().is_some();
        let inserting = img.id().is_none() && img.pos().is_some();

        if let Some(id) = res.image.id().filter(|_| inserting) {
            info!("Sending job #{id}");
            enqueue(id).await?;
        }

        if updating {
            if let Some(bytes) = img.bytes() {
                res.image.set_bytes(bytes.clone());
            }
        }
    }

    Ok(res)
}
