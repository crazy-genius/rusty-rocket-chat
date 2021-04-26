pub mod model;
mod client;
mod decoder;

use self::model::{Auth, Result, RocketChatErrorType, ErrorKind};
use self::client::{get_client, get_authenticated_client, do_req};
use self::decoder::{do_decode};

use curl::easy::{Easy, List};
use serde_json::Value;

pub fn authenticate() -> Result<Auth> {
    let data = Vec::from(r#"{
        "user": "***",
        "password": "***"
    }"#);
    let mut client = get_client(List::new());

    client.url("https://pop888.pw/api/v1/login")?;
    client.post(true).unwrap();

    let auth_data: Vec<u8> = do_req(client, data)?;

    let auth: Auth = do_decode(auth_data).unwrap();

    return Ok(auth);
}

pub fn logout(token: String, user_id: String) -> Result<()> {
    let mut client: Easy = get_authenticated_client(token, user_id);

    client.url("https://pop888.pw/api/v1/logout")?;
    client.post(true).unwrap();

    return match do_req(client, Vec::new()) {
        Ok(logout_data) => {
            let data: Value = do_decode(logout_data)?;
            let success: bool = match data.get("status").unwrap().as_str() {
                Some("success") => true,
                None => false,
                _ => false,
            };

            if success {
                return Ok(());
            }

            Err(RocketChatErrorType::Regular(ErrorKind::General))
        },
        Err(err) => {
            Err(err)
        },
    };
}

#[test]
fn test_auth_resp() {
    let data = r#"{
  "status": "success",
  "data": {
      "authToken": "9HqLlyZOugoStsXCUfD_0YdwnNnunAJF8V47U3QHXSq",
      "userId": "aobEdbYhXfu5hkeqG",
      "me": {
            "_id": "aYjNnig8BEAWeQzMh",
            "name": "Rocket Cat",
            "emails": [
                {
                  "address": "rocket.cat@rocket.chat",
                  "verified": false
                }
            ],
            "status": "offline",
            "statusConnection": "offline",
            "username": "rocket.cat",
            "utcOffset": -3,
            "active": true,
            "roles": [
                "admin"
            ],
            "settings": {
                "preferences": {}
              },
            "avatarUrl": "http://localhost:3000/avatar/test"
        }
   }
}"#;

    let auth: Auth = do_decode(Vec::from(data)).unwrap();

    assert_eq!(
        auth.user_id,
        "aobEdbYhXfu5hkeqG"
    )
}
