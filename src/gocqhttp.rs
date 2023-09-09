use std::fmt::Display;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum IP {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl Display for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IP::V4(a, b, c, d) => write!(f, "{}.{}.{}.{}", a, b, c, d),
            IP::V6(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GoCqhttp {
    path: String,
    server: SocketAddr,
    post: SocketAddr,
}

impl GoCqhttp {
    pub fn new(path: String, server: SocketAddr, post: SocketAddr) -> GoCqhttp {
        GoCqhttp { path, server, post }
    }

    pub fn is_running(&self) -> bool {
        let listener = std::net::TcpListener::bind(self.server);
        match listener {
            Ok(_) => {
                drop(listener);
                false
            }
            Err(_) => true,
        }
    }

    pub fn start(&self) -> Result<&Self, Box<dyn std::error::Error>> {
        std::process::Command::new(&self.path).spawn()?;
        Ok(self)
    }
}