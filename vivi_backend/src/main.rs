use hyper::header::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, HeaderMap, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use vivi::model::article;
use vivi::model::comment;
use vivi::model::user;
use vivi::tool::error::ErrorMsg;
use vivi::tool::sign;
#[macro_use]
extern crate lazy_static;

type Operation = (Method, &'static str);
type LoginHandle = fn(Vec<u8>) -> Result<Vec<u8>, ErrorMsg>;
type Handle = fn(Vec<u8>, String) -> Result<Vec<u8>, ErrorMsg>;

lazy_static! {
    static ref NO_TKN_TABLE: HashMap<Operation, LoginHandle> = [
        ((Method::POST, "/login"), user::login as LoginHandle),
        ((Method::POST, "/reg"), user::register),
        ((Method::POST, "/article"), article::get_article),
    ]
    .iter()
    .cloned()
    .collect();
    static ref FUNCTION_TABLE: HashMap<Operation, Handle> = [
        (
            (Method::POST, "/user/update/info"),
            user::update_user_info as Handle
        ),
        (
            (Method::POST, "/user/update/avatar"),
            user::update_user_avatar
        ),
        ((Method::POST, "/avatar"), user::download_avatar),
        ((Method::POST, "/article/publish"), article::publish),
        ((Method::POST, "/article/delete"), article::delete_article),
        ((Method::POST, "/article/like"), article::like),
        ((Method::POST, "/comment"), comment::get_comments),
        ((Method::POST, "/comment/publish"), comment::publish),
    ]
    .iter()
    .cloned()
    .collect();
}

fn process_result(result: Result<Vec<u8>, ErrorMsg>, rsp: &mut Response<Body>) {
    match result {
        Ok(data) => {
            *rsp.status_mut() = StatusCode::OK;
            *rsp.body_mut() = Body::from(data);
        }
        Err(msg) => {
            *rsp.status_mut() = msg.code;
            *rsp.body_mut() = Body::from(msg.msg);
        }
    }
}

fn retrieve_id_from_token(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    headers
        .get("token")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| sign::verify(v).ok())
        .and_then(|v| Some(v.0))
}

async fn entry(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!(
        "Receive request from {}, method = {}",
        req.uri(),
        req.method()
    );
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
    match retrieve_id_from_token(&parts.headers) {
        Some(id) => match FUNCTION_TABLE.get(key) {
            Some(func) => process_result(func(data, id), &mut response),
            None => *response.status_mut() = StatusCode::BAD_REQUEST,
        },
        None => match NO_TKN_TABLE.get(key) {
            Some(func) => process_result(func(data), &mut response),
            None => *response.status_mut() = StatusCode::BAD_REQUEST,
        },
    }
    println!("Pcocess request from {} {}", &parts.uri, response.status());
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
