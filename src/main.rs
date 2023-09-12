use bot::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    let bot = bot::Bot::from_json("config/config.json");
    bot.init().expect("Bot failed to init!");
    bot.run().await
}