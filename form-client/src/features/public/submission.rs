use crate::api::http::ApiError;
use crate::api::public::{fetch_form, submit_completed_form};
use crate::forms::model::CompletedForm;
use crate::forms::render::submission::FormSubmissionRenderer;
use crate::ui::not_found::token_unavailable_page;
use dioxus::prelude::*;
use web_sys::window;

const PAGE_CSS: &str = r#"
:root{
  --bg:#f6f8fb;
  --card:#ffffff;
  --ink:#1f2937;
  --muted:#6b7280;
  --line:#d1d5db;
  --accent:#1d4ed8;
  --accent-2:#0f766e;
  --warn:#92400e;
  --danger:#b91c1c;
  --danger-bg:#fef2f2;
  --shadow:0 10px 30px rgba(0,0,0,.08);
  --radius:16px;
}

*{ box-sizing:border-box; }

body{
  margin:0;
  font-family:Arial, Helvetica, sans-serif;
  color:var(--ink);
  background:linear-gradient(180deg,#eef4ff 0%, var(--bg) 180px);
  line-height:1.45;
}

.wrap{
  max-width:1100px;
  margin:0 auto;
  padding:0 20px 48px;
}

.toolbar{
  position:sticky;
  top:0;
  z-index:20;
  backdrop-filter:blur(10px);
  background:rgba(246,248,251,.82);
  border-bottom:1px solid rgba(209,213,219,.8);
  width:100vw;
  margin:0 0 32px;
  margin-left:calc(50% - 50vw);
  margin-right:calc(50% - 50vw);
  padding:12px 20px;
}

.toolbar-inner{
  max-width:1100px;
  margin:0 auto;
  display:flex;
  gap:12px;
  flex-wrap:wrap;
  align-items:center;
  justify-content:space-between;
}

.toolbar-actions{
  display:flex;
  gap:10px;
  flex-wrap:wrap;
}

.form-submission__header,
.form-section,
.form-submission-details,
.submitted-notice{
  background:var(--card);
  border:1px solid #e5e7eb;
  border-radius:var(--radius);
  box-shadow:var(--shadow);
  padding:24px;
  margin-bottom:20px;
}

h1{
  margin:0 0 10px;
  font-size:2rem;
}

h2{
  margin:0 0 12px;
  font-size:1.25rem;
  color:#111827;
}

h3{
  margin:18px 0 8px;
  font-size:1rem;
}

p,
li{
  font-size:.98rem;
}

.field label{
  display:block;
  font-weight:700;
  margin-bottom:6px;
  font-size:.92rem;
}

.form-submission-details__grid{
  display:grid;
  grid-template-columns:repeat(auto-fit,minmax(220px,1fr));
  gap:14px;
}

input[type='text'],
input[type='date'],
textarea{
  width:100%;
  border:1px solid var(--line);
  border-radius:12px;
  padding:10px 12px;
  font:inherit;
  background:#fff;
}

input[type='date']{
  min-height:46px;
  color:var(--ink);
  font-weight:600;
  letter-spacing:.01em;
  color-scheme:light;
  cursor:pointer;
}

input[type='date']::-webkit-datetime-edit{
  padding:0;
}

input[type='date']::-webkit-datetime-edit-fields-wrapper{
  display:flex;
  gap:2px;
}

input[type='date']::-webkit-calendar-picker-indicator{
  width:20px;
  height:20px;
  margin-left:8px;
  padding:4px;
  border-radius:8px;
  background-color:#eff6ff;
  cursor:pointer;
}

input[type='date']:focus,
input[type='text']:focus,
textarea:focus{
  outline:3px solid rgba(29,78,216,.18);
  border-color:var(--accent);
}

textarea{
  min-height:96px;
  resize:vertical;
}

.form-question{
  padding:18px 0;
  border-top:1px solid #e5e7eb;
}

.form-question:first-of-type{
  border-top:none;
}

.form-question__heading{
  display:flex;
  gap:10px;
  align-items:center;
  flex-wrap:wrap;
}

.form-question__required{
  display:inline-block;
  font-size:.78rem;
  font-weight:700;
  padding:5px 9px;
  border-radius:999px;
  background:#dbeafe;
  color:#1d4ed8;
}

.form-question__prompt,
.form-question__error{
  margin-top:12px;
}

.form-question__error{
  color:var(--danger);
  font-weight:700;
}

.form-question--missing{
  border-left:4px solid var(--danger);
  padding-left:16px;
}

.form-submission__errors{
  border:1px solid #fecaca;
  background:var(--danger-bg);
  color:#7f1d1d;
  border-radius:14px;
  padding:14px 16px;
  margin-bottom:20px;
}

.radio-row,
.choice-list{
  display:flex;
  flex-wrap:wrap;
  gap:18px;
  margin:10px 0 14px;
}

.radio-row label,
.choice-list label{
  display:flex;
  align-items:center;
  gap:8px;
  font-weight:600;
  cursor:pointer;
}

.markdown h3{
  margin:18px 0 8px;
  font-size:1rem;
}

.markdown blockquote{
  border-left:4px solid var(--warn);
  background:#fff7ed;
  padding:14px 16px;
  border-radius:12px;
  margin:16px 0;
}

.markdown blockquote p{
  margin:0;
}

.form-actions{
  display:flex;
  justify-content:flex-end;
  margin-top:20px;
}

button{
  border:none;
  border-radius:12px;
  padding:11px 16px;
  font:inherit;
  font-weight:700;
  cursor:pointer;
}

.btn-secondary{
  background:#e5e7eb;
  color:#111827;
}

.form-actions button{
  background:var(--accent-2);
  color:#fff;
}

.submitted-notice{
  border-color:#99f6e4;
  background:#f0fdfa;
}

.submitted-notice p{
  margin:0;
}

.submitted-notice--error{
  border-color:#fecaca;
  background:var(--danger-bg);
  color:#7f1d1d;
}

.submit-success-modal{
  position:fixed;
  inset:0;
  z-index:100;
  display:flex;
  align-items:center;
  justify-content:center;
  padding:20px;
  background:rgba(15,23,42,.45);
  backdrop-filter:blur(6px);
}

.submit-success-modal__panel{
  width:min(520px,100%);
  background:#fff;
  border:1px solid #ccfbf1;
  border-radius:20px;
  box-shadow:0 24px 80px rgba(15,23,42,.24);
  padding:28px;
}

.submit-success-modal__panel h2{
  margin:0 0 10px;
}

.submit-success-modal__actions{
  display:flex;
  flex-wrap:wrap;
  gap:10px;
  justify-content:flex-end;
  margin-top:22px;
}

.btn-primary{
  background:var(--accent-2);
  color:#fff;
}

.close-window-note{
  margin-top:14px;
  color:var(--muted);
}

.field__readonly{
  background:#f3f4f6;
  color:#6b7280;
  cursor:default;
  pointer-events:none;
}

input[type='email'],
input[type='tel'],
input[type='number']{
  width:100%;
  border:1px solid var(--line);
  border-radius:12px;
  padding:10px 12px;
  font:inherit;
  background:#fff;
}

input[type='email']:focus,
input[type='tel']:focus,
input[type='number']:focus{
  outline:3px solid rgba(29,78,216,.18);
  border-color:var(--accent);
}

select{
  width:100%;
  border:1px solid var(--line);
  border-radius:12px;
  padding:10px 12px;
  font:inherit;
  background:#fff;
  cursor:pointer;
  appearance:auto;
}

select:focus{
  outline:3px solid rgba(29,78,216,.18);
  border-color:var(--accent);
}

select[multiple]{
  min-height:120px;
  padding:6px;
}

.dropdown-multi-list{
  display:flex;
  flex-direction:column;
  gap:4px;
  margin-top:8px;
  border:1px solid var(--line);
  border-radius:12px;
  overflow:hidden;
}

.dropdown-multi-list__item{
  display:flex;
  align-items:center;
  gap:10px;
  padding:10px 14px;
  font-weight:600;
  cursor:pointer;
  border-bottom:1px solid var(--line);
}

.dropdown-multi-list__item:last-child{
  border-bottom:none;
}

.dropdown-multi-list__item:hover{
  background:#f0f9ff;
}

.ranked-list{
  display:flex;
  flex-direction:column;
  gap:8px;
  margin-top:10px;
}

.ranked-list__item{
  display:flex;
  align-items:center;
  gap:10px;
  padding:10px 12px;
  background:#f9fafb;
  border:1px solid var(--line);
  border-radius:10px;
  cursor:grab;
  user-select:none;
  transition:border-color .15s ease, background .15s ease, opacity .15s ease, transform .15s ease;
}

.ranked-list__item:active{
  cursor:grabbing;
}

.ranked-list__item--dragging{
  opacity:.58;
  border-style:dashed;
  transform:scale(.995);
}

.ranked-list__item--drop-target{
  background:#eff6ff;
  border-color:#2563eb;
}

.ranked-list__rank{
  width:24px;
  height:24px;
  border-radius:50%;
  background:#dbeafe;
  color:#1d4ed8;
  font-size:.78rem;
  font-weight:700;
  display:flex;
  align-items:center;
  justify-content:center;
  flex-shrink:0;
}

.ranked-list__label{
  flex:1;
  font-weight:600;
}

.ranked-list__controls{
  display:flex;
  gap:4px;
}

.ranked-list__btn{
  background:#e5e7eb;
  color:#374151;
  border-radius:6px;
  padding:3px 9px;
  font-size:.85rem;
  border:none;
  cursor:pointer;
}

.ranked-list__btn:disabled{
  opacity:.35;
  cursor:default;
}

@media print{
  .toolbar,
  .form-actions,
  .submit-success-modal,
  .submitted-notice{ display:none; }
  body{ background:#fff; }
  .form-submission__header,
  .form-section,
  .form-submission-details{ box-shadow:none; }
}
"#;

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
      style { "{PAGE_CSS}" }

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
