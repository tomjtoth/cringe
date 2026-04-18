pub async fn sleep(secs: u64) {
    let duration = std::time::Duration::from_secs(secs);

    #[cfg(feature = "server")]
    tokio::time::sleep(duration).await;

    #[cfg(target_arch = "wasm32")]
    gloo_timers::future::sleep(duration).await;
}

pub fn random_id() -> u128 {
    #[allow(unused_mut)]
    let mut buf = [0u8; 16];

    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        let window = window().expect("no window");
        let crypto = window.crypto().expect("no crypto");
        crypto
            .get_random_values_with_u8_array(&mut buf)
            .expect("failed to get random");
    }

    u128::from_le_bytes(buf)
}
