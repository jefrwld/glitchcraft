use axum::{http::StatusCode, response::Html, routing::{get, post}, Router};
use axum::extract::Multipart;
use std::fs;
use axum::extract::DefaultBodyLimit;


async fn hello() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}

async fn serve_html() -> Html<String> {
    let html_content = fs::read_to_string("src/static/index.html")
        .expect("Could not find index.html");
    Html(html_content)
}

async fn glitch_handler(multipart: Multipart) -> &'static str {
   upload_file(multipart).await;
   "Upload complete"
}

async fn upload_file(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(filename) = field.file_name() {
            let data = field.bytes().await.unwrap(); 
            println!("successfully parsed data"); 
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_html))
        .route("/glitch", post(glitch_handler))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)); // 10MB limit

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
