use okapi::openapi3::OpenApi;
use oximod::set_global_client;
use rocket::Rocket;

pub mod models;
pub mod util;
pub mod routes;
mod error;

pub use error::{Error, ApiError, Result, ApiResult};

pub fn get_spec() -> OpenApi {
    let (_, spec) = routes::mount(Rocket::build());
    spec
}

#[rocket::main]
pub async fn run_api() -> anyhow::Result<()> {
    set_global_client(String::from("mongodb://root:root@db:27017/")).await?;

    let (rocket, _) = routes::mount(Rocket::build());
    let launch_result = rocket.launch().await;

    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    }

    Ok(())
}
