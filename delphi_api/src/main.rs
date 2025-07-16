use bson::oid::ObjectId;
use chrono::Utc;
use oximod::{set_global_client, ModelTrait};
use rocket::{ get, serde::json::Json };
use rocket_okapi::{ openapi, openapi_get_routes, swagger_ui::{make_swagger_ui, SwaggerUIConfig} };
use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use anyhow::Result;

use crate::models::Session;

pub mod models;
pub mod util;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ApiStatus {
    pub session: Session,
    pub time: chrono::DateTime<Utc>
}

#[openapi(tag = "Root")]
#[get("/status")]
async fn get_status() -> Json<ApiStatus> {
    let session = Session::new().id(ObjectId::new()).created(Utc::now());
    session.save().await.expect("Failed to save!");
    Json(ApiStatus {
        session: session,
        time: Utc::now()
    })
}

#[rocket::main]
async fn main() -> Result<()> {
    set_global_client(String::from("mongodb://root:root@db:27017/")).await?;

    let launch_result = rocket
        ::build()
        .mount("/", openapi_get_routes![get_status])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(
                &(SwaggerUIConfig {
                    url: "../openapi.json".to_owned(),
                    ..Default::default()
                })
            )
        )
        .launch().await;

    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    }

    Ok(())
}
