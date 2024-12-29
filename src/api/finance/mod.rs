mod currency;

use std::sync::Arc;

use axum::Router;

use crate::api::http::state::StateInner;

pub fn router(state: Arc<StateInner>) -> Router {
    use axum::routing::{get, post};

    Router::new()
        .route(
            currency::numeric_code::get::PATH,
            get(currency::numeric_code::get::handler),
        )
        .route(
            currency::numeric_code::get_code::PATH,
            get(currency::numeric_code::get_code::handler),
        )
        .route(
            currency::transaction::get::PATH,
            get(currency::transaction::get::handler),
        )
        .route(
            currency::transaction::post::PATH,
            post(currency::transaction::post::handler),
        )
        .with_state(state)
}
