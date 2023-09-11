use serde::Deserialize;

use crate::Result;
use crate::gocqhttp::GoCqhttp;
use crate::gocqhttp::api::APIResponse;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub user_id: i64,
    pub nickname: String,
}


impl GoCqhttp {
    /// [获取登录号信息](https://docs.go-cqhttp.org/api/#%E8%8E%B7%E5%8F%96%E7%99%BB%E5%BD%95%E5%8F%B7%E4%BF%A1%E6%81%AF)
    pub async fn get_login_info(&self) -> Result<LoginInfo> {
        let resp = self.get("/get_login_info").send().await?;
        Ok(APIResponse::<LoginInfo>::from_response(resp)
            .await?
            .unwrap_data()
            .unwrap())
    }

    /// [设置登录号资料](https://docs.go-cqhttp.org/api/#%E8%AE%BE%E7%BD%AE%E7%99%BB%E5%BD%95%E5%8F%B7%E8%B5%84%E6%96%99)
    pub async fn set_qq_profile(
        &self,
        nickname: String, company: String, email: String, college: String, personal_note: String,
    ) -> Result<()> {
        let params = [
            ("nickname", nickname),
            ("company", company),
            ("email", email),
            ("college", college),
            ("personal_note", personal_note),
        ];
        let resp = self.post("/set_qq_profile")
            .query(&params)
            .send()
            .await?;
        APIResponse::<()>::from_response(resp).await?;
        Ok(())
    }
}
