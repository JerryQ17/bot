mod account;
mod file;
mod friend_info;
mod friend_operation;
mod gocqhttp_related;
mod group_message;
mod group_operation;
mod group_setting;
mod management;
mod message;
mod picture;
mod voice;
#[macro_use]
mod macros;

use reqwest::{RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::gocqhttp::GoCqhttp;
use crate::{Error, Result};

#[derive(Deserialize)]
struct APIResponse<T> {
    status: String,
    retcode: i32,
    message: Option<String>,
    wording: Option<String>,
    data: Option<T>,
    echo: Option<String>,
}

enum APIResponseStatus {
    Ok,
    Async,
    Failed,
}

// 从API响应中提取数据
impl<T: DeserializeOwned> APIResponse<T> {
    async fn from_response(response: Response) -> Result<Self> {
        let status = response.status().as_u16();
        match status {
            200 | 401 | 403 | 404 | 406 => Ok(response.json::<APIResponse<T>>().await?),
            _ => {
                let msg = format!("API请求失败，代码{}: {}", status, response.text().await?);
                Err(Error::from(msg))
            }
        }
    }

    async fn get_data_in_response(r: Response) -> Result<T> {
        APIResponse::<T>::from_response(r).await?.data()
    }

    fn data(self) -> Result<T> {
        match self.status() {
            APIResponseStatus::Ok => match self.data {
                Some(data) => Ok(data),
                None => Err(Error::new(&"API请求成功，但没有数据")),
            },
            APIResponseStatus::Async => Err(Error::new(&"已提交异步处理，请等待处理完成")),
            APIResponseStatus::Failed => {
                let msg = self.message.unwrap_or("API请求失败".to_string());
                let detail = self.wording.unwrap_or("原因未知".to_string());
                Err(Error::from(format!("{}: {}", msg, detail)))
            }
        }
    }
}

impl APIResponse<()> {
    async fn assert_ok_in_response(r: Response) -> Result<()> {
        APIResponse::<()>::from_response(r).await?.assert_ok()
    }
}

// 检查API请求状态
impl<T> APIResponse<T> {
    fn status(&self) -> APIResponseStatus {
        match self.status.as_str() {
            "ok" => APIResponseStatus::Ok,
            "async" => APIResponseStatus::Async,
            "failed" => APIResponseStatus::Failed,
            _ => panic!("Invalid API Response Status: {}", self.status),
        }
    }

    fn is_ok(&self) -> bool {
        self.status == "ok" || self.retcode == 0
    }

    fn is_async(&self) -> bool {
        self.status == "async" || self.retcode == 1
    }

    fn is_failed(&self) -> bool {
        self.status == "failed" || (self.retcode != 0 && self.retcode != 1)
    }

    fn assert_ok(self) -> Result<()> {
        match self.status() {
            APIResponseStatus::Ok => Ok(()),
            APIResponseStatus::Async => Err(Error::new(&"已提交异步处理，请等待处理完成")),
            APIResponseStatus::Failed => {
                let msg = self.message.unwrap_or("API请求失败".to_string());
                let detail = self.wording.unwrap_or("原因未知".to_string());
                Err(Error::from(format!("{}: {}", msg, detail)))
            }
        }
    }
}

impl GoCqhttp {
    fn get_builder(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .get(format!("http://{}/{}", self.server, endpoint))
    }

    fn post_builder(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .post(format!("http://{}/{}", self.server, endpoint))
    }

    async fn get(&self, endpoint: &str) -> Result<Response> {
        self.get_builder(endpoint)
            .send()
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestStruct {
        a: i32,
        b: String,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestEmptyStruct {}

    #[test]
    fn test_api_response() {
        let json = r#"{
            "status": "ok",
            "retcode": 0,
            "message": "success",
            "data": {
                "a": 123456,
                "b": "nickname"
            }
        }"#;
        let resp = serde_json::from_str::<APIResponse<TestStruct>>(json).unwrap();
        assert!(resp.is_ok());
        let data = resp.data().unwrap();
        assert_eq!(
            data,
            TestStruct {
                a: 123456,
                b: String::from("nickname")
            }
        );
        assert_eq!(data.a, 123456);
        assert_eq!(data.b, "nickname");
    }

    #[test]
    fn test_api_response_async() {
        let json = r#"{
            "status": "async",
            "retcode": 1,
            "message": "async",
            "wording": "async"
        }"#;
        let resp = serde_json::from_str::<APIResponse<TestStruct>>(json).unwrap();
        assert!(resp.is_async());
        assert_eq!(resp.message.unwrap(), "async");
        assert_eq!(resp.wording.unwrap(), "async");
    }

    #[test]
    #[should_panic]
    fn test_api_response_failed() {
        let json = r#"{
            "status": "failed",
            "retcode": 100,
            "message": "Request Failed",
            "wording": "there is no data"
        }"#;
        let resp = serde_json::from_str::<APIResponse<TestStruct>>(json).unwrap();
        assert!(resp.is_failed());
        resp.data().unwrap();
    }

    #[test]
    fn test_api_response_empty() {
        let json = r#"{
            "status": "ok",
            "retcode": 0,
            "message": "success",
            "data": {}
        }"#;
        let resp = serde_json::from_str::<APIResponse<TestEmptyStruct>>(json).unwrap();
        assert!(resp.is_ok());
        let data = resp.data().unwrap();
        assert_eq!(data, TestEmptyStruct {});
    }

    #[test]
    fn test_api_response_null() {
        let json = r#"{
            "status": "ok",
            "retcode": 0,
            "message": "success",
            "data": null
        }"#;
        let resp = serde_json::from_str::<APIResponse<()>>(json).unwrap();
        assert!(resp.is_ok());
        assert!(resp.assert_ok().is_ok());
    }
}

mod api_test_setup {
    use super::GoCqhttp;
    use std::fs::File;
    use std::io::BufReader;

    pub fn setup() -> GoCqhttp {
        let file = File::open("config/gch_api_test.json").unwrap();
        let reader = BufReader::new(file);
        let gch: GoCqhttp = serde_json::from_reader(reader).unwrap();
        if !gch.is_running() {
            gch.start().unwrap();
        }
        gch
    }
}
