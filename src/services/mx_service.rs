use axum::http::StatusCode;
use chrono::NaiveDate;
use sqlx::{Pool, Postgres};
use tokio::io::AsyncBufReadExt;
use crate::services::user_manager::UserService;
use crate::types;
use crate::types::data_representations::{GoogleUser, MorningExercise};
use crate::types::internal_types::list_to_string;

pub trait MxService {
    async fn get_mx_by_id(&self, id: i64) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_date(&self, date: NaiveDate) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_index(&self, index: i64) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_title(&self, title: &str) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mxs_by_owner(&self, owner_id: i32) -> Result<Vec<MorningExercise>, (StatusCode, String)>;
    async fn get_mxs(&self) -> Result<Vec<MorningExercise>, (StatusCode, String)>;
    async fn create_mx(&self, mx: MorningExercise) -> StatusCode;
    async fn delete_mx_by_id(&self, id: i64) -> StatusCode;
    async fn delete_mx_by_title(&self, title: &str) -> StatusCode;
    async fn edit_mx(&self, mxid: i32) -> StatusCode;
    async fn get_mxs_by_sql_filter(&self, query: String) -> Result<Vec<MorningExercise>, (StatusCode,String)>;
    async fn approve_mx_by_id(&self, id: i32) -> StatusCode;
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
                            String,bool), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE id = ?")
            .bind(id)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let editors: Vec<i32> =  types::internal_types::string_to_list::<i32>(query.11).unwrap();
                let reqtech: Vec<String> =types::internal_types::string_to_list::<String>(query.9).unwrap();
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4, query.5,query.6, query.7, query.8, reqtech, query.10, editors,query.12)
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
                            String, bool), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE date = ?")
            .bind(date)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let editors: Vec<i32> =  types::internal_types::string_to_list::<i32>(query.11).unwrap();
                let reqtech: Vec<String> =types::internal_types::string_to_list::<String>(query.9).unwrap();
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4, query.5,query.6, query.7, query.8, reqtech, query.10, editors,query.12)
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
                            String, bool), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE mx_index = ?")
            .bind(index)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let editors: Vec<i32> =  types::internal_types::string_to_list::<i32>(query.11).unwrap();
                let reqtech: Vec<String> =types::internal_types::string_to_list::<String>(query.9).unwrap();
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4, query.5,query.6, query.7, query.8, reqtech, query.10, editors,query.12)
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
                            String,
                            bool), _> = sqlx::query_as
            (&format!("SELECT * FROM MX WHERE title = '{}'",title))
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let editors: Vec<i32> =  types::internal_types::string_to_list::<i32>(query.11).unwrap();
                let reqtech: Vec<String> =types::internal_types::string_to_list::<String>(query.9).unwrap();

                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4, query.5,query.6, query.7, query.8, reqtech, query.10, editors,query.12)
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
                         String,bool)> = sqlx::query_as("SELECT * FROM MX WHERE owner = $1").bind(owner_id)
            .fetch_all(self)
            .await
            .map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "query failed".to_string()))?;

        let mut mxs: Vec<MorningExercise> = Vec::new();
        for mx in query {

            let editors: Vec<i32> =  types::internal_types::string_to_list::<i32>(mx.11).unwrap();
            let reqtech: Vec<String> =types::internal_types::string_to_list::<String>(mx.9).unwrap();
            let user = self.get_user_by_id(mx.1)
                .await
                .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
            mxs.push(MorningExercise::new(mx.0,user,mx.2,mx.3,mx.4, mx.5,mx.6, mx.7, mx.8, reqtech, mx.10, editors,mx.12));

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
            String, bool)> = sqlx::query_as("SELECT * FROM MX")
            .fetch_all(self)
            .await
            .map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "query failed".to_string()))?;

        // println!("dfs");

        let mut mxs: Vec<MorningExercise> = Vec::new();
        for mx in query {
            let editors: Vec<i32> =  types::internal_types::string_to_list::<i32>(mx.11).unwrap();
            let reqtech: Vec<String> =types::internal_types::string_to_list::<String>(mx.9).unwrap();
            let user = self.get_user_by_id(mx.1)
                .await
                .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
            mxs.push(MorningExercise::new(mx.0,user,mx.2,mx.3,mx.4, mx.5,mx.6, mx.7, mx.8, reqtech, mx.10, editors, mx.12));
        }


        Ok(mxs)
    }
    async fn create_mx(&self, mx: MorningExercise) -> StatusCode {
        let date: NaiveDate = mx.date ;
        let owner: i32 = mx.owner.id.unwrap();

        let query = sqlx::query(
            r#"INSERT into MX (date,
                owner,
                Title,
                description,
                min_grade,
                max_grade,
                young_student_prep_instructions,
                is_available_in_day,
                required_tech_json,
                short_description,
                editors_json,
                is_approved) VALUES ($1,$2,$3,$4, $5, $6,$7,$8,$9, $10, $11, $12)"#)
                .bind(mx.date)
                .bind(mx.owner.id.unwrap())
                .bind(mx.title)
                .bind(mx.description)
                .bind(mx.min_grade)
                .bind(mx.max_grade)
                .bind(mx.young_student_prep_instructions)
                .bind(mx.is_available_in_day)
                .bind(list_to_string(mx.required_tech_json))
                .bind(mx.short_description)
                .bind(list_to_string(mx.editors_json))
                .bind(mx.is_approved)
            .execute(self)
            .await
       ;

        match query {
            Ok(_query) => {
                StatusCode::CREATED
            }
            Err(query) => {
                types::internal_types::log_server_route(StatusCode::INTERNAL_SERVER_ERROR, &format!("{:?}", query));
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
        todo!()
        // let mx = self.get_mx_by_id(mxid as i64).await.unwrap();
        // let editors = mx.editors_json;
        // let query = sqlx::query("
        // UPDATE MX SET
        //  date = $1,
        //  owner = $2,
        //  Title= $3,
        //  description= $4,
        //  min_grade= $5,
        //  max_grade= $6,
        //  young_student_prep_instructions= $7,
        //  is_available_in_day= $8,
        //  required_tech_json= $9,
        //  short_description= $10,
        //  editors_json= $11
        //  WHERE id = $12")
        //     .bind(mx.date)
        //     .bind(mx.owner.id.unwrap())
        //     .bind(mx.Title)
        //     .bind(mx.description)
        //     .bind(mx.min_grade)
        //     .bind(mx.max_grade)
        //     .bind(mx.young_student_prep_instructions)
        //     .bind(mx.is_available_in_day)
        //     .bind(mx.required_tech_json)
        //     .bind(mx.short_description)
        //     .bind(editors)
        //     .fetch_one(self)
        //     .await
        //     .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
        //     ;
        //
        // match query {
        //     Ok(_q) => {
        //         StatusCode::OK
        //     }
        //     Err(_q) => {
        //         StatusCode::INTERNAL_SERVER_ERROR
        //     }
        // }

    }

    async fn get_mxs_by_sql_filter(&self, query: String) -> Result<Vec<MorningExercise>, (StatusCode, String)> {
        let query : Vec<(i32, i32, NaiveDate, String, String,
                         i32,
                         i32,
                         String,
                         bool,
                         String,
                         String,
                         String,bool)> = sqlx::query_as(&format!("SELECT * FROM MX WHERE {}",query))
            .fetch_all(self)
            .await
            .map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "query failed".to_string()))?;

        let mut mxs: Vec<MorningExercise> = Vec::new();
        for mx in query {

            let editors: Vec<i32> =  types::internal_types::string_to_list::<i32>(mx.11).unwrap();
            let reqtech: Vec<String> =types::internal_types::string_to_list::<String>(mx.9).unwrap();
            let user = self.get_user_by_id(mx.1)
                .await
                .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
            mxs.push(MorningExercise::new(mx.0,user,mx.2,mx.3,mx.4, mx.5,mx.6, mx.7, mx.8, reqtech, mx.10, editors,mx.12));

        }
        Ok(mxs)
    }

    async fn approve_mx_by_id(&self, id: i32) -> StatusCode {
        let query = sqlx::query("UPDATE MX SET is_approved=TRUE WHERE id=$1")
            .bind(id)
            .fetch_one(self)
            .await
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