use axum::{
    extract::{Query, State},
    response::Redirect,
};
use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse};
use reqwest::Client;
use serde::Deserialize;
use tower_sessions::Session;

use crate::AppState;

pub async fn github_login(State(state): State<AppState>, session: Session) -> Redirect {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .add_scope(Scope::new("read:user".to_string()))
        .set_pkce_challenge(pkce_challenge) // Send the PKCE challenge
        .url();

    session.insert("csrf_token", csrf_token.secret()).await.unwrap();

    session.insert("pkce_verifier", pkce_verifier.secret()).await.unwrap();

    Redirect::to(auth_url.as_str())
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,  // Temp code sent by Github, expires in 10 minutes, used to get access token
    pub state: String, // Verify if same as before, or else it's tampered / forged
}

pub async fn github_callback(
    State(state): State<AppState>,
    session: Session,
    Query(query): Query<AuthRequest>,
) -> Redirect {
    // Check if the received CSRF token is the same as the one we sent
    let local_state: String = match session.get("csrf_token").await.unwrap() {
        Some(state) => state,
        None => {
            // Log the error and redirect to login
            tracing::error!("No CSRF token found in session");
            return Redirect::to(&format!("{}/?error=session_expired", state.frontend_url));
        }
    };
    if local_state != query.state {
        return Redirect::to(&format!("{}/?error=invalid_csrf_token", state.frontend_url));
    }

    // Use oauth2's reqwest here
    let oauth2_http_client = oauth2::reqwest::ClientBuilder::new()
        // According to oauth2 docs: "Following redirects opens the client up to SSRF vulnerabilities."
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // Send the local PKCE verifier. This will run in the previously sent PKCE challenge on GitHub servers
    // to check if the client is still who we claim to be.
    let local_pkce_verifier = session.get("pkce_verifier").await.unwrap().unwrap();
    let to_be_sent_pkce_verifier = PkceCodeVerifier::new(local_pkce_verifier);

    // Obtain access to access token.
    let token_response = state
        .oauth_client
        .exchange_code(oauth2::AuthorizationCode::new(query.code))
        .set_pkce_verifier(to_be_sent_pkce_verifier)
        .request_async(&oauth2_http_client)
        .await
        .unwrap();

    let client = Client::new();

    let gh_user = client
        .get("https://api.github.com/user")
        .header("User-Agent", "town-hall (by zerodrag/town-hall")
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let (github_id, handle, email) = (
        gh_user["id"].as_i64().unwrap(),
        gh_user["name"].as_str().unwrap(),
        gh_user["email"].as_str().unwrap(),
    );

    let internal_user_id: i64 = sqlx::query_scalar!(
        "INSERT INTO users (github_id, handle, email) \
        VALUES ($1, $2, $3) \
        ON CONFLICT (github_id) \
        DO UPDATE SET \
            handle = EXCLUDED.handle, \
            email = EXCLUDED.email \
        RETURNING user_id",
        github_id,
        handle,
        email
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap();

    session.insert("user_id", internal_user_id).await.unwrap();

    Redirect::to(&format!("{}/user/{}", state.frontend_url, handle))
}

pub async fn logout(State(state): State<AppState>, session: Session) -> Redirect {
    session.clear().await;
    Redirect::to(&state.frontend_url)
}
