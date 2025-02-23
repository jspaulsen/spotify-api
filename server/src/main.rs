use axum::Router;
use envconfig::Envconfig;

use configuration::Configuration;

mod api;
mod configuration;
mod clients;
mod database;
mod error;


const VERSION: &str = env!("CARGO_PKG_VERSION");


#[tokio::main]
async fn main() {
    let configuration = Configuration::init_from_env()
        .expect("Failed to load configuration!");

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            configuration
                .log_level
                .to_string()
        )
        .with_current_span(false)
        .init();

    tracing::info!("Service Version: {}", VERSION);

    // let db_connection = database::get_db_connection(&configuration)
    //     .await
    //     .expect("Failed to setup database connection!");
    
    // run database migrations as part of application startup
    // database::migrate(&configuration)
    //     .await
    //     .expect("Failed to run database migrations!");

    let bind_to = format!("{}:{}", configuration.http_host, configuration.http_port);

    tracing::info!("Starting server on {}", bind_to);
    let api = api::Api::new(
        // db_connection,
        configuration
    );

    let listener = tokio::net::TcpListener::bind(&bind_to)
        .await
        .expect("Failed to bind to address!");

    let router: Router = api.into();

    axum::serve(listener, router)
        .await
        .expect("Failed to start server!");
}
