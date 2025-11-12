pub mod handler;
pub mod model;
pub mod routes;

// 重新导出关键组件
pub use handler::NotesStore;
pub use model::Note;
pub use routes::notes_routes;
