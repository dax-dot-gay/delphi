use bson::doc;
use chrono::{ DateTime, Utc };
use delphi_macros::model;
use oximod::ModelTrait;
use rocket::{ http::Status, request::{ FromRequest, Outcome }, Request };
use rocket_okapi::OpenApiFromRequest;
use schemars::JsonSchema;
use scrypt::{
    password_hash::{ rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString },
    Scrypt,
};
use serde::{ Deserialize, Serialize };

use crate::ApiError;

#[derive(OpenApiFromRequest)]
#[model(collection = "auth.sessions")]
pub struct Session {
    #[serde(default = "chrono::Utc::now")]
    #[builder(default = chrono::Utc::now())]
    pub created: DateTime<Utc>,

    #[serde(default = "chrono::Utc::now")]
    #[builder(default = chrono::Utc::now())]
    pub last_access: DateTime<Utc>,

    #[serde(default)]
    #[builder(field)]
    pub user_id: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = ApiError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token_cookie) = req.cookies().get_private("_delphi_token") {
            match Session::get(token_cookie.value()).await {
                Ok(Some(session)) => {
                    let updated = session.last_access(Utc::now());
                    if let Err(e) = updated.save().await {
                        return Outcome::Error((Status::InternalServerError, e.into()));
                    }

                    Outcome::Success(updated)
                }
                Err(e) => Outcome::Error((Status::InternalServerError, e.into())),
                Ok(None) => {
                    let session = Session::builder().build();
                    if let Err(e) = session.save().await {
                        return Outcome::Error((Status::InternalServerError, e.into()));
                    }
                    req.cookies().add_private(("_delphi_token", session.id()));
                    Outcome::Success(session)
                }
            }
        } else {
            let session = Session::builder().build();
            if let Err(e) = session.save().await {
                return Outcome::Error((Status::InternalServerError, e.into()));
            }
            req.cookies().add_private(("_delphi_token", session.id()));
            Outcome::Success(session)
        }
    }
}

impl Session {
    pub async fn user(&self) -> Option<User> {
        if let Some(user_id) = self.user_id.clone() {
            if let Ok(Some(user)) = User::get(user_id).await {
                return Some(user);
            }
        }

        None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Password(String);

impl Password {
    pub fn new(plain: impl AsRef<str>) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let passhash = Scrypt.hash_password(plain.as_ref().as_bytes(), &salt)
            .expect("Failed to hash provided password")
            .to_string();
        Self(passhash)
    }

    pub fn verify(&self, test: impl AsRef<str>) -> bool {
        let parsed = PasswordHash::new(&self.0).expect("Failed to parse internal hash");
        Scrypt.verify_password(test.as_ref().as_bytes(), &parsed).is_ok()
    }
}

impl<T: AsRef<str>> From<T> for Password {
    fn from(value: T) -> Self {
        Self::new(value.as_ref().to_string())
    }
}

impl Default for Password {
    fn default() -> Self {
        Self::new("password")
    }
}

#[derive(OpenApiFromRequest)]
#[model(collection = "auth.users")]
pub struct User {
    #[builder(start_fn, into)]
    pub username: String,

    #[builder(start_fn, into)]
    pub password: Password,

    #[builder(default)]
    pub is_admin: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub is_admin: bool,
}

impl User {
    pub fn create(username: impl Into<String>, password: impl Into<Password>) -> Self {
        Self::builder(username, password).build()
    }

    pub fn create_admin(username: impl Into<String>, password: impl Into<Password>) -> Self {
        Self::builder(username, password).is_admin(true).build()
    }

    pub async fn verify(&self, password: impl AsRef<str>) -> bool {
        let this = self.clone();
        let password = password.as_ref().to_string();
        tokio::task::spawn_blocking(move || this.password.verify(password)).await.unwrap_or(false)
    }

    pub fn profile(&self) -> UserProfile {
        UserProfile { id: self.id(), username: self.username.clone(), is_admin: self.is_admin }
    }

    pub async fn get_username(username: impl Into<String>) -> crate::Result<Option<Self>> {
        Ok(Self::find_one(doc! {"username": username.into()}).await?)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ApiError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match Session::from_request(&req).await {
            rocket::outcome::Outcome::Success(session) => {
                if let Some(user_id) = session.user_id.clone() {
                    if let Ok(Some(user)) = User::get(user_id).await {
                        Outcome::Success(user)
                    } else {
                        let mut updated_session = session.clone();
                        updated_session.user_id = None;
                        if let Err(e) = updated_session.save().await {
                            return Outcome::Error((Status::InternalServerError, e.into()));
                        }

                        (ApiError::unauthorized_expects_authenticated(req.uri().path().to_string())).into()
                    }
                } else {
                    (ApiError::unauthorized_expects_authenticated(req.uri().path().to_string())).into()
                }
            }
            rocket::outcome::Outcome::Forward(status) => Outcome::Forward(status),
            rocket::outcome::Outcome::Error((status, err)) => Outcome::Error((status, err)),
        }
    }
}
