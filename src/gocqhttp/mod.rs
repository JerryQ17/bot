mod api;

use std::path::Path;
use reqwest::Client;
use serde::{Deserialize, Deserializer};
use std::net::{SocketAddr, TcpListener};

use crate::Result;


pub struct GoCqhttp {
    path: String,
    server: SocketAddr,
    post: SocketAddr,
    client: Client,
}

impl<'de> Deserialize<'de> for GoCqhttp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct GoCqhttpHelper {
            path: String,
            server: SocketAddr,
            post: SocketAddr,
        }

        let helper = GoCqhttpHelper::deserialize(deserializer)?;
        Ok(GoCqhttp {
            path: helper.path,
            server: helper.server,
            post: helper.post,
            client: Client::new(),
        })
    }
}


impl GoCqhttp {
    pub fn new(path: String, server: SocketAddr, post: SocketAddr) -> GoCqhttp {
        GoCqhttp {
            path, server, post,
            client: Client::new(),
        }
    }
    pub fn is_running(&self) -> bool {
        TcpListener::bind(self.server).is_err()
    }

    pub fn start(&self) -> Result<()> {
        let cwd = Path::new(&self.path)
            .parent()
            .unwrap();
        std::process::Command::new(&self.path)
            .current_dir(cwd)
            .spawn()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_deserialize() {
        let json = r#"{
            "path": "C:\\Users\\Administrator\\Desktop\\go-cqhttp\\go-cqhttp.exe",
            "server": "127.0.0.1:8080",
            "post": "127.0.0.1:8081"
        }"#;
        let gch: super::GoCqhttp = serde_json::from_str(json).unwrap();
        assert_eq!(gch.path, "C:\\Users\\Administrator\\Desktop\\go-cqhttp\\go-cqhttp.exe");
        assert_eq!(gch.server, "127.0.0.1:8080".parse().unwrap());
        assert_eq!(gch.post, "127.0.0.1:8081".parse().unwrap());
        assert!(gch.client.get("https://www.baidu.com").send().await.is_ok());
    }
}