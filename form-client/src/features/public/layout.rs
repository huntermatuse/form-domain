use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn PublicLayout() -> Element {
    rsx! {
        main { class: "public-app", Outlet::<Route> {} }
    }
}
