use sqlx::types::Json;

use crate::{
    models::person::Prompt,
    state::{crud_query::crud_query, prompt::PromptOpResult, server::ServerCtx},
};

pub(in crate::state) async fn prompt_crud(
    ctx: &ServerCtx,
    prompt: Prompt,
) -> anyhow::Result<PromptOpResult> {
    let Json(mut res) = sqlx::query_scalar::<_, Json<PromptOpResult>>(&crud_query(true))
        .bind(&ctx.session_id)
        .bind(&prompt.id)
        .bind(&prompt.position)
        .bind(&prompt.title)
        .bind(&prompt.body)
        .fetch_one(&ctx.pool)
        .await?;

    if !res.authorized {
        res.prompt.user_id = prompt.user_id;
    }

    Ok(res)
}
