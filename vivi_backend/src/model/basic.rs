use mongodb::bson::oid;
use serde::{Deserialize, Deserializer, Serialize};

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

pub fn deserialize_object_id_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let id = oid::ObjectId::deserialize(deserializer)?;
    Ok(id.to_hex())
}
