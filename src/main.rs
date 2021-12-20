use std::env;

use bonsaidb::{
    core::connection::StorageConnection,
    keystorage::s3::{aws_sdk_s3::Endpoint, S3VaultKeyStorage},
    local::{
        config::{Builder, StorageConfiguration},
        Storage,
    },
};
use http::Uri;

use crate::schema::Projects;

mod projects;
mod schema;
mod updater;
mod webserver;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    // initialize tracing
    tracing_subscriber::fmt()
        .pretty()
        // enable everything
        // .with_max_level(tracing::Level::TRACE)
        // .with_span_events(FmtSpan::ENTER)
        // sets this to be the default, global collector for this application.
        .init();

    let mut configuration = StorageConfiguration::new("projects.bonsaidb");
    if let Ok(bucket) = env::var("VAULT_S3_BUCKET") {
        configuration = configuration.vault_key_storage(
            S3VaultKeyStorage::new(bucket)
                .endpoint(Endpoint::immutable(Uri::try_from(env::var(
                    "VAULT_S3_ENDPOINT",
                )?)?))
                .path(env::var("VAULT_S3_PATH")?),
        );
    }

    let storage = Storage::open(configuration).await?;
    storage.register_schema::<Projects>().await?;
    storage
        .create_database::<Projects>("projects", true)
        .await?;
    let database = storage.database::<Projects>("projects").await?;

    let task_database = database.clone();
    let updater = tokio::spawn(async move {
        updater::update_events_periodically(task_database.clone())
            .await
            .unwrap();
    });
    let server = tokio::spawn(async move {
        webserver::serve(database).await.unwrap();
    });

    tokio::try_join!(updater, server)?;
    Ok(())
}
