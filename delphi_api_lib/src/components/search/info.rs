use std::fmt::Debug;

use bon::Builder;
use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use specta::Type;

use crate::components::search::{ result::SearchResult, SearchComponentError, SearchKind };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Type, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "entity_class")]
pub enum AssociatedPerson {
    Author,
    Editor,
    Actor,
    Director,
    Producer,
    Writer,
    Artist,
    Other {
        description: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Type, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "entity_class")]
pub enum AssociatedOrganization {
    Publisher,
    Studio,
    Sponsor,
    Other {
        description: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AssociatedEntity {
    Tag {
        name: String,
    },
    Character {
        name: String,
    },
    Person {
        name: String,
        #[serde(flatten)]
        entity_class: AssociatedPerson,
    },
    Organization {
        name: String,
        #[serde(flatten)]
        entity_class: AssociatedOrganization,
    },
    Location {
        name: String,
        real: bool,
    },
    Language {
        name: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type, JsonSchema, Builder)]
pub struct CommonSearchInfo {
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

    /// Entities associated with this item (see [AssociatedEntity])
    #[builder(field)]
    pub associated_entities: Vec<AssociatedEntity>,

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

    /// About text
    #[builder(into)]
    pub description: Option<String>,

    /// Image banner
    #[builder(into)]
    pub banner: Option<String>,
}

impl CommonSearchInfo {
    pub fn from_result(
        result: SearchResult
    ) -> CommonSearchInfoBuilder<SetRatingFraction<SetDate<SetThumbnail<SetSubtitle>>>> {
        result.into()
    }
}

use common_search_info_builder::*;

impl From<SearchResult>
for CommonSearchInfoBuilder<SetRatingFraction<SetDate<SetThumbnail<SetSubtitle>>>> {
    fn from(SearchResult {
        id,
        source,
        kind,
        title,
        subtitle,
        thumbnail,
        date,
        rating_fraction,
    }: SearchResult) -> Self {
        CommonSearchInfo::builder(id, source, kind, title)
            .maybe_subtitle(subtitle)
            .maybe_thumbnail(thumbnail)
            .maybe_date(date)
            .maybe_rating_fraction_internal(rating_fraction)
    }
}

impl<S: State> CommonSearchInfoBuilder<S> {
    pub fn rating_fraction(
        self,
        value: impl Into<f32>
    ) -> crate::Result<CommonSearchInfoBuilder<SetRatingFraction<S>>>
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
    ) -> crate::Result<CommonSearchInfoBuilder<SetRatingFraction<S>>>
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
    ) -> crate::Result<CommonSearchInfoBuilder<SetRatingFraction<S>>>
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

    pub fn with_entity(mut self, entity: AssociatedEntity) -> Self {
        self.associated_entities.push(entity);
        self
    }

    pub fn with_entities(mut self, entities: impl IntoIterator<Item = AssociatedEntity>) -> Self {
        self.associated_entities.extend(entities);
        self
    }
}

pub trait SearchInfo {
    fn common(&self) -> CommonSearchInfo;
    fn id(&self) -> String {
        self.common().id
    }
    fn source(&self) -> String {
        self.common().source
    }
    fn kind(&self) -> SearchKind {
        self.common().kind
    }
}
