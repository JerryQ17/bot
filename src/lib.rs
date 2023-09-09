mod config;
mod gocqhttp;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use crate::config::Config;

pub struct Bot {
    config: Config,
}

impl Bot {
    pub fn from_json(path: &str) -> Bot {
        Bot { config: Config::from_json(path).unwrap() }
    }

    pub fn init(&self) -> Result<&Self, Box<dyn std::error::Error>> {
        let gch = self.config.gocqhttp();
        if !gch.is_running() {
            gch.start()?;
        }
        Ok(self)
    }

    pub async fn run(&self) -> std::io::Result<()> {
        HttpServer::new(||
            App::new()
                .service(hello)
                .service(echo)
        )
            .bind((self.config.addr().to_string(), 8080))?
            .run()
            .await
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
