use std::{fs, path::Path};

use delphi_api_lib::get_spec;

fn main() {
    let spec = get_spec();
    let out_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("delphi_client").join("openapi.json");
    fs::write(out_path, serde_json::to_string_pretty(&spec).unwrap()).unwrap();
}