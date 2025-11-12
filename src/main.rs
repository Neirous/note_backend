// 引入必要的依赖
use crate::modules::notes::{NotesStore, notes_routes}; // 修改此行
use axum::Router;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

mod modules {
    pub use note_backend::modules::*;
}

#[tokio::main]
async fn main() {
    // 初始化数据存储
    let store: NotesStore = Arc::new(Mutex::new(vec![]));

    // 创建应用路由
    let app = Router::new()
        .merge(notes_routes(store))
        .route("/", axum::routing::get(root_handler));

    // 定义服务器地址：本地地址（127.0.0.1）+ 端口 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // 创建 TCP 监听器
    let listener = TcpListener::bind(addr).await.unwrap();

    // 打印启动信息（方便确认服务器是否启动）
    println!("服务器启动成功！访问地址：http://{}", addr);

    // 使用 axum::serve 启动服务
    axum::serve(listener, app).await.unwrap();
}

// 根路径处理器
async fn root_handler() -> &'static str {
    "Hello, 笔记系统！"
}
