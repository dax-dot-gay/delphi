use chrono::{DateTime, Utc};
use delphi_macros::model;
use oximod::ModelTrait;
use rocket::{http::Status, request::{FromRequest, Outcome}, Request};
use rocket_okapi::OpenApiFromRequest;

use crate::ApiError;

#[derive(OpenApiFromRequest)]
#[model(collection = "auth.sessions")]
pub struct Session {
    #[serde(default = "chrono::Utc::now")]
    #[builder(default = chrono::Utc::now())]
    pub created: DateTime<Utc>,

    #[serde(default = "chrono::Utc::now")]
    #[builder(default = chrono::Utc::now())]
    pub last_access: DateTime<Utc>
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
                },
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