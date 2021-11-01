use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: &'static str,
    pub tagline: &'static str,
    pub description: &'static str,
    pub homepage: Option<&'static str>,
    pub repository: &'static str,
    pub documentation: Option<&'static str>,
}

const ACTIONABLE: Project = Project {
    name: "actionable",
    tagline: "An enum-based async framework for building permission-driven APIs",
    description: r#"<p>Actionable defines a basic Role Based Access Control permissions framework as well as a set of traits that allow "dispatching" an enum. Additionally, it offers procedural macros to remove some common sources of code duplication."#,
    homepage: None,
    repository: "https://github.com/khonsulabs/actionable",
    documentation: Some("https://khonsulabs.github.io/actionable/main/actionable/"),
};

const BONSAIDB: Project = Project {
    name: "BonsaiDb",
    tagline: "A document database that grows with you.",
    description: r#"
        We evaluated the landscape of pure-Rust database implementations, and none fit our goals for an eventual architecture that scaled the way we wanted. Additionally, the non-Rust standards are difficult to deploy in a highly-available fashion.
    "#,
    homepage: Some("https://bonsaidb.io/"),
    repository: "https://github.com/khonsulabs/bonsaidb",
    documentation: Some("https://dev.bonsaidb.io/main/bonsaidb"),
};

const CUSTODIAN: Project = Project {
    name: "custodian",
    tagline: "End-user secret management with Rust. ",
    description: r#"
        <p>Custodian aims to be a general purpose set of secret management APIs aimed at helping developers store secrets easily, yet securely.
        <p>Currently, the only crate published is custodian-password, an easy-to-use OPAQUE-KE wrapper that BonsaiDb uses when setting a password for a user. This key exchange protocol ensures that the password never leaves the client, and the server can still verify upon a new connection that the original password was provided.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/custodian",
    documentation: Some("https://khonsulabs.github.io/custodian/main/custodian_password/"),
};

const EASYGPU: Project = Project {
    name: "easygpu",
    tagline: "wgpu made a little easier",
    description: r#"
        <p>Kludgine transitioned between multiple rendering backends before utlimately easygpu was developed as an offshoot 
        from a large refactoring of <a href="https://github.com/cloudhead/rgx>rgx</a> aimed at bringing compatibility to wgpu 0.6 at the time.
        <p>easygpu is not aiming to be a major entry in the ecosystem of Rust graphics. It provides a semi-stable base layer that Kludgine builds upon that is slightly easier to consume than wgpu directly. Additionally, this repository contains easygpu-lyon, which provides 2d tesselated shape/path drawing in a resuable pipeline, which Kludgine uses to provide its shape rendering.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/easygpu",
    documentation: None,
};

const ENGLISHID: Project = Project {
    name: "englishid",
    tagline: "Encode and decode data using plain English.",
    description: r#"<p>For Ncog, we had two problems that utilized random data that sometimes might need to be read aloud: invite codes and backup keys. This crate encodes data using an English wordlist with 13 bits of information being represented by each word. For Ncog, this means that invite codes can be 4 words long, and Ed25519 backup keys can be 20 words long."#,
    homepage: None,
    repository: "https://github.com/khonsulabs/englishid",
    documentation: Some("https://khonsulabs.github.io/englishid/main/englishid/"),
};

const FABRUIC: Project = Project {
    name: "fabruic",
    tagline: "An easy-to-use QUIC-based protocol that supports reliable, ordered payload delivery.",
    description: r#"<p>We needed a reliable protocol for BonsaiDb, and QUIC is a great general-purpose networking protocol that solves many issues that TCP connections suffer from. Fabruic will eventaully grow from only supporting QUIC to being transparently multi-protocol, which will enable WebRTC communications with a web browser. This will replace WebSockets as the best path for WASM BonsaiDb clients. Additionally, Fabruic will add support for unordered, unreliable datagram delivery in addition to the current ordered, reliable streams."#,
    homepage: None,
    repository: "https://github.com/khonsulabs/fabruic",
    documentation: Some("https://khonsulabs.github.io/fabruic/main/fabruic/"),
};

const FIGURES: Project = Project {
    name: "figures",
    tagline: "A math library specialized for 2d screen graphics. ",
    description: r#"
        <p>Rust has a vibrant ecosystem for 2d math APIs. Gooey, Kludgine, and easygpu were all using Euclid, but some of the opinionated decisions for their API did not agree with our desires in the types exposed through Gooey.
        <p>Figures takes a similar approach to Euclid by associating a "unit" type with the primitive numerical type. Additionally, it adds the concept of DisplayScale, allowing for logic to be built that can convert between three units of measurement: raw pixels, dpi-scaled pixels, and user-scaled pixels. Gooey and Kludgine use this to enable automatically scaling based on the screen's DPI settings, but also allowing an additonal scaling factor to be set on-top of the DPI-corrected scaling.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/figures",
    documentation: Some("https://khonsulabs.github.io/figures/main/figures/"),
};

