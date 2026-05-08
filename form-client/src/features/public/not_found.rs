use crate::ui::not_found::token_unavailable_page;
use dioxus::prelude::*;

#[component]
pub fn PublicTokenUnavailable(route: Vec<String>) -> Element {
    let _ = route;
    token_unavailable_page()
}

#[component]
pub fn PublicFormTokenUnavailable(route: Vec<String>) -> Element {
    let _ = route;
    token_unavailable_page()
}

#[component]
pub fn PublicViewerTokenUnavailable(route: Vec<String>) -> Element {
    let _ = route;
    token_unavailable_page()
}

#[component]
pub fn PublicTokenMissing() -> Element {
    token_unavailable_page()
}
