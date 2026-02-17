use axum::http::{StatusCode, Error as AxumError};
use axum::Json;
use serde_json::json;
use std::fmt;
use std::io::Error as IOError;
#[derive(Debug)]
pub enum Error 
{
    InternalServer(String),
}

//This one is for responses, so we can display it html
impl axum::response::IntoResponse for Error
{
    fn into_response(self) -> axum::response::Response
    {
        let(status, error_message) = match self
        {
            Self::InternalServer(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!(
        {
            "error": error_message,
        }
        ));

        (status, body).into_response()
    }
}

//This one is used for Display the errors in CLI
impl std::fmt::Display for Error 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self 
        {
            Self::InternalServer(msg) => write!(f, "Error: {}", msg)
        }
    }
}

impl From<IOError>for Error 
{
    fn from(err: IOError) -> Self 
    {
        Self::InternalServer(err.to_string())
    }
}

impl From<AxumError> for Error 
{
    fn from(err: AxumError) -> Self 
    {
        Self::InternalServer(err.to_string())
    }
}

impl core::error::Error for Error {}

