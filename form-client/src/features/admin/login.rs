use crate::api;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AdminLoginPage() -> Element {
    let navigator = use_navigator();
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut is_submitting = use_signal(|| false);

    rsx! {
        main { class: "admin-shell admin-auth-shell",
            section { class: "admin-panel admin-login-panel",
                h1 { "Admin" }
                p { class: "admin-muted", "Enter the site password to continue." }

                if let Some(message) = error.read().as_ref() {
                    p { class: "admin-error", "{message}" }
                }

                form {
                    class: "admin-form-stack",
                    onsubmit: move |event| {
                        event.prevent_default();
                        if *is_submitting.read() {
                            return;
                        }
                        is_submitting.set(true);
                        error.set(None);
                        let password_value = password.read().clone();
                        let navigator = navigator.clone();
                        spawn(async move {
                            match api::admin::login(password_value).await {
                                Ok(_) => {
                                    navigator.push(Route::AdminLandingPage {});
                                }
                                Err(err) => error.set(Some(err.to_string())),
                            }
                            is_submitting.set(false);
                        });
                    },
                    label {
                        "Password"
                        input {
                            r#type: "password",
                            autocomplete: "current-password",
                            value: "{password}",
                            oninput: move |event| password.set(event.value()),
                        }
                    }
                    button {
                        class: "admin-primary-button",
                        r#type: "submit",
                        disabled: *is_submitting.read(),
                        if *is_submitting.read() {
                            "Signing in..."
                        } else {
                            "Sign in"
                        }
                    }
                }
            }
        }
    }
}
