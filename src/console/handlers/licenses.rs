use crate::console::ConsoleState;
use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::get,
};
use std::sync::Arc;

/// `/licenses` redirects to Addons (no separate license management).
pub fn urls() -> Router<Arc<ConsoleState>> {
    Router::new().route("/licenses", get(index))
}

pub async fn index(State(state): State<Arc<ConsoleState>>) -> impl IntoResponse {
    let bp = state.base_path().to_string();
    Redirect::to(&format!("{bp}/addons"))
}
