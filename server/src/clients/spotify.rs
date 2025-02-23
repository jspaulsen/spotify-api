use serde::Deserialize;
use crate::configuration::Configuration;


pub struct SpotifyClient {
    client_id: String,
    client_secret: String,
    redirect_url: String,
}


#[derive(Deserialize)]
pub struct SpotifyTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
}


impl SpotifyClient {
    pub fn new(configuration: &Configuration) -> Self {
        Self {
            client_id: configuration.spotify_client_id.clone(),
            client_secret: configuration.spotify_client_secret.clone(),
            redirect_url: configuration.spotify_redirect_url(),
        }
    }

    pub async fn exchange_code(&self, code: &str) -> Result<SpotifyTokenResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let data = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &self.redirect_url),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        let response = client.post("https://accounts.spotify.com/api/token")
            .form(&data)
            .send()
            .await?;
        
        let token_response: SpotifyTokenResponse = response
            .json()
            .await?;

        Ok(token_response)
    }
}
