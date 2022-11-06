use axum::{handler::Handler, http::StatusCode, response::Html, routing::get};

#[tokio::main]
async fn main() {
    // Build our application by creating our router
    let app = axum::Router::new()
        .fallback(fallback.into_service())
        .route("/", get(hello))
        .route("/demo.html", get(get_demo_html))
        .route("/hello.html", get(hello_html))
        .route("/demo-status", get(demo_status))
        .route("/demo-uri", get(demo_uri))
        .route("/demo.png", get(get_demo_png))
        .route(
            "/foo",
            get(get_foo)
                .put(put_foo)
                .patch(patch_foo)
                .post(post_foo)
                .delete(delete_foo),
        );

    // Run our application as a hyper server on http://localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn hello() -> String {
    "Hello, World!".into()
}

/// axum handler for "GET /demo.html" which responds with HTML text.
/// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn get_demo_html() -> Html<&'static str> {
    "<h1>Hello</h1>".into()
}

/// axum handler that responds with typical HTML coming from a file.
/// This uses the Rust macro `std::include_str` to include a UTF-8 file
/// path, relative to `main.rs`, as a `&'static str` at compile time
async fn hello_html() -> Html<&'static str> {
    include_str!("hello.html").into()
}

/// axum hanlder for "GET /demo-status" which returns a HTTP status
/// code, such as OK(200), and a custom user-visible string message.
pub async fn demo_status() -> (axum::http::StatusCode, String) {
    (StatusCode::OK, "Everything is OK".to_string())
}

/// axum hanlder for "GET /demo-uri" which shows the request's own URI.
/// This shows how to write a handler that receives the URI.
pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}

/// axum handler for "GET /demo.png" which responds with an image PNG.
/// This sets a header "image/png" then sends the decoded images data.
async fn get_demo_png() -> impl axum::response::IntoResponse {
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        base64::decode(png).unwrap(),
    )
}

/// axum handler for "GET /foo" whitch returns a string message.
/// This shows our naming convention for HTTP GET handlers.
pub async fn get_foo() -> String {
    "GET foo".to_string()
}

pub async fn put_foo() -> String {
    "PUT foo".to_string()
}

pub async fn patch_foo() -> String {
    "PATCH foo".to_string()
}

pub async fn post_foo() -> String {
    "POST foo".to_string()
}

pub async fn delete_foo() -> String {
    "DELETE foo".to_string()
}

/// axum handler for any request that fails to match the router routes.
/// This implementaion returns HTTP status code Not Found(404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// Tokio signal handler that will wait for a user to press CTRL+C
/// We use this in our hyper `Server` method `with_graceful_shutdown`.`
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
