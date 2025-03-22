use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use webbrowser;
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tiny_http::{Server, Response, Header};
use mime_guess::Mime;

fn main() {
    // 启动Redis服务器
    let mut redis_child = Command::new("redis\\redis-server.exe")
        .arg("redis.conf")
        .current_dir("redis")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("无法启动Redis服务器");

    thread::sleep(Duration::from_secs(2));

    // 启动wechat849
    let mut wechat_child = Command::new("wechat849\\main.exe")
        .current_dir("wechat849")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("无法启动wechat849");

    // 启动HTTP服务器
    let server = Server::http("0.0.0.0:8001").unwrap();
    println!("WechatRealFriends By StrayMeteor3337");
    println!("在http://localhost:8001启动web静态文件服务, 目录: web");

    // 打开浏览器
    if let Err(e) = webbrowser::open("http://localhost:8001") {
        eprintln!("无法打开浏览器: {}", e);
    }

    // 处理Ctrl+C信号
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // 创建一个线程安全的通道来通知主循环退出
    let (tx, rx) = std::sync::mpsc::channel();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        tx.send(()).unwrap(); // 发送信号通知主循环退出
    }).expect("无法设置信号处理器");

    // 处理HTTP请求
    for request in server.incoming_requests() {
        if !running.load(Ordering::SeqCst) {
            break; // 如果收到退出信号，则停止接受新请求
        }

        let url_path = request.url().split('?').next().unwrap_or("");
        let path = url_path.trim_start_matches('/');
        let mut file_path = format!("web/{}", path);

        // 处理目录请求
        if let Ok(metadata) = std::fs::metadata(&file_path) {
            if metadata.is_dir() {
                file_path.push_str("/index.html");
            }
        }

        match std::fs::read(&file_path) {
            Ok(content) => {
                let mime: Mime = mime_guess::from_path(&file_path).first_or_octet_stream();
                
                // 创建响应并设置MIME类型
                let mut response = Response::from_data(content)
                    .with_status_code(tiny_http::StatusCode(200));
                
                // 添加Content-Type头
                response.add_header(
                    Header::from_bytes("Content-Type", mime.to_string())
                        .unwrap_or_else(|_| Header::from_bytes("Content-Type", "application/octet-stream").unwrap())
                );

                if let Err(e) = request.respond(response) {
                    eprintln!("响应失败: {}", e);
                }
            }
            Err(_) => {
                let response = Response::from_string("404 Not Found")
                    .with_status_code(tiny_http::StatusCode(404))
                    .with_header(Header::from_bytes("Content-Type", "text/plain").unwrap());
                
                if let Err(e) = request.respond(response) {
                    eprintln!("响应失败: {}", e);
                }
            }
        }
    }

    // 等待信号处理器的通知
    let _ = rx.recv();

    // 终止子进程
    println!("正在关闭子进程...");
    let _ = redis_child.kill();
    let _ = wechat_child.kill();

    println!("正在关闭web服务...");
}