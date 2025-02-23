use axum::routing::{get, Route};
use axum::Router;
use sea_orm::DatabaseConnection;

use crate::configuration::Configuration;
use crate::api::state::AppState;

mod state;
mod spotify;

pub struct Api {
    // db: DatabaseConnection,
    configuration: Configuration,
}

impl Api {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            // db,
            configuration,
        }
    }
}


impl Into<Router> for Api {
    fn into(self) -> Router {
        // let state = AppState::new(self.db, self.configuration);
        let state: AppState = AppState::new(self.configuration);

        Router::new()
            .route("/oauth/spotify/login", get(spotify::Spotify::redirect_login))
            .route(&state.configuration.spotify_redirect_path, get(spotify::Spotify::callback))
            // .route("/", get(handler::index))
            .with_state(state)
    }
}
