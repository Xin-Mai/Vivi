use hyper::header::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, HeaderMap, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use vivi::sign::Signer;
use vivi::ErrorMsg;
#[macro_use]
extern crate lazy_static;

type Operation = (Method, &'static str);
type TokenHandle = fn(Vec<u8>) -> Result<Vec<u8>, ErrorMsg>;
type Handle = fn(Vec<u8>, String) -> Result<Vec<u8>, ErrorMsg>;

lazy_static! {
    static ref SIGNER: Signer = Signer::new();
    static ref LOGIN_TABLE: HashMap<Operation, TokenHandle> = [
            ((Method::POST, "/login"), vivi::user::login as TokenHandle),
            ((Method::POST, "/reg"), vivi::user::register),
        ].iter().cloned().collect();
    static ref FUNCTION_TABLE: HashMap<Operation, Handle> = [
            ((Method::GET, "/hello"), vivi::user::hello_world as Handle),
            // ((Method::GET, "/user"), vivi::user::login as Handle),
            // ((Method::GET, "/user"), vivi::user::register),
        ].iter().cloned().collect();
}

fn process_result(result: Result<Vec<u8>, ErrorMsg>, rsp: &mut Response<Body>) {
    match result {
        Ok(data) => *rsp.body_mut() = Body::from(data),
        Err(msg) => {
            *rsp.status_mut() = msg.code;
            *rsp.body_mut() = Body::from(msg.msg);
        }
    }
}

fn retrieve_token(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    headers
        .get("token")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| SIGNER.verify(v).ok())
        .and_then(|v| Some(v.0))
}

async fn entry(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    // separate request with header and body data([u8])
    let (parts, body) = req.into_parts();
    let data = match hyper::body::to_bytes(body).await {
        Ok(data) => data,
        Err(_) => {
            *response.status_mut() = StatusCode::BAD_REQUEST;
            return Ok(response);
        }
    }
    .to_vec();

    // get function map key
    let key = &(parts.method, parts.uri.path());

    // get token from header
    // if not exist: try to process with register/login request
    // else: try to process with other request
    match retrieve_token(&parts.headers) {
        Some(token) => {
            match FUNCTION_TABLE.get(key) {
                Some(func) => process_result(func(data, token), &mut response),
                None => *response.status_mut() = StatusCode::BAD_REQUEST,
            }
        }
        None => {
            match LOGIN_TABLE.get(key) {
                Some(func) => process_result(func(data), &mut response),
                None => *response.status_mut() = StatusCode::BAD_REQUEST,
            }
        }
    }
    Ok(response)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    // let addr = SocketAddr::from(([127, 0, 0, 1], 12306));
    let addr = SocketAddr::from(([10, 0, 12, 6], 12306));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(entry)) });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
