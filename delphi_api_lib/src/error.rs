use rocket::{ http::Status, serde::json::Json, Responder };
use rocket_okapi::response::OpenApiResponderInner;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use strum::EnumProperty;

#[derive(Serialize, Deserialize, thiserror::Error, Clone, Debug, Responder, EnumProperty, JsonSchema)]
#[response(content_type = "json")]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ApiError {
    #[strum(props(code = 500))] #[response(status = 500)] #[error(
        "Internal error: {reason}"
    )] Internal {
        reason: String,
    },

    #[strum(props(code = 404))] #[response(status = 404)] #[error(
        "Unknown username/password: {user}:***"
    )] InvalidLogin {
        user: String,
    },

    #[strum(props(code = 401))] #[response(status = 401)] #[error(
        "Endpoint expects an authenticated user: {path}"
    )] ExpectsAuthenticated {
        path: String,
    },

    #[strum(props(code = 405))] #[response(status = 405)] #[error(
        "Already logged in!"
    )] LoggedIn {
        id: String
    }
}

impl<T: Into<Error>> From<T> for ApiError {
    fn from(value: T) -> Self {
        let intermediate: Error = value.into();
        Self::Internal { reason: format!("{intermediate:?}") }
    }
}

impl<S> Into<rocket::request::Outcome<S, ApiError>> for ApiError {
    fn into(self) -> rocket::request::Outcome<S, ApiError> {
        let status = Status::new(u16::try_from(self.get_int("code").unwrap_or(500)).unwrap_or(500));
        rocket::request::Outcome::Error((status, self))
    }
}

impl OpenApiResponderInner for ApiError {
    fn responses(generator: &mut rocket_okapi::r#gen::OpenApiGenerator) -> rocket_okapi::Result<okapi::openapi3::Responses> {
        <Json<ApiError> as OpenApiResponderInner>::responses(generator)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MongoDB ORM error: {0:?}")] Oximod(
        #[from] oximod_core::error::oximod_error::OxiModError,
    ),
    #[error("MongoDB error error: {0:?}")] Mongodb(#[from] oximod::_mongodb::error::Error),
    #[error("BSON serialization error: {0:?}")] BsonSerialization(#[from] bson::ser::Error),
}

impl Error {
    pub fn into_request_error<S>(self) -> rocket::request::Outcome<S, ApiError> {
        ApiError::from(self).into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ApiResult<T> = std::result::Result<T, ApiError>;
