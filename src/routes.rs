mod user_routes;
mod login_routes;
mod mx_routes;
mod default_routes;
mod middlewares;

use axum::{middleware, Router};
use axum::routing::{delete, get, post};
use http::{HeaderValue, Method};
use crate::types::state::AppState;
use tower_http::cors::CorsLayer;
use crate::types::errors;

pub fn router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap());


    let mx_routes = Router::new()
        .route("/", get(mx_routes::get_all_mxs))
        .layer(middleware::from_fn_with_state(app_state.clone(), middlewares::userisadmin))
        .route("/:username", get(mx_routes::get_user_mxs_by_name))
        .route("/approve", post(mx_routes::approve_mx))
        .route("/create", post(mx_routes::post_mx))
        .route("/delete", delete(mx_routes::delete_mx))
        .route("/mine", get(mx_routes::get_users_mxs));


    let user_routes = Router::new()
        .route("/currentuser", get(user_routes::current_user))
        .route("/setphonenumber", post(user_routes::set_user_number))
        .layer(middleware::from_fn_with_state(app_state.clone(), middlewares::auth))
        .route("/", get(user_routes::get_all_users))
        .route("/getbyid/:id", get(user_routes::get_user_by_id))
        .route("/getuserby", get(user_routes::get_user_by))
        .route("/delete", delete(user_routes::delete_user));



    let auth_routes = Router::new()
        .route("/logout", get(login_routes::logout))
        .route("/login", get(login_routes::login))
        .route("/authorized", get(login_routes::login_authorized));



    let routes = Router::new()
        .route("/", get(default_routes::root))
        .nest("/morningexercises", mx_routes)
        .layer(middleware::from_fn_with_state(app_state.clone(), middlewares::auth))
        .nest("/users", user_routes)
        .nest("/auth", auth_routes)
        .fallback(errors::error_404)
        .with_state(app_state);

    Router::new().nest("/api", routes).layer(cors)
}
