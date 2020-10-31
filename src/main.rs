use serenity::async_trait;
use serenity::Client;
use serenity::model::gateway::Activity;
use serenity::model::prelude::{Message, Ready};
use serenity::prelude::{Context, EventHandler};

mod dnd;
mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!roll") {
            commands::dnd(&ctx, &msg, msg.content[5..].trim()).await.expect("Error while running the roll command");
        }
    }
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        println!("{:#?}", ctx.http.get_current_application_info().await.expect("Error while getting current application info"));
        ctx.set_activity(Activity::playing("with joe 🤣 😂 😂 😂 🤣 🤣 😂 🤣 🤣 🤣")).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to read .env file");
    let token = std::env::var("TOKEN").expect(".env file had no TOKEN key");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await.expect("Failed to create client");
    client.start().await.expect("Client error")
}