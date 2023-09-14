use serde::Deserialize;

use crate::gocqhttp::api::APIResponse;
use crate::gocqhttp::GoCqhttp;
use crate::{http_get_response, Result};

/// `get_stranger_info`API的响应数据结构
#[derive(Deserialize)]
pub struct StrangeInfo {
    /// QQ号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 性别, male 或 female 或 unknown
    pub sex: String,
    /// 年龄
    pub age: i32,
    /// qid ID身份卡
    pub qid: String,
    /// 等级
    pub level: i32,
    /// 等级
    pub login_days: i32,
}

/// `get_friend_list`API的响应数据结构
#[derive(Deserialize)]
pub struct Friend {
    /// QQ号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 备注名
    pub remark: String,
}

/// `get_unidirectional_friend_list`API的响应数据结构
#[derive(Deserialize)]
pub struct UnidirectionalFriend {
    /// QQ号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 来源
    pub source: String,
}

impl GoCqhttp {
    /// [获取陌生人信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E9%99%8C%E7%94%9F%E4%BA%BA%E4%BF%A1%E6%81%AF)
    pub async fn get_stranger_info(&self, user_id: i64, no_cache: bool) -> Result<StrangeInfo> {
        let resp = http_get_response!(self, "get_stranger_info", user_id, no_cache);
        APIResponse::get_data_in_response(resp).await
    }

    /// [获取好友列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%A5%BD%E5%8F%8B%E5%88%97%E8%A1%A8)
    pub async fn get_friend_list(&self) -> Result<Vec<Friend>> {
        let resp = self.get("get_friend_list").await?;
        APIResponse::get_data_in_response(resp).await
    }

    /// [获取单向好友列表](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E5%8D%95%E5%90%91%E5%A5%BD%E5%8F%8B%E5%88%97%E8%A1%A8)
    pub async fn get_unidirectional_friend_list(&self) -> Result<Vec<UnidirectionalFriend>> {
        let resp = self.get("get_unidirectional_friend_list").await?;
        APIResponse::get_data_in_response(resp).await
    }
}

// 仅测试不会改变QQ的状态的API
#[cfg(test)]
mod tests {
    use super::super::api_test_setup::setup;

    #[tokio::test]
    async fn test_get_strange_info() {
        let gch = setup();
        assert!(gch.get_stranger_info(3364697503, false).await.is_ok())
    }

    #[tokio::test]
    async fn test_get_friend_list() {
        let gch = setup();
        assert!(gch.get_friend_list().await.is_ok())
    }

    #[tokio::test]
    async fn test_get_unidirectional_friend_list() {
        let gch = setup();
        assert!(gch.get_unidirectional_friend_list().await.is_ok())
    }
}
