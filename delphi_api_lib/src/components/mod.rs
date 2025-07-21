use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use specta::Type;

pub mod search;

#[derive(Serialize, Deserialize, Clone, Debug, thiserror::Error, Type, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "component")]
pub enum ComponentError {
    #[error(transparent)] Search {
        #[from] error: search::SearchComponentError,
    },
}

impl ComponentError {
    pub fn component_kind(&self) -> String {
        (
            match self {
                ComponentError::Search { .. } => "search",
            }
        ).to_string()
    }
}
