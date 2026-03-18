use std::{env, str::FromStr};

use anyhow::Result;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl,
    StandardRevocableToken, TokenUrl,
    basic::{
        BasicClient, BasicErrorResponse, BasicRevocationErrorResponse,
        BasicTokenIntrospectionResponse, BasicTokenResponse,
    },
};
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

type GithubClient = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub struct AppState {
    pub db_pool: PgPool,
    pub oauth_client: GithubClient,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        // Database
        let db_options = PgConnectOptions::from_str(env::var("DATABASE_URL")?.as_str())?;
        let pool = PgPoolOptions::new().connect_with(db_options).await?;
        // Run migrations under `migrations/`
        sqlx::migrate!().run(&pool).await?;

        // Authentication
        let gh_client_id = ClientId::new(env::var("GITHUB_CLIENT_ID")?);
        let gh_client_secret = ClientSecret::new(env::var("GITHUB_CLIENT_SECRET")?);
        let gh_auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
        let gh_token_url =
            TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;
        let gh_redirect_url =
            RedirectUrl::new("http://localhost:3000/auth/github/callback".to_string())?;
        let gh_oauth_client = BasicClient::new(gh_client_id)
            .set_client_secret(gh_client_secret)
            .set_auth_uri(gh_auth_url)
            .set_token_uri(gh_token_url)
            .set_redirect_uri(gh_redirect_url);

        Ok(Self {
            db_pool: pool,
            oauth_client: gh_oauth_client,
        })
    }
}
