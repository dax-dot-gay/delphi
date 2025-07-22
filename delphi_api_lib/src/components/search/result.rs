use bon::Builder;
use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use specta::Type;

use crate::components::search::{ SearchComponentError, SearchKind };

#[derive(Serialize, Deserialize, Clone, Debug, Type, JsonSchema, Builder)]
pub struct SearchResult {
    /// Result ID
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

    /// Fractional rating 0-1
    #[builder(setters(vis = "", name = rating_fraction_internal))]
    pub rating_fraction: Option<f32>,
}

use search_result_builder::{ State, IsUnset, SetRatingFraction };
impl<S: State> SearchResultBuilder<S> {
    pub fn rating_fraction(
        self,
        value: impl Into<f32>
    ) -> crate::Result<SearchResultBuilder<SetRatingFraction<S>>>
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
    ) -> crate::Result<SearchResultBuilder<SetRatingFraction<S>>>
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
    ) -> crate::Result<SearchResultBuilder<SetRatingFraction<S>>>
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
