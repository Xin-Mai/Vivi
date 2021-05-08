extern crate lazy_static;

pub mod sign;
pub mod user;

use hyper::StatusCode;
pub struct ErrorMsg {
    pub code: StatusCode,
    pub msg: Vec<u8>
}