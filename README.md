# khonsulabs-projects

[![Live Deploy Status](https://img.shields.io/github/workflow/status/khonsulabs/projects/Deploy/main)](https://github.com/khonsulabs/projects/actions?query=workflow:Deploy)

This crate is what serves [KhonsuLabs.com](https://khonsulabs.com/).

The primary goal of the site is to add context to our development efforts for the casual person looking at one of our crates. It can be hard to understand how an individual crate fits into our plans, and if a crate hasn't been touched in a little while, it might be reassuring to see how it fits into our plans.

## Powered by BonsaiDb

This project uses our own database, [BonsaiDb](https://github.com/khonsulabs/bonsaidb), to store events fetched from GitHub. While BonsaiDb does have a server mode, we are using it in local-only mode -- similar to how SQLite is utilized.

The dream of BonsaiDb is that it can "grow with you," and this project will be a good example of that. We plan to migrate from a local-only database to a standalone server to a highly-available cluster as BonsaiDb grows and we add more functionality to the app.

## Application Overview

There are two main components to this application, the background [updater](./src/updater.rs) and the [webserver](./src/webserver.rs).

### Background Updater

The background updater is an async infinite loop that executes `fetch_new_events()` every five minutes.

`fetch_new_events()` requests events for the KhonsuLabs organization on GitHub and looks for any events that aren't contained in the database. If any existing events are found, no additional pages of data are requested from GitHub.

All new events are then inserted into the database as `GitHubEvent`s.

### Webserver

The webserver uses [Axum](https://github.com/tokio-rs/axum). It contains one dynamic endpoint to handle the `/` request, and it also serves files from the [static/](./static) folder.

The index handler queries recent events via the `GitHubEventByDate` view, and renders the page content using [Tera](https://github.com/Keats/tera).

## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), are open-source. This repository is available under the [MIT License](./LICENSE-MIT) or the [Apache License 2.0](./LICENSE-APACHE).
