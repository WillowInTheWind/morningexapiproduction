use std::string::String;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use axum_macros::debug_handler;
use axum::http::StatusCode;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use crate::routes::user_routes::GetUserBy;
use crate::types::state::AppState;
use crate::services::mx_service::MxService;
use crate::services::calendar_service::CalendarService;
use crate::services::user_manager::UserService;
use crate::types;

use crate::types::data_representations::{GoogleUser, MorningExercise};

#[debug_handler]
pub async fn filter_mxs_by_sql(State(state): State<AppState>,
                               Query(params): Query<Filter>
) -> Result<Json<Vec<MorningExercise>>, (StatusCode, String)> {
    // let mut name = params.filter.replace("%20", " ");

    let query = state.dbreference.get_mxs_by_sql_filter(params.filter.clone()).await?;
    types::internal_types::log_server_route(StatusCode::OK, &format!("Mxs requested with query {})",params.filter));
    Ok(Json(query))
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    filter: String
}

/// Returns all MXs using the Morning_Ex_service
#[debug_handler]
pub async fn get_all_mxs(
    State(app_state): State<AppState>
) -> Json<Vec<MorningExercise>> {

    types::internal_types::log_server_route(StatusCode::OK, "All Mxs requested");
    Json(app_state.dbreference.get_mxs().await.unwrap())
}
pub async fn get_users_mxs(
    Extension(user): Extension<GoogleUser>,
    State(state): State<AppState>
) -> Json<Vec<MorningExercise>> {
    types::internal_types::log_server_route(StatusCode::OK, &format!("User {} requested their Mxs", user.name.bright_blue()));

    Json(state.dbreference.get_mxs_by_owner(user.id.unwrap()).await.unwrap())
}
#[debug_handler]
pub async fn post_mx(State(state): State<AppState>,
                     Extension(user): Extension<GoogleUser>,
                     Json(payload): Json<MxPost>) -> StatusCode {

    let editors = types::internal_types::string_to_list( payload.editors_json).unwrap();

    let reqtech = types::internal_types::string_to_list( payload.required_tech_json).unwrap();

    let mx = MorningExercise::new(
        1,
        user.clone(),
        payload.date,
        payload.title,
        payload.description,
        payload.min_grade,
        payload.max_grade,
        payload.young_student_prep_instructions,
        payload.is_available_in_day,
        reqtech,
        payload.short_description,
        editors,
        false
    );

    types::internal_types::log_server_route(StatusCode::CREATED, &format!("User {} posted a new Mx", user.name.bright_blue()));
    state.dbreference.create_mx(mx).await
}
#[debug_handler]
pub async fn approve_mx(State(state): State<AppState>,
                     Extension(user): Extension<GoogleUser>,
                     Json(payload): Json<Mxcalendarbody>) -> StatusCode {
    types::internal_types::log_server_route(StatusCode::CREATED, "A new MX was approved");

    let morningex = state.dbreference.get_mx_by_title(&payload.title).await;
    if morningex.is_err() {
        return StatusCode::UNAUTHORIZED
    }
    let mx = morningex.unwrap();
    state.dbreference.approve_mx_by_id(mx.id).await;


    let statuscode = state.reqwest_client.mxtocalendar( state.oauth_client, user.clone(), mx.clone()).await.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR);
    if statuscode == Ok(StatusCode::CREATED) {
        types::internal_types::log_server_route(StatusCode::CREATED, &format!("MX added to {} calendar", user.name.bright_blue()));
        statuscode.unwrap()
    }
    else {
        types::internal_types::log_server_route(StatusCode::INTERNAL_SERVER_ERROR, "MX failed to be added to users calendar");
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_user_mxs_by_name(Path(params): Path<String>, State(state): State<AppState>) -> Json<Vec<MorningExercise>>  {
    let user = state.dbreference.get_user_by_name(&params).await.unwrap();
    types::internal_types::log_server_route(StatusCode::OK, &format!("Mxs for user {} queried", user.name.bright_blue()));
    Json(state.dbreference.get_mxs_by_owner(user.id.unwrap()).await.unwrap())
}

pub async fn get_user_mx_by_title(Query(params): Query<title>, State(state): State<AppState>) -> Json<MorningExercise>  {
    types::internal_types::log_server_route(StatusCode::OK, &format!("Mxs with Title {} queried", params.name.bright_blue()));
    Json(state.dbreference.get_mx_by_title(&params.name).await.unwrap())
}
#[derive(Debug, Serialize, Deserialize)]
pub struct title {
    name: String
}

#[debug_handler]
pub async fn delete_mx(State(state): State<AppState>,
                       Json(payload): Json<MorningExercise>) -> Response {
    let mx_id = payload.title;
    types::internal_types::log_server_route(StatusCode::OK, &format!("Mx {}, was deleted", mx_id.bright_blue()));
    state.dbreference.delete_mx_by_title(&mx_id).await.into_response()
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MxPost {
    date: chrono::NaiveDate,
    title: String,
    description: String,
    min_grade: i32,
    max_grade: i32,
    young_student_prep_instructions: String,
    is_available_in_day: bool,
    required_tech_json: String,
    short_description: String,
    editors_json: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct  Mxcalendarbody {
    title: String
}