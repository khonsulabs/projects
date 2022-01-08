use async_trait::async_trait;
use bonsaidb::core::schema::{
    Collection, CollectionName, DefaultViewSerialization, InvalidNameError, Name, Schema,
    SchemaName, Schematic, SerializedCollection, View,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use transmog_json::{serde_json::Value, Json};

#[derive(Debug)]
pub struct Projects;

impl Schema for Projects {
    fn schema_name() -> Result<SchemaName, InvalidNameError> {
        SchemaName::new("khonsulabs", "projects")
    }

    fn define_collections(schema: &mut Schematic) -> Result<(), bonsaidb::core::Error> {
        schema.define_collection::<Event>()?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub actor: User,
    #[serde(rename = "repo")]
    pub repository: Repository,
    pub payload: Value,
    pub public: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[async_trait]
impl Collection for Event {
    fn collection_name() -> Result<CollectionName, InvalidNameError> {
        CollectionName::new("khonsulabs", "github-events")
    }

    fn define_views(schema: &mut Schematic) -> Result<(), bonsaidb::core::Error> {
        schema.define_view(GitHubEventById)?;
        schema.define_view(GitHubEventByDate)?;
        Ok(())
    }
}

impl SerializedCollection for Event {
    type Contents = Self;
    type Format = Json;

    fn format() -> Self::Format {
        Json::default()
    }
}

#[derive(Debug)]
pub struct GitHubEventById;

impl View for GitHubEventById {
    type Collection = Event;
    type Key = String;
    type Value = ();

    fn version(&self) -> u64 {
        0
    }

    fn name(&self) -> Result<Name, InvalidNameError> {
        Name::new("by-id")
    }

    fn map(
        &self,
        document: &bonsaidb::core::document::Document,
    ) -> bonsaidb::core::schema::MapResult<Self::Key, Self::Value> {
        let event = document.contents::<Event>().unwrap();
        Ok(document.emit_key(event.id))
    }
}

impl DefaultViewSerialization for GitHubEventById {}

#[derive(Debug)]
pub struct GitHubEventByDate;

impl View for GitHubEventByDate {
    type Collection = Event;
    type Key = String;
    type Value = ();

    fn version(&self) -> u64 {
        0
    }

    fn name(&self) -> Result<Name, InvalidNameError> {
        Name::new("by-date")
    }

    fn map(
        &self,
        document: &bonsaidb::core::document::Document,
    ) -> bonsaidb::core::schema::MapResult<Self::Key, Self::Value> {
        let event = document.contents::<Event>().unwrap();
        Ok(document.emit_key(event.created_at.format("%Y-%m-%d").to_string()))
    }
}

impl DefaultViewSerialization for GitHubEventByDate {}

#[derive(Deserialize, Serialize, Debug)]
pub struct PushPayload {
    #[serde(rename = "ref")]
    pub reference: String,
    pub head: String,
    pub before: String,
    pub commits: Vec<Commit>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IssuesPayload {
    pub action: String,
    pub issue: Issue,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Issue {
    pub title: String,
    pub id: u64,
    pub html_url: String,
    pub number: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReleasePayload {
    pub action: String,
    pub release: Release,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Release {
    pub id: u64,
    pub name: String,
    pub html_url: String,
    pub author: User,
    pub draft: bool,
    pub prerelease: bool,
    pub short_description_html: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Commit {
    pub sha: String,
    pub message: String,
    pub author: Author,
    pub url: String,
    pub distinct: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
}
