mod jwt;
mod config;
mod routes;
mod services;
mod types;

use std::env;
use colored::Colorize;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main()
{
        dotenv::dotenv().ok();
        tracing_subscriber::fmt::init();

        let oauth_client = config::oauth_client().unwrap();
        let environment_variables = config::initialize_environment_variable()
            .await;
        let database_url =env::var("DATABASE_URL").expect("DATABASE_URL must set");

        println!("\n \n \n{}", "->> Enviorment variables found".yellow());

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();
        let _ = sqlx::migrate!("./migrations").run(&pool).await;

        let server_local = &database_url[database_url.find("@").unwrap_or(database_url.len())..];
        println!("{} {}", "->> Opened connectiom to postgreSQL database at".red(), server_local.bright_blue());

        let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();
        let app_state: types::state::AppState  = types::state::AppState {
                dbreference: pool,
                oauth_client,
                reqwest_client: client,
        };
        let app_router = routes::router(app_state);
        let server_address: String = format!("{}:{}", environment_variables.address, environment_variables.port);
        let listener = tokio::net::TcpListener::bind(server_address)
            .await
            .unwrap();

        println!("{} {} \n", "->> LISTENING on".purple(), listener.local_addr().unwrap().to_string().bright_blue());

        axum::serve(listener, app_router)
            .await
            .unwrap();

}
