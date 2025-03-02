use std::collections::HashMap;

use axum::{
    extract::{Query, State}, http::StatusCode, response::{
        IntoResponse, Redirect, Response
    }
};

use crate::{
    api::state::AppState,
    clients::spotify::SpotifyClient, 
    error::HttpError,
};

// endpoint that redirects to spotify login page
// endpoint that receives the callback from spotify
// endpoint that returns the player state
pub struct Spotify {}


impl Spotify {
    pub async fn redirect_login(state: State<AppState>) -> Redirect {
        let spotify_scopes: &str = "user-read-playback-state user-read-currently-playing"; /* TODO: We may want to configure the scopes as well.*/
        let redirect_url = format!(
            "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope={}",
            state.configuration.spotify_client_id,
            state.configuration.spotify_redirect_url(),
            spotify_scopes,
        );

        Redirect::to(&redirect_url)
    }

    pub async fn callback(
        state: State<AppState>,
        Query(params): Query<HashMap<String, String>>
    ) -> Result<impl IntoResponse, HttpError> {
        let maybe_error = params.get("error");
        let spotify_client = SpotifyClient::new(&state.configuration);

        /* TODO: This should probably redirect to the application in some fashion / form / capacity */
        if let Some(error) = maybe_error {
            return Err(HttpError::internal_server_error(Some(error))); // We probably don't want to return the error to the user.
        }

        let code = params.get("code")
            .ok_or(HttpError::BadRequest("Missing code parameter".to_string()))?;

        let token_response = spotify_client
            .exchange_code(code)
            .await
            .map_err(HttpError::from)?;

        /* TODO: store the access and refresh tokens somewhere */
        Ok(
            Response::builder()
                .status(StatusCode::OK)
                .body(
                    format!("Access Token: {}", token_response.access_token)
                )
                .unwrap()
        )
    }

    pub async fn player_state() {
        // return the player state
    }
}
