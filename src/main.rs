// 引入必要的依赖
use axum::{
    Router,       // 路由管理器
    routing::get, // 用于定义 GET 路由
};
use std::net::SocketAddr;
use tokio::net::TcpListener; // 用于定义服务器地址（IP + 端口）

// 异步主函数（Axum 基于 Tokio 异步运行时，必须用 #[tokio::main] 标记）
#[tokio::main]
async fn main() {
    // 1. 定义一个简单的处理函数：访问根路径时返回 "Hello, 笔记系统！"
    //   函数返回类型是 &'static str（静态字符串，简单起见）
    let root_handler = || async { "Hello, 笔记系统！" };

    // 2. 创建路由：将根路径（"/"）与上面的处理函数绑定，使用 GET 方法
    let app = Router::new().route("/", get(root_handler)); // get(...) 表示这是一个 GET 请求的路由

    // 3. 定义服务器地址：本地地址（127.0.0.1）+ 端口 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // 4. 创建 TCP 监听器（替代 Server::bind）
    let listener = TcpListener::bind(addr).await.unwrap();

    // 5. 打印启动信息（方便确认服务器是否启动）
    println!("服务器启动成功！访问地址：http://{}", addr);

    // 6. 使用 axum::serve 启动服务
    axum::serve(listener, app).await.unwrap();
}
