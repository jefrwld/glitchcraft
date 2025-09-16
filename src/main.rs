use axum::{response::Html, routing::{get, post}, Router};
use axum::extract::Multipart;
use axum::body::Bytes;
use std::process::Command;
use std::fs;
use uuid::Uuid;



async fn hello() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}

async fn serve_html() -> Html<String> {
    let html_content = fs::read_to_string("src/static/index.html")
        .expect("Could not find index.html");
    Html(html_content)
}

async fn glitch_handler() -> Html<&'static str> {

    // Command 1: ffgac
    let output1 = Command::new("ffgac")
        .args(["-i", "test.mov", "-an", "-vcodec", "mpeg2video", "-f", "rawvideo", "-y", "simple.mpv"])
        .output();
    
    // Command 2: ffedit  
    let output2 = Command::new("ffedit")
        .args(["-i", "simple.mpv", "-f", "mv", "-s", "simple-glitch.js", "-o", "glitched.mpv"])
        .output();
    
    // Command 3: ffmpeg
    let output3 = Command::new("ffmpeg")
        .args(["-i", "glitched.mpv", "-c:v", "libx264", "-pix_fmt", "yuv420p", "result.mov"])
        .output();
    
    Html("<h1>Glitch commands executed!</h1>")
}

async fn upload_file(video_data: Bytes) -> Result<(), String> {

    let filename = format!("{}.mp4", Uuid::new_v4());
    let uplaod_path = format!("uploads/{}", filename);
    let video: Bytes = video_data;

    //write video
    tokio::fs::write(&uplaod_path, &video).await
        .map_err(|e| format!("write file failed: {}", e))?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_html))
        .route("/glitch", post(glitch_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
