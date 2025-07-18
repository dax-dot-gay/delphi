use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use specta::Type;

#[derive(Clone, Debug, Serialize, Deserialize, Type, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Book,
    ResearchPaper,
    Movie,
    Show,
}
