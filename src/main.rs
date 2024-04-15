mod jwt;
mod config;
mod routes;
mod services;
mod types;

use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main()
{
        dotenv().ok();
    //Init environment variables and tracing
        tracing_subscriber::fmt::init();
        let database_url =env::var("DATABASE_URL").expect("DATABASE_URL must set");
        let environment_variables = config::initialize_environment_variable()
            .await;
    //Init App State
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("could not connect");

        let oauth_client = config::oauth_client().unwrap();
        let client = reqwest::Client::new();
        let app_state: types::state::AppState  = types::state::AppState {
            dbreference: pool,
            oauth_client,
            reqwest_client: client,
        };
        println!("->> Successful connection to postgres database");
    //Init App routes
        let app_router = routes::router(app_state);
    //Launch Server
        let server_adress: String = format!("{}:{}", environment_variables.address, environment_variables.port);
        let listener = tokio::net::TcpListener::bind(
                server_adress
                )
            .await
            .unwrap();
        println!("->> LISTENING on {:?}\n", listener.local_addr());
        axum::serve(listener, app_router)
            .await
            .unwrap();
}
