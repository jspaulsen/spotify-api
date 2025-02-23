use std::sync::Arc;
use std::collections::HashMap;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

use crate::configuration::Configuration;


#[derive(Clone)]
pub(crate) struct AppState {
    // db: Arc<DatabaseConnection>,
    pub configuration: Configuration,
    pub(crate) state: Arc<Mutex<HashMap<String, String>>>,
}


impl AppState {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            // db: Arc::new(db),
            state: Arc::new(Mutex::new(HashMap::new())),
            configuration,
        }
    }
}
