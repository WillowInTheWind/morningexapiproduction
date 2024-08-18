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
    async fn create_mx(&self, mx: MorningExercise) -> (StatusCode,String);
    async fn delete_mx_by_id(&self, id: i64) -> (StatusCode,String);
    async fn delete_mx_by_title(&self, title: &str) -> (StatusCode,String);
    async fn edit_mx(&self, mx: MorningExercise) -> (StatusCode,String);
    async fn get_mxs_by_sql_filter(&self, query: String) -> Result<Vec<MorningExercise>, (StatusCode,String)>;
    async fn approve_mx_by_id(&self, id: i32) -> (StatusCode,String);
    async fn revoke_mx_by_id(&self, id: i32) -> (StatusCode, String);
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
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR,  "Failed to find User associated with MX".to_string()))?;
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
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR,  "Failed to find User associated with MX".to_string()))?;
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
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR,  "Failed to find User associated with MX".to_string()))?;
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
                    .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR,  "Failed to find User associated with MX".to_string()))?;
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
                .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR,  "Failed to find User associated with MX".to_string()))?;
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
                .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR,  "Failed to find User associated with MX".to_string()))?;
            mxs.push(MorningExercise::new(mx.0,user,mx.2,mx.3,mx.4, mx.5,mx.6, mx.7, mx.8, reqtech, mx.10, editors, mx.12));
        }


        Ok(mxs)
    }
    async fn create_mx(&self, mx: MorningExercise) -> (StatusCode,String) {
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
                (StatusCode::CREATED, "MX was succesfully created and added to Database".to_string())
            }
            Err(query) => {
                types::internal_types::log_server_route(StatusCode::INTERNAL_SERVER_ERROR, &format!("{:?}", query));
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", query))
            }
        }

    }
    async fn delete_mx_by_id(&self, id: i64) -> (StatusCode,String) {
       let query = sqlx::query("Delete FROM MX WHERE id = ?")
            .bind(id)
            .fetch_one(self)
            .await
            .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)
            ;

        match query {
            Ok(_q) => {
                (StatusCode::OK, "an Mx was succesfully Deleted".to_string())
            }
            Err(q) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", q))
            }
         }

    }
    async fn delete_mx_by_title(&self, title: &str) ->  (StatusCode,String) {
        let query = sqlx::query("DELETE FROM MX WHERE title = $1")
            .bind(title)
            .execute(self)
            .await
            ;

        match query {
            Ok(_q) => {
                (StatusCode::OK, format!("{} was successfully Deleted", title))
            }
            Err(q) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("ERROR: {:?}", q))
            }
        }
    }
    async fn edit_mx(&self, mx: MorningExercise) ->  (StatusCode,String) {
        let query = sqlx::query("
        UPDATE MX SET
         date = $1,
         owner = $2,
         Title= $3,
         description= $4,
         min_grade= $5,
         max_grade= $6,
         young_student_prep_instructions= $7,
         is_available_in_day= $8,
         required_tech_json= $9,
         short_description= $10,
         editors_json= $11,
        is_approved = FALSE
         WHERE id = $12")
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
            .fetch_one(self)
            .await
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
            ;

        match query {
            Ok(_q) => {
                (StatusCode::OK, "Succesfully Edited".to_string())
            }
            Err(q) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("ERROR Editing mx - {:?}", q))
            }
        }

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
            .map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "SQL was incorrect or invalid: ".to_string() + &*_e.to_string()))?;

        let mut mxs: Vec<MorningExercise> = Vec::new();
        for mx in query {

            let editors: Vec<i32> =  types::internal_types::string_to_list::<i32>(mx.11).unwrap();
            let reqtech: Vec<String> =types::internal_types::string_to_list::<String>(mx.9).unwrap();
            let user = self.get_user_by_id(mx.1)
                .await
                .map_err(|_err|(StatusCode::INTERNAL_SERVER_ERROR, "Failed to find User associated with MX".to_string()))?;
            mxs.push(MorningExercise::new(mx.0,user,mx.2,mx.3,mx.4, mx.5,mx.6, mx.7, mx.8, reqtech, mx.10, editors,mx.12));

        }
        Ok(mxs)
    }

    async fn approve_mx_by_id(&self, id: i32) -> (StatusCode, String) {
        let query = sqlx::query("UPDATE MX SET is_approved=TRUE WHERE id=$1")
            .bind(id)
            .fetch_one(self)
            .await
            ;

        match query {
            Ok(_q) => {
                (StatusCode::OK, format!("MX #{} was successfully approved", id))
            }
            Err(q) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", q))
            }
        }
    }
    async fn revoke_mx_by_id(&self, id: i32) -> (StatusCode, String) {
        let query = sqlx::query("UPDATE MX SET is_approved=FALSE WHERE id=$1")
            .bind(id)
            .fetch_one(self)
            .await
            ;

        match query {
            Ok(_q) => {
                (StatusCode::OK, format!("MX #{} approval was removed", id))
            }
            Err(q) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", q))
            }
        }
    }
}