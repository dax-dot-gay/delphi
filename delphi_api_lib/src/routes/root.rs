use chrono::Utc;
use okapi::openapi3::OpenApi;
use rocket::{get, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes_spec};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::Session;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ApiStatus {
    pub session: Session,
    pub time: chrono::DateTime<Utc>
}

#[openapi(tag = "Root")]
#[get("/status")]
async fn get_status(session: Session) -> Json<ApiStatus> {
    Json(ApiStatus {
        session: session,
        time: Utc::now()
    })
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![get_status]
}