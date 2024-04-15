use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use axum_macros::debug_handler;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::services::user_manager::UserService;
use crate::types::state::AppState;
use crate::types::data_representations::{GoogleUser};


pub async fn delete_user() -> Response {
    todo!()
}
pub async fn get_user_by_id(Path(params): Path<i32>,State(state): State<AppState>,) -> Json<GoogleUser> {
    println!("->> User get request by id");

    Json(state.dbreference.get_user_by_id(params).await.unwrap())
}
#[debug_handler]
pub async fn get_user_by(Query(params): Query<GetUserBy>,State(state): State<AppState>,) -> Result<Json<GoogleUser>, StatusCode> {
    println!("->> User get request by {}", params.user_property);

    match params.user_property.as_str(){
        "email" => {
            Ok(Json(state.dbreference.get_user_by_email(&params.property).await.unwrap()))
        }
        "sub" => {
            Ok(Json(state.dbreference.get_user_by_sub(&params.property).await.unwrap()))        }
        "name" => {
            let name = params.property.replace("%20", " ").replace("/", "");
            Ok(Json(state.dbreference.get_user_by_name(&name).await.unwrap()) )       }
        _ => {Err(StatusCode::NOT_FOUND)}
    }
}
#[derive(Deserialize, Serialize)]
pub struct  GetUserBy {
    user_property: String,
    property: String
}
#[debug_handler]
pub async fn get_all_users(
    State(state): State<AppState>,
) -> Json<Vec<GoogleUser>> {
    println!("->> User get request");

    Json(state.dbreference.get_users().await.map_err(
        |err|
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    ).unwrap())
}

pub async fn current_user(Extension(user): Extension<GoogleUser>) -> Json<GoogleUser> {
    Json(user)
}
#[debug_handler]
pub async fn set_user_number(Extension(user): Extension<GoogleUser>, State(state): State<AppState>, Json(payload): Json<PhoneNumber>) -> Result<Response, StatusCode> {
    println!("->> user {} tried to set phone number", user.name);

    if payload.number.chars().count() != 10 && payload.number.chars().count() != 12 {
        return Err(StatusCode::NOT_ACCEPTABLE)
    }
    if !payload.number  .parse::<i64>().is_ok() {
        println!("->> phone number was not number");
        return Err(StatusCode::NOT_ACCEPTABLE)
    }
    println!("->> user {} was succesfully set", user.name);

    let request = state.dbreference.set_user_phone_number(payload.number, user.id.unwrap()).await.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
   Ok(Json(request).into_response())
}
#[derive(Deserialize, Serialize)]
pub struct PhoneNumber {
    number: String
}