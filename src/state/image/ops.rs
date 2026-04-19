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

    if res.authorized {
        let inserted = img.id().is_none() && img.pos().is_some();
        let updated = img.id().is_some() && img.pos().is_some();

        if let Some(id) = res.image.id().filter(|_| inserted) {
            info!("Sending job #{id}");
            enqueue(id).await?;
        } else if updated {
            // not transferring bytes over network, when we have it here already
            if let Some(bytes) = img.bytes() {
                res.image.set_bytes(bytes.clone());
            }
        }
    } else {
        res.image.set_user_id(img.user_id())
    }

    Ok(res)
}
