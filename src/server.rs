use std::time::Duration;
use leptos::*;


#[cfg(feature = "ssr")]
pub fn random_millis() -> Duration {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_number: u64 = rng.gen_range(25..=500);
    Duration::from_millis(random_number)
}

#[server]
pub async fn server_fn_1() -> Result<String, ServerFnError> {
    tracing::debug!("server_fn_1 called");
    tokio::time::sleep(random_millis()).await;
    Ok("Hello1".to_string())
}

#[server]
pub async fn server_fn_2() -> Result<String, ServerFnError> {
    tracing::debug!("server_fn_2 called");
    tokio::time::sleep(random_millis()).await;
    Ok("Hello2".to_string())
}

#[server]
pub async fn server_fn_3() -> Result<String, ServerFnError> {
    tracing::debug!("server_fn_3 called");
    tokio::time::sleep(random_millis()).await;
    Ok("Hello3".to_string())
}

