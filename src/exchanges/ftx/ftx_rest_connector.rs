use std::time::SystemTime;

use hex::ToHex;
use hmac::Mac;
use reqwest::{header::HeaderMap, Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{FtxApiDetails, HmacSha256};

const FTX_REST_ENDPOINT: &str = "https://ftx.com/api";

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FtxApiResponse {
    Result { success: bool, result: Value },
    Error { success: bool, error: Value },
}

pub struct FtxRestConnector {
    api_details: Option<FtxApiDetails>,
    client: Client,
}

impl FtxRestConnector {
    pub fn new(api_details: Option<FtxApiDetails>) -> Self {
        FtxRestConnector {
            api_details,
            client: Client::new(),
        }
    }

    pub async fn get(
        &self,
        api_path: &str,
        query_params: Option<&mut [(&str, &str)]>,
    ) -> Result<Value, String> {
        let api_url = self.build_api_url(api_path, query_params);
        let full_path = self.get_full_path(&api_url);

        match self
            .client
            .get(api_url)
            .headers(self.auth_header("GET", full_path.as_str()))
            .send()
            .await
        {
            Ok(res) => {
                match res
                    .json::<FtxApiResponse>()
                    .await
                    .expect("Failed to parse json")
                {
                    FtxApiResponse::Result { success: _, result } => Ok(result),
                    FtxApiResponse::Error { success: _, error } => Err(error.to_string()),
                }
            }
            Err(e) => Err(format!("Failed to make request: {e}").to_string()),
        }
    }

    pub fn post() {
        unimplemented!()
    }

    pub fn delete() {
        unimplemented!()
    }

    fn build_api_url(&self, api_path: &str, query_params: Option<&mut [(&str, &str)]>) -> Url {
        match query_params {
            Some(qps) => {
                Url::parse_with_params(format!("{FTX_REST_ENDPOINT}{api_path}").as_str(), qps)
            }
            None => Url::parse(format!("{FTX_REST_ENDPOINT}{api_path}").as_str()),
        }
        .expect("couldn't parse url")
    }

    fn get_full_path(&self, api_url: &Url) -> String {
        match api_url.query() {
            Some(qps_str) => format!("{}?{}", api_url.path(), qps_str),
            None => api_url.path().to_string(),
        }
    }

    fn auth_header(&self, req_method: &str, full_req_path: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(api_details) = &self.api_details {
            let now_in_millis = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let mut mac = HmacSha256::new_from_slice(api_details.api_secret.as_bytes())
                .expect("HMAC can take key of any size");
            let signature_payload = format!("{now_in_millis}{req_method}{full_req_path}");

            mac.update(signature_payload.as_bytes());
            let signature = mac.finalize().into_bytes().encode_hex::<String>();

            headers.insert(
                "FTX-SUBACCOUNT",
                api_details.api_subaccount.parse().unwrap(),
            );
            headers.insert("FTX-KEY", api_details.api_key.parse().unwrap());
            headers.insert("FTX-SIGN", signature.parse().unwrap());
            headers.insert("FTX-TS", now_in_millis.to_string().parse().unwrap());
        }
        headers
    }
}
