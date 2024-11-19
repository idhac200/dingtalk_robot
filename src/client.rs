use crate::msg::Msg;
use base64::prelude::{Engine, BASE64_STANDARD};
use minreq::{Error, Response};

const API_URL: &str = "https://oapi.dingtalk.com/robot/send";

pub struct Client {
    access_token: String,
    secret: Option<String>,
}

impl Client {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            secret: None,
        }
    }

    pub fn add_secret(&mut self, secret: impl Into<String>) {
        self.secret = Some(secret.into())
    }

    pub fn send(&self, msg: Msg) -> Result<Response, Error> {
        let mut req = minreq::post(API_URL).with_json(&msg)?;
        req = req.with_param("access_token", &self.access_token);
        if let Some(secret) = &self.secret {
            let timestamp = crate::utils::current_timestamp();
            let string_to_sign = format!("{timestamp}\n{secret}");
            let mac = crate::utils::hmac_sha256(secret, string_to_sign);
            req = req
                .with_param("timestamp", timestamp.to_string())
                .with_param("sign", BASE64_STANDARD.encode(mac));
        }
        req.send()
    }
}
