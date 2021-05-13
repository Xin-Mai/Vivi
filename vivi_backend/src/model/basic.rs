use crate::tool::error::ErrorMsg;
use mongodb::options::FindOptions;
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

pub struct OptionWrapper(pub FindOptions);
impl Into<Option<FindOptions>> for OptionWrapper {
    fn into(self) -> Option<FindOptions> {
        Some(self.0)
    }
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
