use luncomm::router::create_router;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = create_router().layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6789")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
