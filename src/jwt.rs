use axum_extra::extract::CookieJar;
use chrono::{DateTime, Duration, TimeDelta, Utc};
use jsonwebtoken::{encode, Header};
use axum_extra::extract::cookie::{Cookie, SameSite};
use crate::types::errors::AppError;
use crate::config::{KEYS, TOKEN_LENGTH_SECONDS};
use crate::types::data_representations::Claims;

pub async fn create_jwt_token(user_id: i32) -> Result<CookieJar, AppError> {
    let mut now: DateTime<Utc> = Utc::now();
    let expires_in: TimeDelta = Duration::try_seconds(TOKEN_LENGTH_SECONDS).unwrap();
    now += expires_in;
    let exp: usize = now.timestamp() as usize;

    //create JWT claim to be encoded, pointing to the user's id in the Database
    let claims: Claims = Claims {
        sub: user_id,
        exp
    };

    //encode the userid and expiration time
    let jwt_token: String = encode(
        &Header::default(),
        &claims,
        &KEYS.encoding,
    ).unwrap();

    //cookie to store
    let token_cookie:(&str, String) = ("__session", jwt_token);
    let cookie = Cookie::build
        (
        token_cookie
        )
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/");
    let jar = CookieJar::new()
        .add
        (
            cookie
        );

    Ok(jar)
}
pub async fn remove_jwt_token() -> Result<CookieJar, AppError> {
    let jar = CookieJar::new().add(
        Cookie::build(
    ("__session", "")
        )
        .secure(true).path("/")
    );
    Ok(jar)
}