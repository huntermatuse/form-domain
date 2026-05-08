use crate::forms::model::QuestionResponse;

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SubmissionDraft {
    pub(super) company_name: String,
    pub(super) signer_name: String,
    pub(super) signer_title: String,
    pub(super) submitted_at: String,
    pub(super) responses: Vec<QuestionResponse>,
}

pub(super) fn default_submission_draft() -> SubmissionDraft {
    let d = js_sys::Date::new_0();
    let today = format!(
        "{:04}-{:02}-{:02}",
        d.get_full_year(),
        d.get_month() + 1,
        d.get_date()
    );

    SubmissionDraft {
        submitted_at: today,
        ..Default::default()
    }
}
