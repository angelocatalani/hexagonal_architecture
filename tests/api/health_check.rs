use crate::api::helpers::spawn_app;

#[actix_rt::test]
async fn health_check_works() {
    let health_check_endpoint = format!("http://127.0.0.1:{}/health_check", spawn_app().await);
    let client = reqwest::Client::new();
    let response = client.get(&health_check_endpoint).send().await.unwrap();
    assert!(response.status().is_success());
}
