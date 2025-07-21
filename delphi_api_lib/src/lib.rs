use okapi::openapi3::OpenApi;
use oximod::set_global_client;
use rocket::{ fairing::AdHoc, Rocket };

pub mod models;
pub mod util;
pub mod routes;
pub mod config;
pub mod components;
mod error;

pub use error::{ Error, ApiError, Result, ApiResult };

use crate::{ config::Config, models::User };

pub fn get_spec() -> OpenApi {
    let (_, spec) = routes::mount(Rocket::build());
    spec
}

#[rocket::main]
pub async fn run_api() -> anyhow::Result<()> {
    let (rocket, _) = routes::mount(Rocket::build());
    let launch_result = rocket
        .attach(
            AdHoc::on_ignite("Initial Setup", |rocket|
                Box::pin(async move {
                    let config = rocket
                        .figment()
                        .extract_inner("delphi")
                        .unwrap_or(Config::default());
                    set_global_client(config.database.uri()).await.expect(
                        "Failed to connect to database!"
                    );

                    if let Some((admin_username, admin_password)) = config.auth.admin_user.clone() {
                        if let Ok(Some(user)) = User::get_username(admin_username.clone()).await {
                            if !user.is_admin {
                                panic!("Desired admin user already exists and is not an admin!");
                            }
                        } else {
                            User::create_admin(admin_username, admin_password)
                                .save().await
                                .expect("Failed to save new admin");
                        }
                    }

                    rocket.manage(config)
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
