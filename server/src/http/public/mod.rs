// public surface
// no auth
// only submission links

use axum::Router;

mod form;

pub fn router() -> Router {
    Router::new().merge(form::router())
}
