use std::time::Duration;

use bonsaidb::{
    core::{
        connection::{AccessPolicy, Connection, QueryKey},
        schema::Collection,
    },
    local::Database,
};
use reqwest::{
    header::{ACCEPT, USER_AGENT},
    Client,
};

use crate::schema::{Event, GitHubEventById, Projects};

pub async fn update_events_periodically(storage: Database<Projects>) -> anyhow::Result<()> {
    let instance = Client::new();
    loop {
        tracing::info!("Fetching new events from GitHub");
        fetch_new_events(&storage, &instance).await?;
        tracing::info!("Sleeping");
        tokio::time::sleep(Duration::from_secs(300)).await;
    }
}

async fn fetch_new_events(database: &Database<Projects>, client: &Client) -> anyhow::Result<()> {
    let mut events_to_process = Vec::new();

    // Loop and gather all the vents we need to insert, potentially across multiple pages.
    'page: for page in 1_u32.. {
        tracing::info!("Requesting page {} from github", page);
        let response = client
            .get(format!(
                "https://api.github.com/orgs/khonsulabs/events?page={}&perpage=100",
                page
            ))
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT, "khonsulabs-projects-daemon")
            .basic_auth("ecton", Some(std::env::var("GITHUB_TOKEN").unwrap()))
            .send()
            .await?;
        let text = response.text().await?;
        let events: Vec<Event> = match serde_json::from_str(&text) {
            Ok(events) => events,
            Err(_) => break 'page,
        };
        for event in events.into_iter().filter(|evt| {
            matches!(
                evt.kind.as_str(),
                "PushEvent"
                    | "IssuesEvent"
                    | "PullRequestEvent"
                    | "ReleaseEvent"
                    | "SponshorshipEvent"
            )
        }) {
            if database
                .query::<GitHubEventById>(
                    Some(QueryKey::Matches(event.id.clone())),
                    AccessPolicy::UpdateBefore,
                )
                .await
                .unwrap()
                .is_empty()
            {
                events_to_process.push(event);
            } else {
                break 'page;
            }
        }
    }

    tracing::info!("Received {} events", events_to_process.len());
    for event in events_to_process {
        tracing::debug!("Inserting event {:?}", event);
        event.insert_into(database).await?;
    }

    Ok(())
}
