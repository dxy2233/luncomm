pub mod router {
    mod chat;
    pub mod index;
    pub use index::create_router;
}

mod controller {
    pub mod chat;
}
