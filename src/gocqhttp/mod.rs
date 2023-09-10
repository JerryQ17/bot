mod api;

use std::path::Path;
use std::net::{SocketAddr, TcpListener};
use reqwest::Client;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct GoCqhttp {
    path: String,
    server: SocketAddr,
    post: SocketAddr,
    #[serde(skip)]
    client: Client,
}


impl GoCqhttp {
    pub fn is_running(&self) -> bool {
        TcpListener::bind(self.server).is_err()
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cwd = Path::new(&self.path)
            .parent()
            .unwrap();
        std::process::Command::new(&self.path)
            .current_dir(cwd)
            .spawn()?;
        Ok(())
    }
}
