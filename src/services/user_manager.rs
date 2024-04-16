use http::StatusCode;
use sqlx::{Pool, Error, Postgres};
use crate::types::data_representations::{ GoogleUser};

pub(crate) trait UserService: Send + Sync {
    async fn get_users(&self) -> Result<Vec<GoogleUser>, sqlx::Error>;
    async fn get_user_by_id(&self, id: i32) -> Result<GoogleUser, sqlx::Error>;
    async fn set_user_phone_number(&self, number: String, id: i32) -> Result<StatusCode, Error>;
    async fn get_user_by_name(&self, name:&str) -> Result<GoogleUser, sqlx::Error>;
    async fn get_user_by_sub(&self, sub:&str) -> Result<GoogleUser, sqlx::Error>;
    async fn get_user_by_email(&self, email:&str) -> Result<GoogleUser, sqlx::Error>;

    async fn create_user(&self, new_user: GoogleUser) -> Result<i32, sqlx::Error>;

    async fn delete_user_by_id(&self, id: i32) -> Result<StatusCode, sqlx::Error>;
    async fn delete_user_by_user_name(&self, name: String) -> Result<StatusCode, sqlx::Error>;

    async fn edit_username(&self, new_user: GoogleUser) ->  Result<GoogleUser, sqlx::Error>;
    async fn reset_user_token(&self, token: String, id: i32) -> Result<StatusCode, Error>;
    async fn delete_user_by_email(&self, email: String) -> Result<StatusCode, Error>;
}
impl UserService for Pool<Postgres> {
    async fn get_users(&self) -> Result<Vec<GoogleUser>, Error> {
        let query: Result<Vec<GoogleUser>, Error> = sqlx::query_as(
             "SELECT * FROM GoogleUsers")
            .fetch_all(self).await;
        query
    }
    async fn get_user_by_id(&self, id: i32) -> Result<GoogleUser, Error> {
        let query = sqlx::query_as(
             "SELECT * FROM GoogleUsers where id = $1")
            .bind( id)
            .fetch_one(self).await;
        query
    }

    async fn set_user_phone_number(&self, number: String, id: i32) -> Result<StatusCode, Error> {
            sqlx::query(
                "Update GoogleUsers SET phone_number = $1 where id = $2",

            )
                .bind(number)
                .bind(id)
                .fetch_one(self)
                .await?
            ;

        Ok(StatusCode::CREATED)
    }
    async fn get_user_by_name(&self, name: &str) -> Result<GoogleUser, Error> {
        let query: Result<GoogleUser, Error> = sqlx::query_as(
             "SELECT * FROM GoogleUsers Where name = $1")
            .bind(name)
            .fetch_one(self)
            .await;
        query
    }

    async fn get_user_by_sub(&self, sub: &str) -> Result<GoogleUser, Error> {
        let query: Result<GoogleUser, Error> = sqlx::query_as(
            "SELECT * FROM GoogleUsers Where sub = $1" ).bind(sub)
            .fetch_one(self).await;
        query
    }

    async fn get_user_by_email(&self, email: &str) -> Result<GoogleUser, Error> {
        let query = sqlx::query_as(
             "SELECT * FROM GoogleUsers Where email = $1").bind( email)
            .fetch_one(self).await;
        query
    }

    async fn create_user(&self, new_user: GoogleUser) -> Result<i32, Error> {
        let token = new_user.token.unwrap().clone();
        let _query =
            sqlx::query(
                r#"INSERT into GoogleUsers (sub, picture, email, name, token) values ($1,$2, $3, $4, $5) RETURNING id
"#,
            ).bind(new_user.sub
            ).bind(new_user.picture
            ).bind(new_user.email
            ).bind(new_user.name
            ).bind(token
            )
                .fetch_one(self)
                .await?;


        Ok(200)
    }

    async fn delete_user_by_id(&self, id: i32) -> Result<StatusCode, Error> {
            sqlx::query(
                "Delete from GoogleUsers where id = $1",
            ).bind(id
            )
                .fetch_one(self)
                .await?;

        Ok(StatusCode::CREATED)
    }

    async fn delete_user_by_user_name(&self, name: String) -> Result<StatusCode, Error> {
            sqlx::query(
                "Delete from GoogleUsers where name = $1",
            ).bind(name
            )
                .fetch_one(self)
                .await?;

        Ok(StatusCode::OK)
    }
    async fn edit_username(&self, _new_user: GoogleUser) -> Result<GoogleUser, Error> {
        todo!()
    }

    async fn reset_user_token(&self, token: String, id: i32) -> Result<StatusCode, Error> {

            sqlx::query(
                "Update GoogleUsers SET token = $1 where id = $2",
            ).bind(token).bind(
                id
            )
                .execute(self)
                .await?
            ;
        Ok(StatusCode::OK)
    }
    async fn delete_user_by_email(&self, email: String) -> Result<StatusCode, Error> {

            sqlx::query(
                "Delete from GoogleUsers where email = $1"
            ).bind(email
            )
                .execute(self)
                .await?
                ;

        Ok(StatusCode::OK)
    }
}
