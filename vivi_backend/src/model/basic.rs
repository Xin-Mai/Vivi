use crate::tool::error::ErrorMsg;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BasicRsp<T: Serialize> {
    code: i32,
    msg: T,
}

#[derive(Deserialize)]
pub struct SingleStrReq {
    pub id: String,
}

// may fail because of incorrect serialization
pub fn rsp_ok<T: Serialize>(msg: T) -> Result<Vec<u8>, ErrorMsg> {
    let rsp = BasicRsp { code: 0, msg: msg };
    Ok(serde_json::to_vec(&rsp)?)
}

pub fn rsp_err(msg: &'static str) -> Result<Vec<u8>, ErrorMsg> {
    let rsp = BasicRsp {
        code: -1,
        msg: msg.to_string(),
    };
    Ok(serde_json::to_vec(&rsp)?)
}
