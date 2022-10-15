use std::convert::Infallible;
use hyper::Body;

async fn handle(_: hyper::Request<Body>) -> Result<hyper::Response<hyper::Body>, Infallible> {
    Ok(hyper::Response::new("Hello, World!".into()))
}

#[tokio::main]
async fn main() {
    let make_svc = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, Infallible>(hyper::service::service_fn(handle))
    });

    let server = hyper::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
