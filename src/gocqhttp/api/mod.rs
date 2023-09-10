mod account;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use reqwest::{RequestBuilder, Response};


use crate::Result;
use super::GoCqhttp;

#[derive(Deserialize)]
struct APIResponse<T> {
    status: String,
    retcode: i32,
    message: Option<String>,
    wording: Option<String>,
    data: Option<T>,
    echo: Option<String>,
}


impl<T: DeserializeOwned> APIResponse<T> {
    pub async fn from_response(response: Response) -> Result<Self> {
        Ok(response
            .json::<APIResponse<T>>()
            .await?)
    }

    fn unwrap(self) -> Result<Option<T>> {
        if self.status == "ok" {
            Ok(self.data)
        } else {
            Err(self.message.unwrap().into())
        }
    }
}


impl GoCqhttp {
    pub fn get(&self, endpoint: &str) -> RequestBuilder {
        self.client.get(self.server.to_string() + endpoint)
    }

    pub fn post(&self, endpoint: &str) -> RequestBuilder {
        self.client.post(self.server.to_string() + endpoint)
    }
}