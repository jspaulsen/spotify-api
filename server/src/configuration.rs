use envconfig::Envconfig;
use tracing::Level;


#[derive(Clone)]
#[derive(Envconfig)]
pub struct Configuration {
    #[envconfig(from = "DATABASE_URL", default = "TODO: Add default value")] // TODO: Add default value
    pub database_url: String,

    #[envconfig(from = "HTTP_PORT", default = "8080")]
    pub http_port: u16,

    #[envconfig(from = "HTTP_HOST", default = "0.0.0.0")]
    pub http_host: String,

    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub log_level: Level,

    #[envconfig(from = "SPOTIFY_CLIENT_ID")]
    pub spotify_client_id: String,
    
    #[envconfig(from = "SPOTIFY_CLIENT_SECRET")]
    pub spotify_client_secret: String,

    /* Used for building the redirect URI. */
    #[envconfig(from = "BASE_URI", default = "http://127.0.0.1:8080")]
    pub base_uri: String,

    #[envconfig(from = "SPOTIFY_REDIRECT_PATH", default = "/oauth/spotify/callback")]
    pub spotify_redirect_path: String,
}

impl Configuration {
    pub fn spotify_redirect_url(&self) -> String {
        let spotify_path = self
            .spotify_redirect_path
            .trim_start_matches('/');

        format!(
            "{}/{}", 
            self.base_uri,
            spotify_path,
        )
    }
}
