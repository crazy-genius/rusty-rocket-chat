use std::io::Read;
use curl::easy::{Easy, List};

use super::model::{Result};
use nom::AsBytes;

pub fn do_req(mut client:  Easy, content: Vec<u8>) -> Result<Vec<u8>> {
    let mut dst = Vec::new();
    let mut data: &[u8] = content.as_bytes();

    {
        let mut transfer = client.transfer();

        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap_or(0))
        }).unwrap();

        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();

        transfer.perform().unwrap();
    }

    return Ok(dst)
}

pub fn get_client(mut headers: List) -> Easy {
    let mut client = Easy::new();

    headers.append("Content-type: application/json").unwrap();
    client.http_headers(headers).unwrap();

    return client;
}

pub fn get_authenticated_client(token: String, user_id: String) -> Easy {
    let mut headers = List::new();
    headers.append(&*format!("X-Auth-Token: {}", token)).unwrap();
    headers.append(&*format!("X-User-Id: {}", user_id)).unwrap();

    get_client(headers)
}