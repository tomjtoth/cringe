mod bots;

use dioxus::{
    fullstack::TypedHeader,
    prelude::{
        dioxus_fullstack::{extract::Extension, FullstackContext},
        *,
    },
};
use sqlx::PgPool;
use tokio::sync::mpsc::Sender;

use crate::{auth::COOKIE_NAME, state::image::init_converter};

async fn get_db() -> sqlx::PgPool {
    let Extension(pool) = FullstackContext::extract()
        .await
        .expect("PgPool extension is missing from server context");

    pool
}

#[get("/api/healthz")]
async fn healthz() -> Result<()> {
    let pool = get_db().await;
    sqlx::query("SELECT 1").execute(&pool).await?;

    Ok(())
}

pub async fn get_ctx() -> (Option<String>, sqlx::PgPool) {
    futures::join!(get_session_id(), get_db())
}

async fn get_cookies() -> Option<axum_extra::headers::Cookie> {
    let TypedHeader(cookies) =
        FullstackContext::extract::<TypedHeader<axum_extra::headers::Cookie>, _>()
            .await
            .ok()?;

    return Some(cookies);
}

async fn get_session_id() -> Option<String> {
    let cookies = get_cookies().await?;
    let id = cookies.get(COOKIE_NAME)?;

    Some(String::from(id))
}

pub async fn init() -> anyhow::Result<(PgPool, Sender<i32>)> {
    dotenvy::dotenv().ok();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    sqlx::migrate!().run(&pool).await?;

    if let Err(e) = bots::seed(&pool).await {
        error!("Failed to load bots.yaml: {}", e);
    }

    let tx = init_converter(pool.clone());

    Ok((pool, tx))
}
