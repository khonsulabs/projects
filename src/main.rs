use bonsaidb::{
    core::connection::ServerConnection,
    local::{config::Configuration, Storage},
};

use crate::schema::Projects;

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
    let storage = Storage::open_local("projects.bonsaidb", Configuration::default()).await?;
    storage.register_schema::<Projects>().await?;
    storage
        .create_database::<Projects>("projects", true)
        .await?;
    let database = storage.database("projects").await?;

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
