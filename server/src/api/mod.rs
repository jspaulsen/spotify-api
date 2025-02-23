use axum::routing::Route;
use axum::Router;
use sea_orm::DatabaseConnection;

use crate::configuration::Configuration;
use crate::api::state::AppState;

mod state;


pub struct Api {
    db: DatabaseConnection,
    configuration: Configuration,
}

impl Api {
    pub fn new(db: DatabaseConnection, configuration: Configuration) -> Self {
        Self {
            db,
            configuration,
        }
    }
}


impl Into<Router> for Api {
    fn into(self) -> Router {
        let state = AppState::new(self.db, self.configuration);

        Router::new()
            // .route("/", get(handler::index))
            .with_state(state)
    }
}