mod user_routes;
mod login_routes;
mod mx_routes;
mod default_routes;
mod middlewares;
mod api_routes;

use std::env;
use axum::{middleware, Router};
use axum::routing::{delete, get, post};
use http::{HeaderValue, Method};
use http::header::{ACCEPT, AUTHORIZATION};
use crate::types::state::AppState;
use tower_http::cors::{Any, CorsLayer};
use crate::types::errors;


pub fn router(app_state: AppState) -> Router {
    let approved_cors = env::var("REDIRECT_URL").unwrap();

    let origins = [env::var("REDIRECT_URL").unwrap().strip_suffix("/auth").unwrap().to_string().parse::<HeaderValue>().unwrap(),
        "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        "https://witskilldule.web.app/".parse::<HeaderValue>().unwrap()];
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_origin(origins);

    let mx_routes = Router::new()
        .route("/", get(mx_routes::get_all_mxs))
        .layer(middleware::from_fn_with_state(app_state.clone(), middlewares::userisadmin))
        .route("/:username", get(mx_routes::get_user_mxs_by_name))
        .route("/approve", post(mx_routes::approve_mx))
        .route("/revoke", post(mx_routes::revoke_mx))
        .route("/edit", post(mx_routes::edit_mx))
        .route("/create", post(mx_routes::post_mx))
        .route("/delete/:mx", delete(mx_routes::delete_mx))
        .route("/getbytitle", get(mx_routes::get_user_mx_by_title))
        .route("/mine", get(mx_routes::get_users_mxs))
        .route("/filterby", get(mx_routes::filter_mxs_by_sql));

    let api_routes = Router::new()
        .route("/upcoming", get(api_routes::get_planned_mxs));

    let user_routes = Router::new()
        // .route("/makeuseradmin/:id", post(user_routes::make_admin_user()))
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
        .nest("/external", api_routes)
        .fallback(errors::error_404)
        .with_state(app_state);

    Router::new().nest("/api", routes).layer(cors)
}
