use crate::tool::error::ErrorMsg;

pub fn hello(_: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    Ok("Hello world".as_bytes().to_vec())
}