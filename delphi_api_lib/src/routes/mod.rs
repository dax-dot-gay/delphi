use okapi::{merge::marge_spec_list, openapi3::OpenApi};
use rocket::{Rocket, Build};
use rocket_okapi::handlers::OpenApiHandler;

mod root;

macro_rules! mount {
    ($path:expr, $module:ident, $rocket:ident, $specs:ident) => {
        let (routes, spec) = $module::routes();
        let $rocket = $rocket.mount($path, routes);
        $specs.push(($path.to_string(), spec));
    };
}

pub fn mount(rocket: Rocket<Build>) -> (Rocket<Build>, OpenApi) {
    let mut specs: Vec<(String, OpenApi)> = Vec::new();

    mount!("/", root, rocket, specs);

    let spec = marge_spec_list(&specs).unwrap();
    (rocket.mount("/", vec![OpenApiHandler::new(spec.clone()).into_route("/openapi.json")]), spec)
}