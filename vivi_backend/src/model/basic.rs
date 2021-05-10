use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BasicRsp {
    code: i32,
    msg: Vec<u8>,
}

impl BasicRsp {
    pub fn ok(msg: Vec<u8>) -> Self {
        BasicRsp { code: 0, msg: msg }
    }
    pub fn err(msg: &'static str) -> Self {
        BasicRsp {
            code: -1,
            msg: msg.as_bytes().to_vec(),
        }
    }
}
