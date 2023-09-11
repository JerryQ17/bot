mod account;

use std::io::Error;
use std::io::ErrorKind;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use reqwest::{RequestBuilder, Response};

use crate::Result;
use crate::gocqhttp::GoCqhttp;


#[derive(Deserialize, Debug)]
struct APIResponse<T> {
    status: String,
    retcode: i32,
    message: Option<String>,
    wording: Option<String>,
    data: Option<T>,
    echo: Option<String>,
}


// 从API响应中提取数据
impl<T: DeserializeOwned> APIResponse<T> {
    async fn from_response(response: Response) -> Result<Self> {
        let status = response.status().as_u16();
        match status {
            200 | 401 | 403 | 404 | 406 => Ok(response.json::<APIResponse<T>>().await?),
            _ => {
                let msg = format!("执行API请求失败，代码{}: {}", status, response.text().await?);
                Err(Error::new(ErrorKind::Other, msg).into())
            }
        }
    }

    fn data(self) -> Result<Option<T>> {
        if self.is_ok() {
            Ok(self.data)
        } else if self.is_async() {
            Err(Error::new(
                ErrorKind::Other,
                "已提交异步处理，请等待处理完成",
            ).into())
        } else {
            let msg = self.message.unwrap_or("API Request Failed".to_string());
            let detail = self.wording.unwrap_or("Unknown Reason".to_string());
            Err(Error::new(
                ErrorKind::Other,
                format!("{}: {}", msg, detail),
            ).into())
        }
    }

    fn unwrap_data(self) -> Result<T> {
        let data = self.data()?;
        match data {
            Some(data) => Ok(data),
            None => Err(Error::new(
                ErrorKind::Other,
                "API Request Failed: No Data",
            ).into())
        }
    }
}


// 检查API请求状态
impl<T> APIResponse<T> {
    fn is_ok(&self) -> bool {
        self.status == "ok" || self.retcode == 0
    }

    fn is_async(&self) -> bool {
        self.status == "async" || self.retcode == 1
    }

    fn is_failed(&self) -> bool {
        self.status == "failed" || (self.retcode != 0 && self.retcode != 1)
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
        let data = resp.unwrap_data().unwrap();
        assert_eq!(data, TestStruct { a: 123456, b: "nickname".to_string() });
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
        assert_eq!(data.unwrap(), TestEmptyStruct {});
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
        dbg!(&resp);
        assert!(resp.data().is_ok());
    }
}