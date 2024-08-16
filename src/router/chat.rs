use crate::controller::chat::{link_handler, send_handler};
use axum::{routing::post, Router};

pub fn chat_router() -> Router {
    Router::new()
        .route("/link", post(link_handler))
        .route("/send", post(send_handler))
}
