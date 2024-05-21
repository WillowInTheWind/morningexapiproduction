use axum::http::StatusCode;
use chrono::NaiveDate;
use sqlx::{Pool, Postgres};
use crate::services::user_manager::UserService;
use crate::types::data_representations::{GoogleUser, MorningExercise};

pub trait MxService {
    async fn get_mx_by_id(&self, id:i64) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_date(&self, date: NaiveDate) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_index(&self, index: i64) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_title(&self, title: &str) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mxs_by_owner(&self, owner_id: i32) -> Result<Vec<MorningExercise>, (StatusCode, String)>;
    async fn get_mxs(&self) ->Result<Vec<MorningExercise>, (StatusCode, String)>;
    async fn create_mx(&self, mx: MorningExercise) -> StatusCode;
    async fn delete_mx_by_id(&self, id: i64) -> StatusCode;
    async fn delete_mx_by_title(&self, title: &str) -> StatusCode;
    async fn edit_mx(&self, mxid: i32) ->  StatusCode;
}
impl MxService for Pool<Postgres> {
    async fn get_mx_by_id(&self, id: i64) -> Result<MorningExercise, (StatusCode, String)> {
        let query : Result<(i32, i32, NaiveDate, String, String,
                            i32,
                            i32,
                            String,
                            bool,
                            String,
                            String,
                            String), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE id = ?")
            .bind(id)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let editors: Vec<String> = serde_json::from_str(&query.9).unwrap();
                let reqtech: Vec<GoogleUser> = serde_json::from_str(&query.11).unwrap();
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4, query.5,query.6, query.7, query.8, editors, query.10, reqtech)
            }
            Err(_e) => {
                return Err((StatusCode::NOT_FOUND, "No such MXs".to_string()))
            }
        };
        Ok(mx)
    }
    async fn get_mx_by_date(&self, date: NaiveDate) -> Result<MorningExercise, (StatusCode, String)> {
        let query : Result<(i32, i32, NaiveDate, String, String,
                            i32,
                            i32,
                            String,
                            bool,
                            String,
                            String,
                            String), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE date = ?")
            .bind(date)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let editors: Vec<String> = serde_json::from_str(&query.9).unwrap();
                let reqtech: Vec<GoogleUser> = serde_json::from_str(&query.11).unwrap();
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4, query.5,query.6, query.7, query.8, editors, query.10, reqtech)
            }
            Err(_query) => {
                return Err((StatusCode::NOT_FOUND, "No such MXs".to_string()))
            }
        };
        Ok(mx)
    }
    async fn get_mx_by_index(&self, index: i64) -> Result<MorningExercise, (StatusCode, String)> {
        let query : Result<(i32, i32, NaiveDate, String, String,
                            i32,
                            i32,
                            String,
                            bool,
                            String,
                            String,
                            String), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE mx_index = ?")
            .bind(index)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let editors: Vec<String> = serde_json::from_str(&query.9).unwrap();
                let reqtech: Vec<GoogleUser> = serde_json::from_str(&query.11).unwrap();
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4, query.5,query.6, query.7, query.8, editors, query.10, reqtech)
            }
            Err(_e) => {
                return Err((StatusCode::NOT_FOUND, "No such MXs".to_string()))
            }
        };
        Ok(mx)
    }
    async fn get_mx_by_title(&self, title: &str) -> Result<MorningExercise, (StatusCode, String)> {
        let query : Result<(i32, i32, NaiveDate, String, String,
                            i32,
                            i32,
                            String,
                            bool,
                            String,
                            String,
                            String), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE title = ?")
            .bind(title)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let editors: Vec<String> = serde_json::from_str(&query.9).unwrap();
                let reqtech: Vec<GoogleUser> = serde_json::from_str(&query.11).unwrap();
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4, query.5,query.6, query.7, query.8, editors, query.10, reqtech)
            }
            Err(_e) => {
                return Err((StatusCode::NOT_FOUND, "No such MXs".to_string()))
            }
        };
        Ok(mx)
    }
    async fn get_mxs_by_owner(&self, owner_id: i32) -> Result<Vec<MorningExercise>, (StatusCode, String)> {
        let query : Vec<(i32, i32, NaiveDate, String, String,
                         i32,
                         i32,
                         String,
                         bool,
                         String,
                         String,
                         String)> = sqlx::query_as("SELECT * FROM MX")
            .fetch_all(self)
            .await
            .map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "query failed".to_string()))?;

        let mut mxs: Vec<MorningExercise> = Vec::new();
        for mx in query {

            let editors: Vec<String> = serde_json::from_str(&mx.9).unwrap();
            let reqtech: Vec<GoogleUser> = serde_json::from_str(&mx.11).unwrap();
            let user = self.get_user_by_id(mx.1)
                .await
                .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
            mxs.push(MorningExercise::new(mx.0,user,mx.2,mx.3,mx.4, mx.5,mx.6, mx.7, mx.8, editors, mx.10, reqtech));

        }
        Ok(mxs)

    }

    async fn get_mxs(&self) -> Result<Vec<MorningExercise>, (StatusCode, String)> {
        let query : Vec<(i32, i32, NaiveDate, String, String,
                         i32,
            i32,
            String,
            bool,
            String,
            String,
            String)> = sqlx::query_as("SELECT * FROM MX")
            .fetch_all(self)
            .await
            .map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "query failed".to_string()))?;

        // println!("dfs");

        let mut mxs: Vec<MorningExercise> = Vec::new();
        for mx in query {
            let editors: Vec<String> = serde_json::from_str(&mx.9).unwrap();
            let reqtech: Vec<GoogleUser> = serde_json::from_str(&mx.11).unwrap();
            let user = self.get_user_by_id(mx.1)
                .await
                .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
            mxs.push(MorningExercise::new(mx.0,user,mx.2,mx.3,mx.4, mx.5,mx.6, mx.7, mx.8, editors, mx.10, reqtech));
        }


        Ok(mxs)
    }
    async fn create_mx(&self, mx: MorningExercise) -> StatusCode {
        let date: NaiveDate = mx.date ;
        let owner: i32 = mx.owner.id.unwrap();
        let title: String = mx.title;
        let description: String = mx.description;

        let query = sqlx::query(
            r#"INSERT into MX (mx_index, date,owner, title,description) VALUES ($1,$2,$3,$4, $5)"#).bind(
            1).bind(
            date).bind(
            owner).bind(
            title).bind(
            description)
            .execute(self)
            .await
       ;

        match query {
            Ok(_query) => {
                StatusCode::CREATED
            }
            Err(_query) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }

    }
    async fn delete_mx_by_id(&self, id: i64) -> StatusCode {
       let query = sqlx::query("Delete FROM MX WHERE id = ?")
            .bind(id)
            .fetch_one(self)
            .await
            .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)
            ;

        match query {
            Ok(_q) => {
                StatusCode::OK
            }
            Err(_q) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
         }

    }
    async fn delete_mx_by_title(&self, title: &str) -> StatusCode {
        let query = sqlx::query("Delete FROM MX WHERE id = ?")
            .bind(title)
            .fetch_one(self)
            .await
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
            ;

        match query {
            Ok(_q) => {
                StatusCode::OK
            }
            Err(_q) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
    async fn edit_mx(&self, mxid: i32) -> StatusCode {
        let query = sqlx::query("Delete FROM MX WHERE id = ?")
            .bind(mxid)
            .fetch_one(self)
            .await
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
            ;

        match query {
            Ok(_q) => {
                StatusCode::OK
            }
            Err(_q) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }

    }
}