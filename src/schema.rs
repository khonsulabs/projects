use async_trait::async_trait;
use bonsaidb::core::{
    document::{BorrowedDocument, Document},
    schema::{
        Collection, CollectionName, DefaultViewSerialization, Name, Schema, SchemaName, Schematic,
        SerializedCollection, View, ViewMapResult, ViewSchema,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use transmog_json::{serde_json::Value, Json};

#[derive(Debug)]
pub struct Projects;

impl Schema for Projects {
    fn schema_name() -> SchemaName {
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
    fn collection_name() -> CollectionName {
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

#[derive(Debug, Clone)]
pub struct GitHubEventById;

impl View for GitHubEventById {
    type Collection = Event;
    type Key = String;
    type Value = ();

    fn name(&self) -> Name {
        Name::new("by-id")
    }
}

impl ViewSchema for GitHubEventById {
    type View = Self;
    fn map(&self, document: &BorrowedDocument<'_>) -> ViewMapResult<Self> {
        let event = document.contents::<Event>().unwrap();
        Ok(document.emit_key(event.id))
    }
}

impl DefaultViewSerialization for GitHubEventById {}

#[derive(Debug, Clone)]
pub struct GitHubEventByDate;

impl View for GitHubEventByDate {
    type Collection = Event;
    type Key = String;
    type Value = ();

    fn name(&self) -> Name {
        Name::new("by-date")
    }
}

impl ViewSchema for GitHubEventByDate {
    type View = Self;
    fn map(&self, document: &BorrowedDocument<'_>) -> ViewMapResult<Self> {
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
