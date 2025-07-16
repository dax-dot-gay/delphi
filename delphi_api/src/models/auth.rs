use chrono::{DateTime, Utc};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, oximod::Model, schemars::JsonSchema)]
#[serde_with::serde_as]
#[db("delphi")]
#[collection("auth.sessions")]
pub struct Session {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(skip)]
    pub _id: Option<bson::oid::ObjectId>,

    #[serde(default = "crate::util::default_uid")]
    pub uid: String,
    
    #[serde(default = "chrono::Utc::now")]
    pub created: DateTime<Utc>
}