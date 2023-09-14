use crate::gocqhttp::api::APIResponse;
use crate::gocqhttp::GoCqhttp;
use crate::{http_post_response, Result};

impl GoCqhttp {
    /// [删除好友](https://docs.go-cqhttp.org/api/#%E5%88%A0%E9%99%A4%E5%A5%BD%E5%8F%8B)
    pub async fn delete_friend(&self, user_id: i64) -> Result<()> {
        let resp = http_post_response!(self, "delete_friend", user_id);
        APIResponse::assert_ok_in_response(resp).await
    }

    /// [删除单向好友](https://docs.go-cqhttp.org/api/#%E5%88%A0%E9%99%A4%E5%8D%95%E5%90%91%E5%A5%BD%E5%8F%8B)
    pub async fn delete_unidirectional_friend(&self, user_id: i64) -> Result<()> {
        let resp = http_post_response!(self, "delete_unidirectional_friend", user_id);
        APIResponse::assert_ok_in_response(resp).await
    }
}
