

use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{ NaiveDate, NaiveDateTime};
use jsonwebtoken::{DecodingKey, EncodingKey};
use crate::types::mx_date_algorithm;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MorningExercise {
    //TODO: add editor value so multiple people can edit the same mx
    id: i32,
    pub mx_index: i32,
    pub date: NaiveDate,
    pub owner: GoogleUser,
    pub title: String,
    pub description: String,
}
#[derive(FromRow, Debug, Deserialize, Serialize, Clone)]
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
    pub fn new_with_date(id:i32,
                         owner: GoogleUser,
                         date: NaiveDate,
                         title: String,
                         description: String,
                         _editors: Option<Vec<GoogleUser>>)
                         -> Self {
        MorningExercise {
            id,
            date,
            mx_index: mx_date_algorithm::weekly_date_to_index() as i32,
            owner,
            title ,
            description ,
        }
    }
    pub fn new(id:i32,
               owner: GoogleUser,
               mx_index: i32,
               date: NaiveDate,
               title: String,
               description: String,
               _editors: Option<GoogleUser>)
               -> Self {
        MorningExercise {
            id,
            mx_index,
            date,
            owner,
            title ,
            description ,
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

