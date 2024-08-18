use std::env;
use std::str::from_utf8_mut;
use axum::{Extension, Json};
use axum::extract::{Path, Query, State};
use axum_macros::debug_handler;
use chrono::{Datelike, Days, Local, NaiveDate, Utc};
use sha256::digest;
use http::{HeaderMap, StatusCode};
use serde::Deserialize;
use crate::services::mx_service::MxService;
use crate::types::data_representations::{GoogleUser, MorningExercise};
use crate::types::state::AppState;
use crate::types;
use crate::types::internal_types::{AuthRequest, DateToString};

#[debug_handler]
pub async fn get_planned_mxs(Query(query): Query<ApiKey>, State(app_state): State<AppState>) -> Result<Json<Vec<MorningExercise>>, (StatusCode, String)>
{
    let key = query.sub;

    if (key != env::var("TEMP_MASTER_KEY").unwrap()) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid API key".to_owned()));
    }
    let current_date: NaiveDate = NaiveDate::from_ymd_opt(Local::now().naive_local().year(), Local::now().naive_local().month(), Local::now().naive_local().day()).unwrap();
    let date_filter: String = format!("date >= '{:?}' AND date <= '{:?}' AND is_approved = TRUE ORDER BY date;", current_date.date_to_short_string(), (current_date.checked_add_days(Days::new(28)) ).unwrap().date_to_short_string());
    println!("{}", date_filter);
    let query = app_state.dbreference.get_mxs_by_sql_filter(date_filter).await?;
    types::internal_types::log_server_route(StatusCode::OK, &format!("All Mxs between {:?} and {:?} found", current_date.date_to_short_string(), (current_date.checked_add_days(Days::new(28)) ).unwrap().date_to_short_string()));
    Ok(Json(query))
}

pub async fn generate_api_token(Extension(user): Extension<GoogleUser>,
                                State(AppState): State<AppState>
) -> String
{
    let key = digest(user.sub);
    key.to_owned()
}
#[derive(Deserialize)]
pub struct ApiKey {
    sub: String,
}