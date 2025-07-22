use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use specta::Type;
pub mod result;
pub mod info;

pub use result::SearchResult;
pub use info::{AssociatedEntity, AssociatedOrganization, AssociatedPerson, CommonSearchInfo, SearchInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Type, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchKind {
    Book,
    Article,
    Movie,
    Show,
    Anime,
}

#[derive(Serialize, Deserialize, Clone, Debug, thiserror::Error, Type, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "error_kind")]
pub enum SearchComponentError {
    #[error("Invalid rating for {kind}: {value:?}")] InvalidRating {
        kind: String,
        value: f32,
    },
}
