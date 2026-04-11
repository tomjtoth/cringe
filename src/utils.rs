pub async fn sleep(secs: u64) {
    let duration = std::time::Duration::from_secs(secs);

    #[cfg(feature = "server")]
    tokio::time::sleep(duration).await;

    #[cfg(target_arch = "wasm32")]
    gloo_timers::future::sleep(duration).await;
}
