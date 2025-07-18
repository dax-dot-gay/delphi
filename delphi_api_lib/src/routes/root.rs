use chrono::Utc;
use okapi::openapi3::OpenApi;
use rocket::{delete, get, post, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes_spec};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{models::{Session, User, UserProfile}, ApiResult};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ApiStatus {
    pub session: Session,
    pub user: Option<UserProfile>,
    pub time: chrono::DateTime<Utc>
}

#[openapi(tag = "Root")]
#[get("/")]
async fn get_status(session: Session) -> Json<ApiStatus> {
    Json(ApiStatus {
        session: session.clone(),
        user: session.user().await.and_then(|u| Some(u.profile())),
        time: Utc::now()
    })
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LoginModel {
    pub username: String,
    pub password: String
}

#[openapi(tag = "Root")]
#[post("/login", data = "<login>")]
async fn login(session: Session, login: Json<LoginModel>) -> ApiResult<Json<UserProfile>> {
    if let Some(user_id) = session.user_id.clone() {
        return Err(crate::ApiError::method_not_allowed_logged_in(user_id));
    }

    let login = login.into_inner();

    if let Some(user) = User::get_username(login.username.clone()).await? {
        if user.verify(login.password).await {
            session.user_id(user.id()).save().await?;
            Ok(Json(user.profile()))
        } else {
            Err(crate::ApiError::not_found_invalid_login(login.username))
        }
    } else {
        User::create("noop", "noop").verify(login.password).await;
        Err(crate::ApiError::not_found_invalid_login(login.username))
    }
}

#[openapi(tag = "Root")]
#[delete("/login")]
async fn logout(session: Session, _user: User) -> ApiResult<()> {
    let mut session = session;
    session.user_id = None;
    session.save().await?;
    Ok(())
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![get_status, login, logout]
}