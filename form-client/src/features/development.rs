use dioxus::prelude::*;

#[component]
pub fn ClientVersionInfoPage() -> Element {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let edition = "2021";
    let build_date = env!("BUILD_DATE");

    rsx! {
        main { class: "admin-shell admin-auth-shell",
            section { class: "admin-panel admin-login-panel",
                h1 { "Client version" }
                dl { class: "version-info-list",
                    div {
                        dt { "name" }
                        dd { "{name}" }
                    }
                    div {
                        dt { "version" }
                        dd { "{version}" }
                    }
                    div {
                        dt { "edition" }
                        dd { "{edition}" }
                    }
                    div {
                        dt { "build_date" }
                        dd { "{build_date}" }
                    }
                }
            }
        }
    }
}
