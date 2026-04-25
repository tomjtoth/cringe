use std::time::Duration;

async fn _sleep(duration: Duration) {
    #[cfg(feature = "server")]
    tokio::time::sleep(duration).await;

    #[cfg(target_arch = "wasm32")]
    gloo_timers::future::sleep(duration).await;
}

pub async fn sleep(secs: u64) {
    let duration = Duration::from_secs(secs);
    _sleep(duration).await;
}

// pub async fn sleep_ms(millis: u64) {
//     let duration = Duration::from_millis(millis);
//     _sleep(duration).await;
// }
