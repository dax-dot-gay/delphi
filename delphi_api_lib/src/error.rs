use rocket::Responder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, thiserror::Error, Clone, Debug, Responder)]
#[response(content_type = "json")]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ApiError {
    #[response(status = 500)]
    #[error("Internal error: {reason}")]
    Internal {
        reason: String
    }
}

impl<T: Into<Error>> From<T> for ApiError {
    fn from(value: T) -> Self {
        let intermediate: Error = value.into();
        Self::Internal { reason: format!("{intermediate:?}") }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MongoDB ORM error: {0:?}")]
    Oximod(#[from] oximod_core::error::oximod_error::OxiModError),
    #[error("MongoDB error error: {0:?}")]
    Mongodb(#[from] oximod::_mongodb::error::Error),
    #[error("BSON serialization error: {0:?}")]
    BsonSerialization(#[from] bson::ser::Error)
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ApiResult<T> = std::result::Result<T, ApiError>;