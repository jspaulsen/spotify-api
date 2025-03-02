use sea_orm::{
    ConnectOptions,
    Database,
    DatabaseConnection,
    DbErr,
};
use sqlx::{
    Error,
    migrate::Migrator,
    postgres::PgPoolOptions,
    sqlite::SqlitePoolOptions,
};

use crate::configuration::Configuration;


pub static MIGRATE: Migrator = sqlx::migrate!();


pub async fn get_db_connection(config: &Configuration) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(config.database_url.to_owned());

    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    
    Database::connect(opt).await
}


// pub async fn migrate(config: &Configuration) -> Result<(), Error> {
//     let pool = PgPoolOptions::new()
//         .connect(&config.database_url)
//         .await?;
    
//     MIGRATE.run(&pool)
//         .await
//         .map_err(|e| Error::Migrate(Box::new(e)))
// }