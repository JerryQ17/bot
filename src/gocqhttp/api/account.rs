use reqwest::Response;
use serde::Deserialize;

use crate::Result;
use crate::gocqhttp::GoCqhttp;
use crate::gocqhttp::api::APIResponse;

/// `get_login_info`API的响应数据结构
#[derive(Deserialize)]
pub struct LoginInfo {
    /// QQ号
    pub user_id: i64,
    /// QQ昵称
    pub nickname: String,
}

/// `ModelShowVariants`的元素类型
#[derive(Deserialize)]
pub struct ModelShowVariant {
    /// 在线机型名
    pub model_show: String,
    /// 是否需要会员
    pub need_pay: bool,
}

/// `get_model_show`API的响应数据结构
#[derive(Deserialize)]
pub struct ModelShowVariants {
    /// 在线机型列表
    pub variants: Vec<ModelShowVariant>,
}

/// `ClientDevices`的元素类型
#[derive(Deserialize)]
pub struct ClientDevice {
    /// 客户端ID
    pub app_id: i64,
    /// 设备名称
    pub device_name: String,
    /// 设备类型
    pub device_kind: String,
}

/// `get_online_clients`API的响应数据结构
#[derive(Deserialize)]
pub struct ClientDevices {
    /// 在线客户端列表
    pub clients: Vec<ClientDevice>,
}


impl GoCqhttp {
    /// [获取登录号信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%99%BB%E5%BD%95%E5%8F%B7%E4%BF%A1%E6%81%AF)
    pub async fn get_login_info(&self) -> Result<LoginInfo> {
        let resp = self
            .get("/get_login_info")
            .send()
            .await?;
        APIResponse::<LoginInfo>::from_response(resp)
            .await?
            .data()
    }

    /// [设置登录号资料](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E7%99%BB%E5%BD%95%E5%8F%B7%E8%B5%84%E6%96%99)
    pub async fn set_qq_profile(
        &self,
        nickname: String, company: String, email: String, college: String, personal_note: String,
    ) -> Result<()> {
        let resp = self
            .post("/set_qq_profile")
            .query(&[
                ("nickname", nickname),
                ("company", company),
                ("email", email),
                ("college", college),
                ("personal_note", personal_note)
            ])
            .send()
            .await?;
        APIResponse::<()>::from_response(resp)
            .await?
            .assert_ok()
    }

    /// [获取企点账号信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E4%BC%81%E7%82%B9%E8%B4%A6%E5%8F%B7%E4%BF%A1%E6%81%AF)
    ///
    /// 注意：该API只有企点协议可用，非企点协议的用户应使用`get_login_info`获取登录号信息
    ///
    /// 注意：在go-cqhttp文档中未说明该API的返回值，因此该API返回了一个`Result<Response>`，Response中的内容请自行解析
    pub async fn qidian_get_account_info(&self) -> Result<Response> {
        self.get("/get_login_info")
            .send()
            .await
            .map_err(|e| e.into())
    }

    /// [获取在线机型](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%9C%A8%E7%BA%BF%E6%9C%BA%E5%9E%8B)
    pub async fn get_model_show(&self, model: String) -> Result<ModelShowVariants> {
        let resp = self
            .get("/_get_model_show")
            .query(&[("model", model)])
            .send()
            .await?;
        APIResponse::<ModelShowVariants>::from_response(resp)
            .await?
            .data()
    }

    /// [设置在线机型](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E5%9C%A8%E7%BA%BF%E6%9C%BA%E5%9E%8B)
    pub async fn set_model_show(&self, model: String, model_show: String) -> Result<()> {
        let resp = self
            .post("/_set_model_show")
            .query(&[
                ("model", model),
                ("model_show", model_show)
            ])
            .send()
            .await?;
        APIResponse::<()>::from_response(resp)
            .await?
            .assert_ok()
    }

    /// [获取当前账号在线客户端列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%BD%93%E5%89%8D%E8%B4%A6%E5%8F%B7%E5%9C%A8%E7%BA%BF%E5%AE%A2%E6%88%B7%E7%AB%AF%E5%88%97%E8%A1%A8)
    pub async fn get_online_clients(&self, no_cache: bool) -> Result<ClientDevices> {
        let resp = self
            .get("/_set_model_show")
            .query(&[("no_cache", no_cache)])
            .send()
            .await?;
        APIResponse::<ClientDevices>::from_response(resp)
            .await?
            .data()
    }
}

// 仅测试不会改变QQ的状态的API
#[cfg(test)]
mod tests {
    use super::super::setup_gocqhttp_for_api_test::setup;

    #[tokio::test]
    async fn test_get_login_info() {
        let gch = setup();
        gch.get_login_info().await.unwrap();
    }

    #[tokio::test]
    async fn test_qidian_get_account_info() {
        let gch = setup();
        gch.qidian_get_account_info().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_model_show() {
        let gch = setup();
        gch.get_model_show("iPhone 8".to_string()).await.unwrap();
    }
}