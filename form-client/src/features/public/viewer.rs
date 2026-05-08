use crate::api::http::ApiError;
use crate::api::public::fetch_completed_form;
use crate::forms::render::viewer::CompletedFormViewer;
use crate::ui::not_found::token_unavailable_page;
use dioxus::prelude::*;

const VIEWER_CSS: &str = r#"
:root{
  --bg:#f6f8fb;
  --card:#ffffff;
  --ink:#1f2937;
  --muted:#6b7280;
  --line:#d1d5db;
  --accent:#1d4ed8;
  --warn:#92400e;
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
  padding:32px 20px 48px;
}

.completed-form-viewer__header,
.completed-form-submission,
.completed-form-section{
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

h4{
  margin:0 0 8px;
  font-size:.95rem;
  color:var(--muted);
}

p,
li,
dd,
dt{
  font-size:.98rem;
}

.completed-form-submission__grid{
  display:grid;
  grid-template-columns:repeat(auto-fit,minmax(180px,1fr));
  gap:14px;
  margin:0;
}

.completed-form-detail{
  margin:0;
}

.completed-form-detail dt{
  font-weight:700;
  color:var(--muted);
}

.completed-form-detail dd{
  margin:4px 0 0;
}

.completed-form-question{
  padding:18px 0;
  border-top:1px solid var(--line);
}

.completed-form-question:first-of-type{
  border-top:none;
}

.completed-form-question__heading{
  display:flex;
  gap:10px;
  align-items:center;
  flex-wrap:wrap;
}

.completed-form-question__required,
.completed-form-answer__status{
  display:inline-block;
  font-size:.78rem;
  font-weight:700;
  padding:5px 9px;
  border-radius:999px;
  background:#dbeafe;
  color:var(--accent);
}

.completed-form-answer{
  margin-top:16px;
  padding:14px;
  border:1px dashed var(--line);
  border-radius:14px;
  background:#fafafa;
}

.completed-form-answer__comment{
  margin-top:12px;
}

.completed-form-answer__comment p{
  margin:6px 0 0;
}

.completed-form-answer__missing,
.completed-form-answer__meta{
  color:var(--muted);
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
"#;

#[component]
pub fn PublicCompletedFormViewerPage(token: String) -> Element {
    let token_for_fetch = token.clone();
    let completed_form_resource = use_resource(move || {
        let token = token_for_fetch.clone();
        async move { fetch_completed_form(&token).await }
    });

    rsx! {
        style { "{VIEWER_CSS}" }

        div { class: "wrap",
            match &*completed_form_resource.read() {
                Some(Ok(completed_form)) => rsx! {
                    CompletedFormViewer {
                        completed_form: completed_form.clone(),
                    }
                },
                Some(Err(error)) if is_public_token_error(error) => rsx! {
                    {token_unavailable_page()}
                },
                Some(Err(error)) => rsx! {
                    section { class: "completed-form-viewer__header",
                        h1 { "Unable to load completed form" }
                        p { "{error}" }
                    }
                },
                None => rsx! {
                    section { class: "completed-form-viewer__header",
                        h1 { "Loading completed form..." }
                    }
                },
            }
        }
    }
}

fn is_public_token_error(error: &ApiError) -> bool {
    matches!(
        error,
        ApiError::NotFound | ApiError::Gone | ApiError::BadRequest(_)
    )
}
