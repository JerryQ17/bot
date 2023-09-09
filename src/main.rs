#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bot = Bot::Bot::from_json("config/config.json");
    bot.init().expect("Bot failed to init!");
    bot.run().await
}