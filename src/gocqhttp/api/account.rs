use crate::Result;
use crate::gocqhttp::GoCqhttp;
use crate::gocqhttp::api::APIResponse;


impl GoCqhttp {
    async fn get_login_info(&self) -> Result<()> {
        let resp = self.get("/get_login_info").send().await?;
        APIResponse::<()>::from_response(resp).await?;
        Ok(())
    }
}
