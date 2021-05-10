use crate::tool::error::ErrorMsg;
use mongodb::bson::oid;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BasicRsp<T: Serialize> {
    code: i32,
    msg: T,
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

pub fn deserialize_object_id_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let id = oid::ObjectId::deserialize(deserializer)?;
    Ok(id.to_hex())
}
