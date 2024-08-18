use http::StatusCode;
use oauth2::basic::BasicClient;
use oauth2::{RefreshToken, TokenResponse};
use oauth2::reqwest::async_http_client;
use tracing_subscriber::fmt::format;
use crate::services::cycle_manager;
use crate::types::data_representations::{CalendarEvent, GoogleUser, MorningExercise};

pub trait CalendarService {
    async fn mx_to_calendar(&self, client: BasicClient, user: GoogleUser, mx: MorningExercise) -> Result<StatusCode, StatusCode>;
}

impl CalendarService for reqwest::Client {
     async fn mx_to_calendar(&self, client: BasicClient, user: GoogleUser, mx: MorningExercise) -> Result<StatusCode, StatusCode> {
         // let date = mx.date.and_hms_opt(10,50,0);
         // let enddate = mx.date.and_hms_opt(11,30,0);
         // let title = format!("Day {:?} - {}", cycle_manager::day_to_cycle(date.unwrap()), mx.title);
         // let description = mx.description + "/" + "Sponsored by" + &mx.owner.name;
         // let event = CalendarEvent::new(title, date.unwrap(), enddate.unwrap(), description);
         // if user.token.is_none() {
         //     return Err(StatusCode::UNAUTHORIZED);
         // }
         // let token = client
         //     .exchange_refresh_token(&RefreshToken::new(user.token.unwrap()))
         //     .request_async(async_http_client)
         //     .await
         //     .expect("Failed to exchange refresh token.");
         //
         // let _user_calendar  = self
         //     .post(format!("https://www.googleapis.com/calendar/v3/calendars/{}/events",user.email))
         //     .bearer_auth(token.access_token().secret().as_str())
         //     .json(&event)
         //     .send()
         //     .await
         //     .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

         Ok(StatusCode::CREATED)
     }


}