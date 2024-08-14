use std::env;
use std::str::from_utf8_mut;
use axum::{Extension, Json};
use axum::extract::{Path, Query, State};
use chrono::{Datelike, Days, NaiveDate, Utc};
use sha256::digest;
use http::{HeaderMap, StatusCode};
use crate::services::mx_service::MxService;
use crate::types::data_representations::{GoogleUser, MorningExercise};
use crate::types::state::AppState;
use crate::types;
use crate::types::internal_types::DateToString;

pub async fn get_planned_mxs(headers: HeaderMap, State(app_state): State<AppState>) -> Result<Json<Vec<MorningExercise>>, (StatusCode, String)>
{
    let key = headers.get("x-key");
    if !(key.is_some()) {
        return Err((StatusCode::UNAUTHORIZED, "no key".to_owned()));
    }
    if (key.unwrap().to_str().unwrap() != env::var("TEMP_MASTER_KEY").unwrap()) {
        return Err((StatusCode::UNAUTHORIZED, "no key".to_owned()));
    }
    let current_date: NaiveDate = NaiveDate::from_ymd_opt(Utc::now().naive_utc().year(), Utc::now().naive_utc().month(), Utc::now().naive_utc().day()).unwrap();
    let date_filter: String = format!("date <= '{:?}' AND date >= '{:?}'", current_date.date_to_short_string(), (current_date.checked_add_days(Days::new(28)) ).unwrap().date_to_short_string());
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