use std::env;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use anyhow::Context;
use once_cell::sync::Lazy;
use crate::types::errors::AppError;
use crate::types::data_representations::Keys;
use crate::types::internal_types::EnvironmentVariables;


pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
pub static TOKEN_LENGTH_SECONDS: i64 = 12*60*60;
pub static MORNING_EX_ADMIN_ACCOUNT: i32 = 8;


pub async fn initialize_environment_variable() -> EnvironmentVariables {
    let address: String = match env::var("HOST") {
        Ok(address) => {address}
        _ => {String::from("127.0.0.1")}
    };
    let port: String = match env::var("PORT") {
        Ok(port) => { port }
        _ => {"8080".to_string()}
    };
    EnvironmentVariables {
        address,
        port,
    }
}



pub fn oauth_client() -> anyhow::Result<BasicClient, AppError> {
    dotenv::dotenv().ok();
    let client_id: ClientId = ClientId::new(env::var("CLIENT_ID").unwrap());
    let client_secret: ClientSecret = ClientSecret::new(env::var("CLIENT_SECRET").unwrap());
    let redirect_url: String = env::var("REDIRECT_URL").unwrap().to_string();
    let redirect_uri: RedirectUrl =RedirectUrl::new(redirect_url).context("failed to create new redirection URL")?;
    let auth_url = "https://accounts.google.com/o/oauth2/v2/auth".to_string();
    let auth_url: AuthUrl = AuthUrl::new(auth_url)
        .expect("Invalid authorization endpoint URL");
    let token_url = "https://www.googleapis.com/oauth2/v3/token".to_string();
    let token_url: TokenUrl = TokenUrl::new(token_url)
        .expect("Invalid token endpoint URL");


    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
        .set_redirect_uri(
            redirect_uri,
        );

    Ok(client)
}

