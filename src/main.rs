use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application by creating our router
    let app = axum::Router::new().route("/", get(hello));

    // Run our application as a hyper server on http://localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn hello() -> String {
    "Hello, World!".into()
}