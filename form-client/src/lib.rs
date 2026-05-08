use dioxus::prelude::*;

pub mod api;
pub mod features;
pub mod forms;
pub mod route;
pub mod ui;

pub use route::Route;

const TOKENS_CSS: Asset = asset!(
    "/assets/css/tokens.css",
    AssetOptions::css().with_static_head(true)
);

const MAIN_CSS: Asset = asset!(
    "/assets/main.css",
    AssetOptions::css().with_static_head(true)
);

const FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TOKENS_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
