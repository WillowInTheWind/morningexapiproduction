mod jwt;
mod config;
mod routes;
mod services;
mod types;

use std::env;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main()
{
        println!("->> Server Init Beginning");
        dotenv::dotenv().ok();
    //Init environment variables and tracing
        tracing_subscriber::fmt::init();
        let oauth_client = config::oauth_client().unwrap();

        let environment_variables = config::initialize_environment_variable()
            .await;
        let database_url =env::var("DATABASE_URL").expect("DATABASE_URL must set");
    //Init App State
        println!("->> Enviorment variables initilized");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("could not connect");
        println!("->> Successful connection to postgres database");

        let client = reqwest::Client::builder().use_rustls_tls().build().unwrap()
            ;
        println!("->> HTTP client initialized");
        let app_state: types::state::AppState  = types::state::AppState {
            dbreference: pool,
            oauth_client,
            reqwest_client: client,
        };
    //Init App routes
        let app_router = routes::router(app_state);
        println!("->> Router initialized");
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
