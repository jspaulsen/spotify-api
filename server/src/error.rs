use axum::{
    http::StatusCode,
    Json,
    response::{
        IntoResponse,
        Response,
    },
};

use serde::Serialize;
use serde_json::json;


#[derive(Debug, Clone, Serialize)]
pub enum HttpError {
    BadRequest(String),
    InternalServerError(String),
    BadGateway(String),
}


pub trait Loggable<T, E> {
    fn log_error<C>(self, context: C) -> Result<T, E>
    where
        C: std::fmt::Display + Send + Sync + 'static;
}


impl<T, E> Loggable<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static {

    fn log_error<C: std::fmt::Display + Send + Sync + 'static>(self, context: C) -> Result<T, E> {
        self
            .map_err(|e| {
                tracing::error!("{} {:#?}", context, e);

                e
            })
    }
}


impl HttpError {
    pub fn internal_server_error<S: AsRef<str>>(message: Option<S>) -> Self {
        let message = message
            .map(|s| s
                .as_ref()
                .to_string()
            ).unwrap_or("Internal Server Error".to_string());

        Self::InternalServerError(message)
    }

    pub fn bad_request<S: AsRef<str>>(message: Option<S>) -> Self {
        let message = message
            .map(|s| s
                .as_ref()
                .to_string()
            ).unwrap_or("Bad Request".to_string());

        Self::BadRequest(message)
    }

    pub fn bad_gateway(message: Option<String>, cause: Option<String>) -> Self {
        let message: String = message
            .unwrap_or("Bad Gateway".to_string());

        let cause: String = cause
            .unwrap_or("Unknown".to_string());

        Self::BadGateway(format!("{}: {}", message, cause))
    }
}

impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> Self {
        let maybe_status = err.status();

        if let Some(status) = maybe_status {
            if status.is_client_error() {
                Self::internal_server_error(Some(err.to_string()))
            } else {
                Self::bad_gateway(Some(err.to_string()), None)
            }
        } else {
            Self::bad_gateway(Some(err.to_string()), None)
        }
    }
}


impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            HttpError::InternalServerError(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
            HttpError::BadRequest(s) => (StatusCode::BAD_REQUEST, s),
            HttpError::BadGateway(s) => (StatusCode::BAD_GATEWAY, s),
        };

        (status_code, Json(json!({"message": message}))).into_response()
    }
}


// impl From<AuthBasicError> for HttpError {
//     fn from(err: AuthBasicError) -> Self {
//         Self::bad_request(Some(err.to_string()))
//     }
// }


// impl From<SqlxError> for HttpError {
//     fn from(_: SqlxError) -> Self {
//         Self::bad_gateway(None)
//     }
// }


// impl From<anyhow::Error> for HttpError {
//     fn from(_: anyhow::Error) -> Self {
//         Self::internal_server_error(None)
//     }
// }
