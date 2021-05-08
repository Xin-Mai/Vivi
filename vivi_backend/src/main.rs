use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn dispatcher(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    let (parts, body) = req.into_parts();
    let data = match hyper::body::to_bytes(body).await {
        Ok(data) => data,
        Err(_) => {
            *response.status_mut() = StatusCode::BAD_REQUEST;
            return Ok(response);
        }
    }
    .to_vec();
    *response.body_mut() = Body::from(
        "Hello World!"
    );

    // *response.body_mut() = Body::from(match (&(parts.method), parts.uri.path()) {
    //     (&Method::GET, "/channel") => match channel::channel_get_all() {
    //         Ok(data) => data,
    //         _ => {
    //             *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    //             return Ok(response);
    //         }
    //     },
    //     (&Method::POST, "/channel/send") => match channel::channel_send(data) {
    //         Ok(data) => data,
    //         _ => {
    //             *response.status_mut() = StatusCode::BAD_REQUEST;
    //             return Ok(response)
    //         }
    //     },
    //     _ => {
    //         *response.status_mut() = StatusCode::NOT_FOUND;
    //         "Only support /channel && /channel/send request".as_bytes().to_vec()
    //     }
    // });
    Ok(response)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([1, 116, 248, 164], 12306));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(dispatcher)) });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
