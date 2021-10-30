use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use bonsaidb::core::schema::{
    Collection, CollectionName, CollectionSerializer, InvalidNameError, Name, Schema, SchemaName,
    Schematic, View,
};
use octocrab::models::events::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Projects;

impl Schema for Projects {
    fn schema_name() -> Result<SchemaName, InvalidNameError> {
        SchemaName::new("khonsulabs", "projects")
    }

    fn define_collections(schema: &mut Schematic) -> Result<(), bonsaidb::core::Error> {
        schema.define_collection::<GithubEvent>()?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubEvent(pub Event);

impl From<Event> for GithubEvent {
    fn from(evt: Event) -> Self {
        Self(evt)
    }
}

impl Deref for GithubEvent {
    type Target = Event;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GithubEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl Collection for GithubEvent {
    fn collection_name() -> Result<CollectionName, InvalidNameError> {
        CollectionName::new("khonsulabs", "github-events")
    }

    fn define_views(schema: &mut Schematic) -> Result<(), bonsaidb::core::Error> {
        schema.define_view(GithubEventById)?;
        schema.define_view(GithubEventByDate)?;
        Ok(())
    }

    fn serializer() -> CollectionSerializer {
        CollectionSerializer::Json
    }
}

#[derive(Debug)]
pub struct GithubEventById;

impl View for GithubEventById {
    type Collection = GithubEvent;
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
        document: &bonsaidb::core::document::Document<'_>,
    ) -> bonsaidb::core::schema::MapResult<Self::Key, Self::Value> {
        let event = document.contents::<GithubEvent>().unwrap();
        Ok(vec![document.emit_key(event.0.id)])
    }
}

#[derive(Debug)]
pub struct GithubEventByDate;

impl View for GithubEventByDate {
    type Collection = GithubEvent;
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
        document: &bonsaidb::core::document::Document<'_>,
    ) -> bonsaidb::core::schema::MapResult<Self::Key, Self::Value> {
        let event = document.contents::<GithubEvent>().unwrap();
        Ok(vec![document.emit_key(
            event.0.created_at.format("%Y-%m-%d").to_string(),
        )])
    }
}
