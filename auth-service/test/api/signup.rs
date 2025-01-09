use crate::helpers::TestApp;

#[tokio::test]
async fn signup_returns_200() {
    let app = TestApp::new().await;

    let response = app.post_signup(serde_json::json!({
        "email": "email@example.com",
        "password": "password",
    })).await;

    assert_eq!(response.status().as_u16(), 200);
}