use serde::de::DeserializeOwned;
use serde_json::Value;

use super::model::{Result, RocketChatErrorType};

pub fn do_decode<T>(content: Vec<u8>) -> Result<T>
    where T: DeserializeOwned
{
    let r: Value = serde_json::from_slice(content.as_slice())?;

    let status = match r.get("status").unwrap().as_str() {
        Some("success") => true,
        _ => false,
    };

    if !status {
        let msg = r.get("message").unwrap().as_str().unwrap_or("unknown error");
        return Err(RocketChatErrorType::Custom(msg.parse().unwrap()));
    }

    let data = r.get("data")
        .expect("data field required").clone();

    let item: T = serde_json::from_value(data).expect("error");
    return Ok(item);
}