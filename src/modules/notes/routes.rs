use super::handler::{NotesStore, create_note, delete_note, get_note, get_notes, update_note};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn notes_routes(store: NotesStore) -> Router {
    Router::new()
        .route("/notes", get(get_notes))
        .route("/notes", post(create_note))
        .route("/notes/:id", get(get_note))
        .route("/notes/:id", put(update_note))
        .route("/notes/:id", delete(delete_note))
        .with_state(store)
}
