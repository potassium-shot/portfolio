use dioxus::{fullstack::Json, prelude::StatusCode};

pub struct Error {
    inner: anyhow::Error,
    http_code: u16,
}

impl Error {
    fn from_error(value: impl std::error::Error + Send + Sync + 'static, http_code: u16) -> Self {
        Self {
            inner: anyhow::Error::from(value),
            http_code,
        }
    }
}

pub trait ResultExt<T> {
    fn error_code(self, http_code: u16) -> Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultExt<T> for std::result::Result<T, E> {
    fn error_code(self, http_code: u16) -> Result<T> {
        self.map_err(|e| Error::from_error(e, http_code))
    }
}

impl dioxus::fullstack::response::IntoResponse for Error {
    fn into_response(self) -> dioxus::fullstack::response::Response {
        let mut resp = Json(ErrorJson {
            error: self.inner.to_string(),
        })
        .into_response();
        *resp.status_mut() = StatusCode::from_u16(self.http_code).unwrap();
        resp
    }
}

impl dioxus::fullstack::AsStatusCode for Error {
    fn as_status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.http_code).unwrap()
    }
}

impl From<Error> for dioxus::fullstack::ServerFnError {
    fn from(value: Error) -> Self {
        Self::ServerError {
            message: value.inner.to_string(),
            code: value.http_code,
            details: None,
        }
    }
}

#[derive(serde::Serialize)]
struct ErrorJson {
    error: String,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
