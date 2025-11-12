pub mod modules;

// 重新导出主要组件
pub use modules::notes::{model::Note, routes::notes_routes};
