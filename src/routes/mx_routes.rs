use std::string::String;
use async_session::hmac::digest::generic_array::typenum::private::IsNotEqualPrivate;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use axum_macros::debug_handler;
use axum::http::StatusCode;
use chrono::NaiveDate;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::query;
use crate::routes::user_routes::GetUserBy;
use crate::types::state::AppState;
use crate::services::mx_service::MxService;
use crate::services::calendar_service::CalendarService;
use crate::services::user_manager::UserService;
use crate::{config, types};

use crate::types::data_representations::{GoogleUser, MorningExercise};
use crate::types::internal_types::string_to_list;

#[debug_handler]
pub async fn filter_mxs_by_sql(State(state): State<AppState>,
                               Query(params): Query<Filter>
) -> Result<Json<Vec<MorningExercise>>, (StatusCode, String)> {
    // let mut name = params.filter.replace("%20", " ");

    let query = state.dbreference.get_mxs_by_sql_filter(params.filter.clone()).await?;
    types::internal_types::log_server_route(StatusCode::OK, &format!("Mxs requested with query {})",params.filter));
    Ok(Json(query))
}
pub async fn all_consumed_dates(Extension(state): Extension<AppState>) -> Json<NaiveDate> {

    Json(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
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
pub async fn edit_mx(
    Extension(user): Extension<GoogleUser>,
    State(state): State<AppState>,
    Json(payload): Json<MxEdit>) -> (StatusCode, String) {
        let mx: MorningExercise = state.dbreference.get_mx_by_id(payload.id).await.unwrap();
        if !(user.name == mx.owner.name || mx.editors_json.contains(&user.id.unwrap()) ||  (user.is_admin.is_some() && user.is_admin.unwrap())) {
            return (StatusCode::UNAUTHORIZED, "You do not have permission to edit mx".to_string())
        }
        let mut reqtech = string_to_list(payload.required_tech_json).unwrap();
        reqtech.retain(|x| x!="");
    let mut editors = string_to_list(payload.editors_json).unwrap();

        let new_mx = MorningExercise::new(mx.id,
                                          mx.owner,
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
                                          false);
        let query = state.dbreference.edit_mx(new_mx).await;
        types::internal_types::log_server_route(StatusCode::CREATED, &format!("User {} edited an Mx: {}", user.name.bright_blue(), query.1));
    (StatusCode::CREATED, "Mx edited".to_string())
}
#[debug_handler]
pub async fn post_mx(State(state): State<AppState>,
                     Extension(user): Extension<GoogleUser>,
                     Json(payload): Json<MxPost>) -> impl IntoResponse {

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
    let query = state.dbreference.create_mx(mx).await;
    (query.0, Json(query.1).into_response())
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


    let status_code = state.reqwest_client.mx_to_calendar(state.oauth_client.clone(), user.clone(), mx.clone()).await.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR);
    let mx_status_code = state.reqwest_client.mx_to_calendar(state.oauth_client, state.dbreference.get_user_by_id(config::MORNING_EX_ADMIN_ACCOUNT).await.unwrap(), mx.clone()).await.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR);
    if status_code == Ok(StatusCode::CREATED) {
        types::internal_types::log_server_route(StatusCode::CREATED, &format!("MX added to {} calendar", user.name.bright_blue()));
        status_code.unwrap()
    }
    else {
        types::internal_types::log_server_route(StatusCode::INTERNAL_SERVER_ERROR, "MX failed to be added to users calendar");
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[debug_handler]
pub async fn revoke_mx(State(state): State<AppState>,
                        Extension(user): Extension<GoogleUser>,
                        Json(payload): Json<Mxcalendarbody>) -> StatusCode {
    types::internal_types::log_server_route(StatusCode::CREATED, "An MX was revoke");

    let morning_ex = state.dbreference.get_mx_by_title(&payload.title).await;
    if morning_ex.is_err() {
        return StatusCode::UNAUTHORIZED
    }
    let mx = morning_ex.unwrap();
    let revoke_mx = state.dbreference.revoke_mx_by_id(mx.id).await;

    if revoke_mx.0 == StatusCode::OK {
        types::internal_types::log_server_route(StatusCode::CREATED, &format!("MX removed from {}s calendar", user.name.bright_blue()));
        revoke_mx.0
    }
    else {
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
                       Path(params): Path<i64>) -> (StatusCode, String) {
    let mx_id = params;
    let query = state.dbreference.delete_mx_by_id(mx_id).await;
    if query.0 == StatusCode::INTERNAL_SERVER_ERROR {
        types::internal_types::log_server_route(StatusCode::OK, &format!("Mx failed to be  deleted - {:?}", query.1));
    }
    query
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MxEdit {
    id: i64,
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