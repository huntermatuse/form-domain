use crate::api::http::ApiError;
use crate::api::public::{fetch_form, submit_completed_form};
use crate::forms::model::CompletedForm;
use crate::forms::render::submission::FormSubmissionRenderer;
use crate::ui::not_found::token_unavailable_page;
use dioxus::prelude::*;
use web_sys::window;

const PAGE_CSS: Asset = asset!("/assets/css/pages/submission.css", AssetOptions::css());

#[component]
pub fn PlasmaRfiPage(token: String) -> Element {
    let token_for_fetch = token.clone();
    let form_resource = use_resource(move || {
        let token = token_for_fetch.clone();
        async move { fetch_form(&token).await }
    });
    let mut is_submitting = use_signal(|| false);
    let mut show_success = use_signal(|| false);
    let mut token_unavailable = use_signal(|| false);
    let mut submit_error = use_signal(|| None::<String>);
    let close_window_note = use_signal(|| None::<String>);

    rsx! {
      document::Link { rel: "stylesheet", href: PAGE_CSS }

      div { class: "wrap",
        if *token_unavailable.read() {
          {token_unavailable_page()}
        } else {
          if let Some(error) = submit_error.read().as_ref() {
            div { class: "submitted-notice submitted-notice--error",
              p { "{error}" }
            }
          }

          match &*form_resource.read() {
              Some(Ok(form)) => rsx! {
                Toolbar { title: form.title.clone() }

                FormSubmissionRenderer {
                  form: form.clone(),
                  is_submitting: *is_submitting.read(),
                  on_submit: {
                      let token = token.clone();
                      move |completed_form: CompletedForm| {
                          if *is_submitting.read() || *show_success.read() {
                              return;
                          }

                          is_submitting.set(true);
                          submit_error.set(None);

                          let token = token.clone();
                          spawn(async move {
                              match submit_completed_form(&token, &completed_form).await {
                                  Ok(()) => {
                                      token_unavailable.set(false);
                                      show_success.set(true);
                                  }
                                  Err(
                                      error,
                                  ) if is_public_token_error(&error) && !*show_success.read() => {
                                      token_unavailable.set(true);
                                  }
                                  Err(error) => {
                                      if !*show_success.read() {
                                          submit_error.set(Some(error.to_string()));
                                      }
                                  }
                              }

                              is_submitting.set(false);
                          });
                      }
                  },
                }

                if *show_success.read() {
                  SubmissionSuccessModal { close_window_note }
                }
              },
              Some(Err(error)) if is_public_token_error(error) => rsx! {
                {token_unavailable_page()}
              },
              Some(Err(error)) => rsx! {
                div { class: "submitted-notice submitted-notice--error",
                  p { "{error}" }
                }
              },
              None => rsx! {
                div { class: "submitted-notice",
                  p { "Loading form..." }
                }
              },
          }
        }
      }
    }
}

#[component]
fn SubmissionSuccessModal(close_window_note: Signal<Option<String>>) -> Element {
    rsx! {
      div {
        class: "submit-success-modal",
        role: "dialog",
        aria_modal: "true",
        aria_label: "Form submitted",

        div { class: "submit-success-modal__panel",
          h2 { "Your response has been submitted." }
          p { "You can download or print a PDF copy of this page before closing the window." }

          div { class: "submit-success-modal__actions",
            button {
              class: "btn-secondary",
              r#type: "button",
              onclick: move |_| {
                  print_page();
              },
              "Download / Print PDF"
            }

            button {
              class: "btn-primary",
              r#type: "button",
              onclick: move |_| {
                  close_window();
                  close_window_note
                      .set(
                          Some(
                              "If the window did not close automatically, you can safely close this tab."
                                  .to_string(),
                          ),
                      );
              },
              "Close Window"
            }
          }

          if let Some(note) = close_window_note.read().as_ref() {
            p { class: "close-window-note", "{note}" }
          }
        }
      }
    }
}

#[component]
fn Toolbar(title: String) -> Element {
    rsx! {
      div { class: "toolbar",
        div { class: "toolbar-inner",
          div {
            strong { "{title}" }
          }

        // div { class: "toolbar-actions",
        //     button {
        //         class: "btn-secondary",
        //         onclick: move |_| {
        //             print_page();
        //         },
        //         "Download / Print PDF"
        //     }
        // }
        }
      }
    }
}

fn print_page() {
    if let Some(window) = window() {
        let _ = window.print();
    }
}

fn close_window() {
    if let Some(window) = window() {
        let _ = window.close();
    }
}

fn is_public_token_error(error: &ApiError) -> bool {
    matches!(error, ApiError::NotFound | ApiError::Gone)
}
