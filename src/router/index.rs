use crate::router::chat::chat_router;
use axum::Router;

pub fn create_router() -> Router {
    Router::new().merge(chat_router())
}
