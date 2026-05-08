use crate::api::http::ApiError;
use crate::api::public::fetch_completed_form;
use crate::forms::model::CompletedForm;
use crate::forms::render::print::{print_current_document, PrintCompletedFormMount};
use crate::forms::render::viewer::CompletedFormViewer;
use crate::ui::not_found::token_unavailable_page;
use dioxus::prelude::*;

const VIEWER_CSS: &str = r#"
:root{
  --bg:#f3f6fa;
  --card:#ffffff;
  --ink:#111827;
  --body:#1f2937;
  --muted:#5b6472;
  --soft:#6b7280;
  --line:#cfd8e3;
  --line-soft:#e5eaf1;
  --accent:#1d4ed8;
  --accent-soft:#dbeafe;
  --warn:#9a4a11;
  --warn-bg:#fff7ed;
  --shadow:0 18px 45px rgba(15,23,42,.08);
  --radius:12px;
}

*{ box-sizing:border-box; }

body{
  margin:0;
  font-family:Inter, Arial, Helvetica, sans-serif;
  color:var(--ink);
  background:linear-gradient(180deg,#eef4fb 0%, var(--bg) 220px);
  line-height:1.58;
}

.wrap{
  max-width:980px;
  margin:0 auto;
  padding:20px 24px 56px;
}

.public-viewer-toolbar{
  position:sticky;
  top:0;
  z-index:20;
  display:flex;
  align-items:center;
  justify-content:space-between;
  gap:16px;
  margin:-20px 0 24px;
  padding:14px 0 16px;
  background:rgba(243,246,250,.94);
  backdrop-filter:blur(10px);
  border-bottom:1px solid rgba(207,216,227,.9);
}

.public-viewer-toolbar__title{
  min-width:0;
}

.public-viewer-toolbar__title strong,
.public-viewer-toolbar__title span{
  display:block;
}

.public-viewer-toolbar__title strong{
  overflow:hidden;
  color:#111827;
  font-size:1rem;
  line-height:1.25;
  text-overflow:ellipsis;
  white-space:nowrap;
}

.public-viewer-toolbar__title span{
  color:var(--muted);
  font-size:.86rem;
}

.public-viewer-toolbar button{
  flex-shrink:0;
  border:none;
  border-radius:8px;
  padding:10px 14px;
  background:#e6edf7;
  color:#111827;
  font:inherit;
  font-size:.9rem;
  font-weight:700;
  cursor:pointer;
}

.public-viewer-screen .completed-form-viewer{
  gap:22px;
}

.public-viewer-screen .completed-form-viewer__header,
.public-viewer-screen .completed-form-submission,
.public-viewer-screen .completed-form-section{
  background:var(--card);
  border:1px solid var(--line-soft);
  border-radius:var(--radius);
  box-shadow:var(--shadow);
  padding:24px 26px;
  margin-bottom:0;
}

.public-viewer-screen .completed-form-viewer__header h1{
  margin:0 0 12px;
  color:var(--ink);
  font-size:1.55rem;
  line-height:1.2;
  letter-spacing:0;
}

.public-viewer-screen .completed-form-viewer__header .markdown,
.public-viewer-screen .completed-form-viewer__header .markdown p{
  color:var(--body);
  font-size:1rem;
}

.public-viewer-screen .completed-form-submission h2{
  margin:0 0 18px;
  color:#475569;
  font-size:.78rem;
  font-weight:800;
  letter-spacing:.12em;
  text-transform:uppercase;
}

.public-viewer-screen .completed-form-section > h2{
  margin:0 0 18px;
  padding-bottom:12px;
  border-bottom:1px solid var(--line);
  color:#1e3a5f;
  font-size:1.15rem;
  line-height:1.35;
}

.public-viewer-screen p,
.public-viewer-screen li,
.public-viewer-screen dd,
.public-viewer-screen dt{
  color:var(--body);
  font-size:.98rem;
}

.public-viewer-screen .completed-form-submission__grid{
  display:grid;
  grid-template-columns:repeat(auto-fit,minmax(180px,1fr));
  gap:18px;
  margin:0;
}

.public-viewer-screen .completed-form-detail{
  margin:0;
}

.public-viewer-screen .completed-form-detail dt{
  margin-bottom:5px;
  color:#475569;
  font-size:.74rem;
  font-weight:800;
  letter-spacing:.1em;
  text-transform:uppercase;
}

.public-viewer-screen .completed-form-detail dd{
  margin:0;
  color:#1f2937;
  font-weight:700;
  overflow-wrap:anywhere;
}

.public-viewer-screen .completed-form-question{
  padding:22px 0;
  border-top:1px solid var(--line);
}

.public-viewer-screen .completed-form-question:first-of-type{
  border-top:none;
  padding-top:0;
}

.public-viewer-screen .completed-form-question__heading{
  display:flex;
  gap:10px;
  align-items:center;
  flex-wrap:wrap;
  margin-bottom:12px;
}

.public-viewer-screen .completed-form-question__heading h3{
  margin:0;
  color:#172033;
  font-size:1.02rem;
  line-height:1.35;
}

.public-viewer-screen .completed-form-question__required,
.public-viewer-screen .completed-form-answer__status{
  display:inline-block;
  font-size:.72rem;
  font-weight:800;
  padding:4px 8px;
  border-radius:999px;
  background:var(--accent-soft);
  color:var(--accent);
}

.public-viewer-screen .completed-form-question__prompt{
  margin:14px 0 0;
  color:#4b5563;
  font-size:.94rem;
}

.public-viewer-screen .completed-form-question__prompt strong{
  color:#334155;
}

.public-viewer-screen .completed-form-answer{
  margin-top:16px;
  padding:14px 16px;
  border:1px solid var(--line);
  border-radius:10px;
  background:#fbfdff;
}

.public-viewer-screen .completed-form-answer h4{
  margin:0 0 10px;
  color:#475569;
  font-size:.74rem;
  font-weight:800;
  letter-spacing:.1em;
  text-transform:uppercase;
}

.public-viewer-screen .completed-form-answer p,
.public-viewer-screen .completed-form-answer li{
  color:#111827;
  font-size:.96rem;
}

.public-viewer-screen .completed-form-answer ul,
.public-viewer-screen .completed-form-answer ol{
  margin:0;
  padding-left:22px;
}

.public-viewer-screen .completed-form-answer li + li{
  margin-top:4px;
}

.public-viewer-screen .completed-form-answer__comment{
  margin-top:12px;
  padding:10px 12px;
  border-left:4px solid #334155;
  border-radius:6px;
  background:#111827;
}

.public-viewer-screen .completed-form-answer__comment strong{
  display:block;
  margin-bottom:5px;
  color:#cbd5e1;
  font-size:.72rem;
  font-weight:800;
  letter-spacing:.08em;
  text-transform:uppercase;
}

.public-viewer-screen .completed-form-answer__comment p{
  margin:0;
  color:#f8fafc;
  font-size:.94rem;
}

.public-viewer-screen .completed-form-answer__missing,
.public-viewer-screen .completed-form-answer__meta{
  color:var(--muted);
}

.public-viewer-screen .completed-form-answer__meta{
  margin-top:10px;
  color:#64748b !important;
  font-size:.78rem !important;
}

.public-viewer-screen .markdown{
  color:var(--body);
}

.public-viewer-screen .markdown p{
  margin:0 0 12px;
  color:var(--body);
  font-size:.98rem;
}

.public-viewer-screen .markdown p:last-child{
  margin-bottom:0;
}

.public-viewer-screen .markdown h3{
  margin:18px 0 8px;
  color:#111827;
  font-size:1rem;
}

.public-viewer-screen .markdown ul,
.public-viewer-screen .markdown ol{
  margin:8px 0 12px;
  padding-left:24px;
}

.public-viewer-screen .markdown li + li{
  margin-top:3px;
}

.public-viewer-screen .markdown strong{
  color:#111827;
}

.public-viewer-screen .markdown blockquote{
  border-left:4px solid var(--warn);
  background:var(--warn-bg);
  padding:14px 16px;
  border-radius:8px;
  margin:16px 0;
}

.public-viewer-screen .markdown blockquote p{
  margin:0;
  color:#111827;
}

@media (max-width: 720px){
  .wrap{
    padding:14px 12px 36px;
  }

  .public-viewer-toolbar{
    align-items:stretch;
    flex-direction:column;
    margin:-14px 0 18px;
  }

  .public-viewer-toolbar button{
    width:100%;
  }

  .public-viewer-screen .completed-form-viewer__header,
  .public-viewer-screen .completed-form-submission,
  .public-viewer-screen .completed-form-section{
    padding:18px;
  }
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
                    div { class: "public-viewer-screen",
                        PublicViewerToolbar { completed_form: completed_form.clone() }
                        CompletedFormViewer { completed_form: completed_form.clone() }
                    }
                    PrintCompletedFormMount { completed_form: completed_form.clone() }
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

#[component]
fn PublicViewerToolbar(completed_form: CompletedForm) -> Element {
    let title = completed_form.form.title;
    let submitted_at = completed_form.submission.submitted_at;
    let submitted_date = submitted_at.get(..10).unwrap_or(&submitted_at).to_string();

    rsx! {
        div { class: "public-viewer-toolbar",
            div { class: "public-viewer-toolbar__title",
                strong { "{title}" }
                span { "Submitted {submitted_date}" }
            }

            button {
                r#type: "button",
                onclick: move |_| {
                    print_current_document();
                },
                "Download / Print PDF"
            }
        }
    }
}
