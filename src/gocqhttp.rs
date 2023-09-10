use std::path::Path;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct GoCqhttp {
    path: String,
    server: SocketAddr,
    post: SocketAddr,
}

impl GoCqhttp {
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