mod config;
mod error;
mod gocqhttp;

use actix_web::{
    App,
    get, post,
    HttpResponse, HttpServer, Responder,
};

pub use crate::error::Error;
use crate::config::Config;

pub type Result<T> = std::result::Result<T, Error>;


pub struct Bot {
    config: Config,
}

impl Bot {
    pub fn from_json(path: &str) -> Bot {
        Bot { config: Config::from_json(path).unwrap() }
    }

    pub fn init(&self) -> Result<()> {
        let gch = self.config.gocqhttp();
        if !gch.is_running() {
            gch.start()?;
        }
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        HttpServer::new(||
            App::new()
                .service(hello)
                .service(echo)
        )
            .bind(self.config.addr())?
            .run()
            .await
            .map_err(|e| e.into())
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
