use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{ NaiveDate, NaiveDateTime};
use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MorningExercise {
    //TODO: add editor value so multiple people can edit the same mx
    pub id: i32,
    pub date: NaiveDate,
    pub owner: GoogleUser,
    pub title: String,
    pub description: String,
    // new shit
    pub min_grade: i32,
    pub max_grade: i32,
    pub young_student_prep_instructions: String,
    pub is_available_in_day: bool,
    pub required_tech_json: Vec<String>,
    pub short_description: String,
    pub editors_json: Vec<GoogleUser>,
}
#[derive(FromRow, Debug, Deserialize, Serialize, Clone  )]
pub struct  GoogleUser {
    pub id: Option<i32>,
    pub sub: String,
    pub picture: Option<String>,
    pub email: String,
    pub name: String,
    pub token: Option<String>,
    pub phone_number: Option<String>
}

#[derive(Debug,Deserialize,Serialize)]
pub struct CalendarEvent {
    summary: String,
    start: crate::types::internal_types::time,
    end: crate::types::internal_types::time,
    description: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

/*
    Below  are the constructors and methods of the above types
*/

impl CalendarEvent {
    pub fn new(summary: String, start: NaiveDateTime, end: NaiveDateTime, description: String) -> Self {
        CalendarEvent{
            summary,
            start: crate::types::internal_types::time {
                dateTime: start, timeZone: "America/Chicago".to_string()
            },
            end: crate::types::internal_types::time {
                dateTime: end, timeZone: "America/Chicago".to_string()
            },
            description,
        }
    }
}

impl MorningExercise {
    //constructors

    pub fn new(id:i32,
               owner: GoogleUser,
               date: NaiveDate,
               title: String,
               description: String,
                min_grade: i32,
                max_grade: i32,
                young_student_prep_instructions: String,
                is_available_in_day: bool,
                required_tech_json: Vec<String>,
                short_description: String,
                editors_json: Vec<GoogleUser>
    ) -> Self {
        MorningExercise {
            id,
            date,
            owner,
            title,
            description,
            min_grade,
            max_grade,
            young_student_prep_instructions,
            is_available_in_day,
            required_tech_json,
            short_description,
            editors_json,
        }
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub(crate) fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

