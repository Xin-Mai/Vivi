#[macro_use]
extern crate lazy_static;

pub mod db;
pub mod sign;
pub mod user;

pub fn hello(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    Ok("Hello world".as_bytes().to_vec())
}

use hyper::StatusCode;
pub struct ErrorMsg {
    pub code: StatusCode,
    pub msg: &'static str,
}

impl std::convert::From<serde_json::Error> for ErrorMsg {
    fn from(error: serde_json::Error) -> ErrorMsg {
        println!(
            "Error {:?} occur while serialize/deserialize",
            error.classify()
        );
        if error.is_io() {
            ErrorMsg {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                msg: "Could not parse body",
            }
        } else {
            ErrorMsg {
                code: StatusCode::BAD_REQUEST,
                msg: "Could not parse body",
            }
        }
    }
}

impl std::convert::From<mongodb::error::Error> for ErrorMsg {
    fn from(error: mongodb::error::Error) -> ErrorMsg {
        println!("Error {:?} occur while processing database", error);
        ErrorMsg {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            msg: "Database error!"
        }
    }
}
