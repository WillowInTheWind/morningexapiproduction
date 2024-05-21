use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use http::{header, StatusCode};
use jsonwebtoken::{decode, Validation};
use crate::services::user_manager::UserService;
use crate::types::state::AppState;
use axum_extra::extract::cookie::CookieJar;
use crate::config::KEYS;
use crate::types;
use crate::types::data_representations::{Claims};
pub async fn auth(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {

    let token = get_token_cookie(cookie_jar, &req);
    let token = token.ok_or_else(|| {
        {
            types::internal_types::log_server_route(StatusCode::UNAUTHORIZED, "Failed to find JWT token in users cookies");
            StatusCode::UNAUTHORIZED
        }
    })?;

    let claims = decode::<Claims>(
        &token,
        &KEYS.decoding,
        &Validation::default(),
    )
        .map_err(|_e| {
            {
                types::internal_types::log_server_route(StatusCode::UNAUTHORIZED, &format!("Non Admin user attempted to reach '{}' and server failed to decode JWT token", req.uri()));
                StatusCode::UNAUTHORIZED
            }
        })?
        .claims;

    let user_id = claims.sub;
    let user = state.dbreference.get_user_by_id(user_id).await.map_err(|_e|
        {
            types::internal_types::log_server_route(StatusCode::UNAUTHORIZED, "User does not exist");
            StatusCode::UNAUTHORIZED
        }

    )?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn userisadmin(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = get_token_cookie(cookie_jar, &req);
    let token = token.ok_or_else(|| {
        {
            types::internal_types::log_server_route(StatusCode::UNAUTHORIZED, "Failed to find JWT token in users cookies");
            StatusCode::UNAUTHORIZED
        }
    })?;

    let claims = decode::<Claims>(
        &token,
        &KEYS.decoding,
        &Validation::default(),
    )
        .map_err(|_e| {
            {
                types::internal_types::log_server_route(StatusCode::UNAUTHORIZED, &format!("Non Admin user attempted to reach '{}' and server failed to decode JWT token", req.uri()));
                StatusCode::UNAUTHORIZED
            }
        })?
        .claims;

    let user_id = claims.sub;
    let user = state.dbreference.get_user_by_id(user_id).await.map_err(|_e|
        {
            types::internal_types::log_server_route(StatusCode::UNAUTHORIZED, "User does not exist");
            StatusCode::UNAUTHORIZED
        }

    )?;

    if user.email != "wayland.chase@gmail.com" {
        types::internal_types::log_server_route(StatusCode::UNAUTHORIZED, &format!("Non Admin user attempted to reach '{}'", req.uri()));
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

fn get_token_cookie(cookie_jar: CookieJar, req: &Request) -> Option<String> {
    let token  = cookie_jar
        .get("__session")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });
    token
}