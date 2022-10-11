use std::{collections::HashMap, time::SystemTime};

use serde::{Deserialize, Serialize};

use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

use crate::Result;
use serde_json::Value;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "clientId")]
    pub client_id: String,

    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    #[serde(rename = "accessTokenExpirationTimestampMs")]
    expiration_time: SystemTime,

    #[serde(rename = "isAnonymous")]
    is_anonymous: bool,
}

const FAKE_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";

pub async fn get_access_token(
    http_client: &reqwest::Client,
    dc: &str,
    key: &str,
) -> Result<AccessTokenResponse> {
    let res = http_client
        .get("https://open.spotify.com/get_access_token?reason=transport&productType=web_player")
        .header("user-agent", FAKE_USER_AGENT)
        .header("Cookie", format!("sp_dc={};sp_key={}", dc, key))
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&res)?)
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientTokenRequest {
    client_data: ClientTokenRequestData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientTokenRequestJsSdkData {
    device_brand: String,
    device_model: String,
    os: String,
    os_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientTokenRequestData {
    client_id: String,
    client_version: String,
    js_sdk_data: ClientTokenRequestJsSdkData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientTokenResponse {
    response_type: String,
    granted_token: ClientTokenResponseGranted,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientTokenResponseGranted {
    token: String,
    expires_after_seconds: u32,
    refresh_after_seconds: u32,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub async fn get_client_token(http_client: &reqwest::Client, client_id: &str) -> Result<String> {
    let fake_client_token_request = ClientTokenRequest {
        client_data: ClientTokenRequestData {
            client_id: client_id.to_string(),
            client_version: "1.1.97.136.g81a082e9".to_string(),
            js_sdk_data: ClientTokenRequestJsSdkData {
                device_brand: "unknown".to_string(),
                device_model: "desktop".to_string(),
                os: "Linux".to_string(),
                os_version: "unknown".to_string(),
            },
        },
    };

    let json = serde_json::to_string(&fake_client_token_request)?;

    let res_body = http_client
        .post("https://clienttoken.spotify.com/v1/clienttoken")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await?
        .text()
        .await?;

    let res: ClientTokenResponse = serde_json::from_str(&res_body)?;

    Ok(res.granted_token.token)
}
