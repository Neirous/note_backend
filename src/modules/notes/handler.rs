use crate::modules::notes::model::Note;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::{Arc, Mutex};

// 使用 Arc<Mutex<Vec<Note>>> 作为内存存储
// 在实际应用中应该使用数据库
pub type NotesStore = Arc<Mutex<Vec<Note>>>;

// 获取所有笔记
pub async fn get_notes(State(store): State<NotesStore>) -> Result<Json<Vec<Note>>, StatusCode> {
    let notes = store.lock().unwrap().clone();
    Ok(Json(notes))
}

// 创建新笔记
pub async fn create_note(
    State(store): State<NotesStore>,
    Json(note_data): Json<NoteData>,
) -> Result<Json<Note>, StatusCode> {
    let mut note = Note::new(note_data.title, note_data.content);
    store.lock().unwrap().push(note.clone());
    Ok(Json(note))
}

// 获取单个笔记
pub async fn get_note(
    State(store): State<NotesStore>,
    Path(id): Path<String>,
) -> Result<Json<Note>, StatusCode> {
    let notes = store.lock().unwrap();
    let note = notes
        .iter()
        .find(|note| note.id == id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(note))
}

// 更新笔记
pub async fn update_note(
    State(store): State<NotesStore>,
    Path(id): Path<String>,
    Json(note_data): Json<NoteData>,
) -> Result<Json<Note>, StatusCode> {
    let mut notes = store.lock().unwrap();
    let note = notes
        .iter_mut()
        .find(|note| note.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    note.update(note_data.title, note_data.content);
    Ok(Json(note.clone()))
}

// 删除笔记
pub async fn delete_note(
    State(store): State<NotesStore>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut notes = store.lock().unwrap();
    let size_before = notes.len();
    notes.retain(|note| note.id != id);

    if notes.len() < size_before {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// 用于创建和更新笔记的数据结构
#[derive(serde::Deserialize)]
pub struct NoteData {
    pub title: String,
    pub content: String,
}
