use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::configuration::Configuration;


#[derive(Clone)]
pub(crate) struct AppState {
    db: Arc<DatabaseConnection>,
    configuration: Configuration,
}


impl AppState {
    pub fn new(db: DatabaseConnection, configuration: Configuration) -> Self {
        Self {
            db: Arc::new(db),
            configuration,
        }
    }
}