const GOOEY: Project = Project {
    name: "gooey",
    tagline: "An experimental cross-platform graphical user interface (GUI).",
    description: r#"
        <p>We believe in having native applications, but we also believe that having your app or game be accessible inside of a web browser makes it much more approachable for a potential new user.
        <p>The GUI ecosystem in Rust is rapidly evolving, but we had our own opinions on how to best architect a GUI framework in a Rust-y fashion. Gooey is our attempt at that: a cross-platform API that can run natively inside of any wgpu application or inside of a web-browser, being translated to native DOM elements in the process.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/gooey",
    documentation: Some("https://gooey.rs/main/gooey/"),
};

const GOOEY_CANVAS: Project = Project {
    name: "gooey-canvas",
    tagline: "A Canvas widget for the `Gooey` UI framework ",
    description: r#"<p>The Canvas widget adds the ability to create cross-platform 2d drawing code using the Renderer trait -- the same trait that Gooey uses to rasterize its widgets on the native frontend."#,
    homepage: None,
    repository: "https://github.com/khonsulabs/gooey-canvas",
    documentation: None,
};

const KLUDGINE: Project = Project {
    name: "Kludgine",
    tagline: "2D graphics and windowing built atop wgpu",
    description: r#"
        <p>Deep down our passion is still with games, even though we may be focusing a lot of general-purpose application development at the moment. Kludgine was born after evaluating other libraries at the time and deciding there was still room for improvement.

        <p>Kludgine is the base layer for Gooey, our Graphical User Interface crate.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/nebari",
    documentation: Some("https://nebari.bonsaidb.io/main/nebari"),
};

const NCOG: Project = Project {
    name: "Ncog",
    tagline:
        "A self-hostable collaboration platform built with privacy and data independence in mind.",
    description: r#"
        <p>Our original goal with Khonsu Labs is to build an MMORPG, but at the core of our desires of what an MMORPG should contain, we believed reliable, persistent private and group messaging was important. We also went as far as to believe that if we were going to build or own chat layer instead of integrate with another solution, we should only do so if we felt like we could bring something unique to the table. We believe we can.
        <p>Most solutions to this problem are very difficult to deploy and maintain. Our aim is to design a suite of communication and collaboration services that come with sensible, secure default settings and are easy to customize, deploy, and eventually scale.
        <p>Ncog will be the first large-scale project that unifies Gooey and BonsaiDb.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/ncog",
    documentation: None,
};

const NEBARI: Project = Project {
    name: "Nebari",
    tagline: "ACID-compliant key-value database implementation using an append-only file format.",
    description: r#"
        <p>While we started BonsaiDb atop another storage layer, we decided to pursue an in-house implementation that we could tailor-fit to the needs of BonsaiDb.

        <p>Nebari aims to provide speed, safety, and reliability while still remaining easy to understand and approachable to new contributors.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/nebari",
    documentation: Some("https://nebari.bonsaidb.io/main/nebari"),
};

const POT: Project = Project {
    name: "pot",
    tagline: "An experimental self-describing binary format written in Rust for Serde",
    description: r#"
        <p>BonsaiDb originally used CBOR to encode documents before we wrote our own format. Pot is very similar to CBOR, but it aims to improve in one way: not repeating "identifiers" multiple times. When encoding an array of structures with CBOR that has a propery named "disconnected", the word "disconnected" will show up one for each entry in the array. With Pot, the word disconnected will show up only once. 
        
        <p>This space savings comes at a slight cost, but may yield less overall packets of data being sent. Additionally, Pot has session-based capabilities that further reduce the amount of data needed for network communications.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/pot",
    documentation: Some("https://pot.bonsaidb.io/main/pot/"),
};

const PROJECTS_PROJECT: Project = Project {
    name: "projects",
    tagline: "The website you're accessing.",
    description: r#"
        <p>The website you are browsing is served via the Axum framework. The Github history is being retrieved from a local BonsaiDb database, which is updated in the background periodically. The goal of this website is to try to help tie together all of the work that we are doing into one location. Because we are only a few developers with such a large set of repositories, it will likely be that some crates go periods of time without updates.
        <p>This website should help any consumers of Khonsu Labs' crates to see that each crate plays an important role in our ecosystem. As long as we're working towards our big-picture goals, each one of our crates is crucial to the success of our goals.
        <p>A fun note: This project will likely be BonsaiDb's longest-running production application.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/projects",
    documentation: None,
};

const STYLECS: Project = Project {
    name: "stylecs",
    tagline: "A style component system for Rust",
    description: r#"
        <p>This small crate provides a basic set of abstractions for building "style" information. It is what Gooey uses to implement its styling and theming.
        <p>This crate is general-purpose enough it likely is not useful for many people outside of the context of Gooey or building your own style-based data structures.
    "#,
    homepage: None,
    repository: "https://github.com/khonsulabs/stylecs",
    documentation: Some("https://khonsulabs.github.io/stylecs/main/stylecs/"),
};

pub static PROJECTS: Lazy<HashMap<String, Project>> = Lazy::new(|| {
    [
        ACTIONABLE,
        BONSAIDB,
        CUSTODIAN,
        EASYGPU,
        ENGLISHID,
        FABRUIC,
        FIGURES,
        GOOEY,
        GOOEY_CANVAS,
        KLUDGINE,
        NCOG,
        NEBARI,
        POT,
        PROJECTS_PROJECT,
        STYLECS,
    ]
    .into_iter()
    .map(|project| (project.name.to_ascii_lowercase(), project))
    .collect()
});
