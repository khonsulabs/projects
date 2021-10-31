use std::{collections::HashMap, convert::Infallible, net::SocketAddr, sync::Arc};

use axum::{extract, handler::get, response::Html, service, AddExtensionLayer, Router};
use bonsaidb::{core::connection::Connection, local::Database};
use chrono::{Duration, Utc};
use http::StatusCode;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tower_http::services::ServeDir;

use crate::schema::{
    Event, GithubEventByDate, IssuesPayload, Projects, PushPayload, Release, ReleasePayload,
};

const CONTRIBUTOR_EMAILS: [&str; 2] = ["jon@khonsulabs.com", "daxpedda@gmail.com"];
const FORKED_REPOSITORIES: [&str; 7] = [
    "iqlusioninc/crates",
    "novifinancial/opaque-ke",
    "dalek-cryptography/curve25519-dalek",
    "RustCrypto/password-hashes",
    "novifinancial/voprf",
    "ModProg/derive-where",
    "ModProg/derive-restricted",
];

static PROJECTS: Lazy<HashMap<String, Project>> = Lazy::new(|| {
    [Project {
        name: "BonsaiDb",
        tagline: "A document database that grows with you.",
        description: r#"
            We evaluated the landscape of pure-Rust database implementations, and none fit our goals for an eventual architecture that scaled the way we wanted. Additionally, the non-Rust standards are difficult to deploy in a highly-available fashion.
        "#,
        homepage: Some("https://bonsaidb.io/"),
        repository: "https://github.com/khonsulabs/bonsaidb",
        documentation: "https://dev.bonsaidb.io/main/bonsaidb",
    },
    Project {
        name: "Nebari",
        tagline: "ACID-compliant key-value database implementation using an append-only file format.",
        description: r#"
            <p>While we started BonsaiDb atop another storage layer, we decided to pursue an in-house implementation that we could tailor-fit to the needs of BonsaiDb.

            <p>Nebari aims to provide speed, safety, and reliability while still remaining easy to understand and approachable to new contributors.
        "#,
        homepage: None,
        repository: "https://github.com/khonsulabs/nebari",
        documentation: "https://nebari.bonsaidb.io/main/nebari",
    },
    Project {
        name: "Kludgine",
        tagline: "2D graphics and windowing built atop wgpu",
        description: r#"
            <p>Deep down our passion is still with games, even though we may be focusing a lot of general-purpose application development at the moment. Kludgine was born after evaluating other libraries at the time and deciding there was still room for improvement.

            <p>Kludgine is the base layer for Gooey, our Graphical User Interface crate.
        "#,
        homepage: None,
        repository: "https://github.com/khonsulabs/nebari",
        documentation: "https://nebari.bonsaidb.io/main/nebari",
    },
    ]
    .into_iter()
    .map(|project| (project.name.to_ascii_lowercase(), project))
    .collect()
});

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    pub name: &'static str,
    pub tagline: &'static str,
    pub description: &'static str,
    pub homepage: Option<&'static str>,
    pub repository: &'static str,
    pub documentation: &'static str,
}

pub async fn serve(database: Database<Projects>) -> anyhow::Result<()> {
    let templates = Tera::new("templates/**/*")?;

    let templates = Arc::new(templates);

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .or(
            service::get(ServeDir::new("./static")).handle_error(|error: std::io::Error| {
                Ok::<_, Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }),
        )
        .layer(AddExtensionLayer::new(templates))
        .layer(AddExtensionLayer::new(database));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn handler(
    templates: extract::Extension<Arc<Tera>>,
    database: extract::Extension<Database<Projects>>,
) -> Result<Html<String>, (StatusCode, String)> {
    // While debugging, reload the templates always.
    #[cfg(debug_assertions)]
    let templates = {
        drop(templates);
        Tera::new("templates/**/*")
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    };

    let now = Utc::now();
    let tomorrow = now + Duration::days(1);
    let one_month_ago = now - Duration::weeks(4);
    let events = database
        .view::<GithubEventByDate>()
        .with_key_range(
            one_month_ago.format("%Y-%m-%d").to_string()..tomorrow.format("%Y-%m-%d").to_string(),
        )
        .query_with_docs()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let mut days = Vec::new();
    let mut current_day = None;
    for event in events {
        let github_event = event
            .document
            .contents::<Event>()
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        // Ignore all events from github actions
        if github_event.actor.login == "github-actions[bot]" {
            continue;
        }

        let local_repository_name = github_event.repository.name.split('/').nth(1).unwrap();

        let forked_repo = FORKED_REPOSITORIES
            .into_iter()
            .find(|repo| repo.split('/').nth(1).unwrap() == local_repository_name);

        if current_day.as_ref() != Some(&event.key) {
            current_day = Some(event.key.clone());
            days.push(DayEvents {
                display: github_event.created_at.format("%A, %B %e, %Y").to_string(),
                repositories: HashMap::new(),
                iso_date: event.key.clone(),
            });
        }

        let day_events = days.last_mut().unwrap();
        let repository = day_events
            .repositories
            .entry(local_repository_name.to_string())
            .or_insert_with(|| ActiveRepository {
                url: format!(
                    "https://github.com/{}",
                    forked_repo.unwrap_or(&github_event.repository.name)
                ),
                forked_from: forked_repo.map(|r| r.to_string()),
                ..ActiveRepository::default()
            });

        match github_event.kind.as_str() {
            "IssuesEvent" => {
                let payload =
                    serde_json::value::from_value::<IssuesPayload>(github_event.payload.clone())
                        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
                if payload.action == "closed" {
                    continue;
                }

                repository.issues_closed.push(ClosedIssue {
                    id: payload.issue.number,
                    author: github_event.actor.login.clone(),
                    url: payload.issue.html_url.to_string(),
                    title: payload.issue.title.clone(),
                });
            }
            "PushEvent" => {
                let push =
                    serde_json::value::from_value::<PushPayload>(github_event.payload.clone())
                        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
                for commit in &push.commits {
                    if forked_repo.is_none()
                        || CONTRIBUTOR_EMAILS.contains(&commit.author.email.as_str())
                    {
                        let repository = repository
                            .commit_authors
                            .entry(github_event.actor.login.clone())
                            .or_default();
                        repository
                            .entry(push.reference.split('/').last().unwrap().to_string())
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
            "ReleaseEvent" => {
                let event =
                    serde_json::value::from_value::<ReleasePayload>(github_event.payload.clone())
                        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
                if event.release.draft {
                    continue;
                }

                repository.releases.push(event.release);
            }

            _ => continue,
        }
    }

    days.reverse();
    for day in &mut days {
        day.repositories.retain(|_key, value| {
            !value.issues_closed.is_empty() || !value.commit_authors.is_empty()
        });
    }
    days.retain(|d| !d.repositories.is_empty());

    let mut context = Context::new();
    context.insert("days", &days);
    context.insert("projects", &*PROJECTS);
    Ok(Html(templates.render("index.html", &context).map_err(
        |err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    )?))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayEvents {
    pub display: String,
    pub iso_date: String,
    pub repositories: HashMap<String, ActiveRepository>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ActiveRepository {
    pub url: String,
    pub forked_from: Option<String>,
    pub commit_authors: HashMap<String, HashMap<String, usize>>,
    pub issues_closed: Vec<ClosedIssue>,
    pub releases: Vec<Release>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClosedIssue {
    pub id: u64,
    pub author: String,
    pub url: String,
    pub title: String,
}
