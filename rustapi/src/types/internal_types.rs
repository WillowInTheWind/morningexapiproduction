use serde::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate, NaiveDateTime};

#[derive(Debug,Deserialize,Serialize)]
#[allow(non_camel_case_types,non_snake_case)]
pub struct time {
    //Time is a variable that should not be used outside the CalendarEvent type, there are cleaner ways to handle such a type
//naming must be this way for the calendar event type to serialize properly, not worth writing more code just for naming standards
    pub(crate) dateTime: NaiveDateTime,
    pub(crate) timeZone: String
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct AuthRequest {
    pub(crate) code: String,
    state: String,
}

pub struct EnvironmentVariables {
    pub address: String,
    pub port: String,
}

#[derive(Debug)]
pub struct MxQuery {
    pub id: i32,
    pub mx_index: i32,
    pub owner: i32,
    pub date: NaiveDate,
    pub title: String,
    pub description: String
}

pub(crate) trait DateToString {
    fn date_to_long_string(&self) -> String;
    fn date_to_short_string(&self) -> String;
}

impl DateToString for NaiveDate {
    fn date_to_long_string(&self) -> String {
        let month = match self.month() {
            1 => { "January" }
            2 => { "Febuary" }
            3 => { "March" }
            4 => { "April" }
            5 => {"May"}
            6 => {"June"}
            7 => {"July"}
            8 => {"August"}
            9 => {"September"}
            10 => {"October"}
            11 => {"November"}
            12 => {"December"}
            _ => {"Unreachable value"}
        };
        let day = match self.day() {
            1 => {format!("{:?}st",self.day())}
            2 => {format!("{:?}nd",self.day())}
            3 => {format!("{:?}rd",self.day())}
            _ => {format!("{:?}th",self.day())}
        };
        format!("{} {} {}",month, day, self.year()%1000)

    }
    fn date_to_short_string(&self) -> String {
        format!("{}/{}/{}",self.month(), self.day(), self.year()%1000)
    }
}
