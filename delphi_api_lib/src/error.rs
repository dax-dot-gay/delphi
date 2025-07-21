use delphi_macros::http_error;

use crate::components::ComponentError;

#[http_error]
pub enum ApiError {
    #[err(code = 500, message = "Internal error: {reason}")] 
    Internal {
        reason: String,
    },

    #[err(code = 404, message = "Unknown username/password: {user}:***")] 
    InvalidLogin {
        user: String,
    },

    #[err(code = 401, message = "Endpoint expects an authenticated user: {path}")] 
    ExpectsAuthenticated {
        path: String,
    },

    #[err(code = 405, message = "Already logged in!")]
    LoggedIn {
        id: String
    },

    #[err(code = 500, message = "Component execution error in a {component_kind} component: {error:?}")] ComponentError {
        component_kind: String,
        error: super::ComponentError
    }
}

impl<T: Into<Error>> From<T> for ApiError {
    fn from(value: T) -> Self {
        let intermediate: Error = value.into();
        match intermediate {
            Error::ComponentError { component_kind, error } => Self::internal_server_error_component_error(component_kind, error),
            other => Self::internal_server_error_internal(other.to_string())
        }
    }
}

impl<S> Into<rocket::request::Outcome<S, ApiError>> for ApiError {
    fn into(self) -> rocket::request::Outcome<S, ApiError> {
        let status = self.code();
        rocket::request::Outcome::Error((status, self))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MongoDB ORM error: {0:?}")] Oximod(
        #[from] oximod_core::error::oximod_error::OxiModError,
    ),
    #[error("MongoDB error error: {0:?}")] Mongodb(#[from] oximod::_mongodb::error::Error),
    #[error("BSON serialization error: {0:?}")] BsonSerialization(#[from] bson::ser::Error),
    #[error("Component execution error in a {component_kind} component: {error:?}")] ComponentError {
        component_kind: String,
        error: ComponentError
    }
}

impl<T: Into<ComponentError>> From<T> for Error {
    fn from(value: T) -> Self {
        let intermediate: ComponentError = value.into();
        Self::ComponentError { component_kind: intermediate.component_kind(), error: intermediate }
    }
}

impl Error {
    pub fn into_request_error<S>(self) -> rocket::request::Outcome<S, ApiError> {
        ApiError::from(self).into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ApiResult<T> = std::result::Result<T, ApiError>;
