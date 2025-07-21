use bon::Builder;
use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use specta::Type;

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

#[derive(Serialize, Deserialize, Clone, Debug, Type, JsonSchema, Builder)]
pub struct SearchResultInfo {
    /// Result ID (UUID)
    #[builder(start_fn, into)]
    pub id: String,

    /// Source search component ID
    #[builder(start_fn, into)]
    pub source: String,

    /// Result kind: [SearchKind]
    #[builder(start_fn)]
    pub kind: SearchKind,

    /// Title string
    #[builder(start_fn, into)]
    pub title: String,

    /// Subtitle string (ie author, director, etc)
    #[builder(into)]
    pub subtitle: Option<String>,

    /// URL to a thumbnail image
    #[builder(into)]
    pub thumbnail: Option<String>,

    /// Publish/creation date
    #[builder(into)]
    pub date: Option<NaiveDate>,

    /// About text/blurb
    #[builder(into)]
    pub about: Option<String>,

    /// Fractional rating 0-1
    #[builder(setters(vis = "", name = rating_fraction_internal))]
    pub rating_fraction: Option<f32>,
}

use search_result_info_builder::{ State, IsUnset, SetRatingFraction };
impl<S: State> SearchResultInfoBuilder<S> {
    pub fn rating_fraction(
        self,
        value: impl Into<f32>
    ) -> crate::Result<SearchResultInfoBuilder<SetRatingFraction<S>>>
        where S::RatingFraction: IsUnset
    {
        let value = value.into();
        if value <= 1.0 && value >= 0.0 {
            Ok(self.rating_fraction_internal(value))
        } else {
            Err(
                (SearchComponentError::InvalidRating {
                    kind: "fraction".into(),
                    value: value,
                }).into()
            )
        }
    }

    pub fn rating_stars(
        self,
        value: impl Into<f32>
    ) -> crate::Result<SearchResultInfoBuilder<SetRatingFraction<S>>>
        where S::RatingFraction: IsUnset
    {
        let value = value.into();
        if value <= 5.0 && value >= 0.0 {
            Ok(self.rating_fraction_internal(value / 5.0))
        } else {
            Err(
                (SearchComponentError::InvalidRating {
                    kind: "stars".into(),
                    value: value,
                }).into()
            )
        }
    }

    pub fn rating_percent(
        self,
        value: impl Into<f32>
    ) -> crate::Result<SearchResultInfoBuilder<SetRatingFraction<S>>>
        where S::RatingFraction: IsUnset
    {
        let value = value.into();
        if value <= 100.0 && value >= 0.0 {
            Ok(self.rating_fraction_internal(value / 100.0))
        } else {
            Err(
                (SearchComponentError::InvalidRating {
                    kind: "percent".into(),
                    value: value,
                }).into()
            )
        }
    }
}

pub trait SearchResult {
    fn component_id(&self) -> String;
    fn result_kind(&self) -> SearchKind;
    fn result_info(&self) -> crate::Result<SearchResultInfo>;
    fn result_id(&self) -> String;
}
