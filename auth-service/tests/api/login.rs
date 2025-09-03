use crate::helpers::TestApp;
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let login_body = serde_json::json!({
        "BADemail": "user@example.com",
        "BADpassword": "password123"
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 422);
}

//...

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;
    let login_body = serde_json::json!({
        "email": "user@example.com",
        "password": "short"
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;
    let login_body = serde_json::json!({
        "email": "user@example.com",
        "password": "wrongpassword"
    });

    let response = app.post_login(&login_body).await;

    let responseCode = response.status().as_u16();

    assert_eq!(responseCode, 401);
}
