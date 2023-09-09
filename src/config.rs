use std::fs::File;
use std::error::Error;
use std::net::SocketAddr;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

use crate::gocqhttp::GoCqhttp;


#[derive(Deserialize, Serialize)]
pub struct Config {
    addr: SocketAddr,
    gocqhttp: GoCqhttp,
}

impl Config {
    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn gocqhttp(&self) -> &GoCqhttp {
        &self.gocqhttp
    }

    pub fn from_json(path: &str) -> Result<Config, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }
}
