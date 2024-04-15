use http::StatusCode;
use crate::types::data_representations::{CalendarEvent, GoogleUser, MorningExercise};

pub trait CalendarService {
    async fn mxtocalendar(&self,user: GoogleUser, mx: MorningExercise) -> Result<StatusCode, StatusCode>;
}

impl CalendarService for reqwest::Client {
     async fn mxtocalendar(&self, user: GoogleUser, mx: MorningExercise) -> Result<StatusCode, StatusCode> {
         let date = mx.date.and_hms_opt(10,50,0);
         let enddate = mx.date.and_hms_opt(11,30,0);

         let description = mx.description + "/" + "Sponsored by" + &user.name;
         let event = CalendarEvent::new(mx.title, date.unwrap(), enddate.unwrap(), description);
         if user.token.is_none() {
             return Err(StatusCode::UNAUTHORIZED);
         }
         let _user_calendar  = self
             .post(format!("https://www.googleapis.com/calendar/v3/calendars/{}/events",user.email))
             .bearer_auth(user.token.unwrap())
             .json(&event)
             .send()
             .await
             .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

         println!("->> Calendar created ");
         Ok(StatusCode::CREATED)
     }
 }