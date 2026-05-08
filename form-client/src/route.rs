use crate::features::admin::{
    AdminFormBuilderPage, AdminFormDetailPage, AdminFormListPage, AdminLandingPage,
    AdminLoginPage, AdminSubmissionDetailPage, AdminSubmissionListPage,
};
use crate::features::development::ClientVersionInfoPage;
use crate::features::home::HomePage;
use crate::features::public::layout::PublicLayout;
use crate::features::public::not_found::{
    PublicFormTokenUnavailable, PublicViewerTokenUnavailable,
};
use crate::features::public::submission::PlasmaRfiPage;
use crate::features::public::viewer::PublicCompletedFormViewerPage;
use crate::ui::not_found::NotFound;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    HomePage,

    #[route("/development/versioning/info")]
    ClientVersionInfoPage,

    #[route("/admin/login")]
    AdminLoginPage,

    #[route("/admin")]
    AdminLandingPage,

    #[route("/admin/forms")]
    AdminFormListPage,

    #[route("/admin/forms/new")]
    AdminFormBuilderPage,

    #[route("/admin/forms/:form_id/:version")]
    AdminFormDetailPage { form_id: String, version: i32 },

    #[route("/admin/forms/:form_id/:version/submissions")]
    AdminSubmissionListPage { form_id: String, version: i32 },

    #[route("/admin/forms/:form_id/:version/submissions/:completed_form_id")]
    AdminSubmissionDetailPage { form_id: String, version: i32, completed_form_id: String },

    #[layout(PublicLayout)]
        #[route("/f/:token")]
        PlasmaRfiPage { token: String },

        #[route("/viewer/:token")]
        PublicCompletedFormViewerPage { token: String },

        #[route("/f/:..route")]
        PublicFormTokenUnavailable { route: Vec<String> },

        #[route("/viewer/:..route")]
        PublicViewerTokenUnavailable { route: Vec<String> },
    #[end_layout]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
