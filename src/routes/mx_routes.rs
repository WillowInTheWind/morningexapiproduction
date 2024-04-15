use std::string::String;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use axum_macros::debug_handler;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::types::state::AppState;
use crate::services::mx_service::MxService;
use crate::services::calendar_service::CalendarService;
use crate::services::user_manager::UserService;

use crate::types::data_representations::{GoogleUser, MorningExercise};

#[debug_handler]
pub async fn get_all_mxs(
    State(app_state): State<AppState>
) -> Json<Vec<MorningExercise>> {

    println!("->> MX get request");
    Json(app_state.dbreference.get_mxs().await.unwrap())
}
pub async fn get_users_mxs(
    Extension(user): Extension<GoogleUser>,
    State(state): State<AppState>
) -> Json<Vec<MorningExercise>> {
    println!("->> MX get request by users");

    Json(state.dbreference.get_mxs_by_owner(user.id.unwrap()).await.unwrap())   
}
#[debug_handler]
pub async fn post_mx(State(state): State<AppState>,
                     Extension(user): Extension<GoogleUser>,
                     Json(payload): Json<MxPost>) -> StatusCode {
    println!("->> MX post request");

    let mx = MorningExercise::new_with_date(
        1, user.clone(), payload.date, payload.title, payload.description, None
    );


    state.dbreference.create_mx(mx).await
}
#[debug_handler]
pub async fn approve_mx(State(state): State<AppState>,
                     Extension(user): Extension<GoogleUser>,
                     Json(payload): Json<Mxcalendarbody>) -> StatusCode {
    println!("->> MX approval");


    let morningex = state.dbreference.get_mx_by_title(&payload.title).await;
    if morningex.is_err() {
        return StatusCode::UNAUTHORIZED
    }
    let mx = morningex.unwrap();


    let statuscode = state.reqwest_client.mxtocalendar(user, mx.clone()).await.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR);
    if statuscode == Ok(StatusCode::CREATED) {
        println!("->> Calendar Event Created");
        statuscode.unwrap()
    }
    else {
        println!("->> Calendar Event failed");
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_user_mxs_by_name(Path(params): Path<String>, State(state): State<AppState>) -> Json<Vec<MorningExercise>>  {
    let user = state.dbreference.get_user_by_name(&params).await.unwrap();
    Json(state.dbreference.get_mxs_by_owner(user.id.unwrap()).await.unwrap())
}
pub async fn delete_mx(State(state): State<AppState>,
                       Json(payload): Json<MorningExercise>) -> Response {
    println!("->> MX delete request");
    let mx_id = payload.title;
    state.dbreference.delete_mx_by_title(&mx_id).await.into_response()
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MxPost {
    date: chrono::NaiveDate,
    title: String,
    description: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct  Mxcalendarbody {
    title: String
}