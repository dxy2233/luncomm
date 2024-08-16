use axum::response::{
    sse::{Event, KeepAlive, Sse},
    Json,
};
use futures_util::stream::{self, Stream};
use serde_json::{json, Value};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;

pub async fn link_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| Event::default().data("固定文本"))
        .map(Ok)
        .throttle(Duration::from_millis(5000));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn send_handler() -> Json<Value> {
    let json_response = json!({
        "content": "Hello, world!",
        "status": "success"
    });
    Json(json_response)
}
