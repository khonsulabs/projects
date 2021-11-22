use std::env;

use bonsaidb::{
    core::connection::StorageConnection,
    keystorage::s3::{
        s3::{creds::Credentials, Bucket, Region},
        S3VaultKeyStorage,
    },
    local::{config::Configuration, vault::AnyVaultKeyStorage, Storage},
};

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

    let vault_key_storage = if let Ok(bucket) = env::var("VAULT_S3_BUCKET") {
        Some(Box::new(
            S3VaultKeyStorage::from(Bucket::new(
                &bucket,
                Region::Custom {
                    region: String::default(),
                    endpoint: env::var("VAULT_S3_ENDPOINT")?,
                },
                Credentials::new(
                    Some(&env::var("VAULT_S3_KEY_ID")?),
                    Some(&env::var("VAULT_S3_SECRET_KEY")?),
                    None,
                    None,
                    None,
                )?,
            )?)
            .path(env::var("VAULT_S3_PATH")?),
        ) as Box<dyn AnyVaultKeyStorage>)
    } else {
        None
    };
    let storage = Storage::open_local(
        "projects.bonsaidb",
        Configuration {
            vault_key_storage,
            ..Configuration::default()
        },
    )
    .await?;
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
