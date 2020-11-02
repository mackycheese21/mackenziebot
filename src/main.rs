use std::marker::PhantomData;
use std::sync::Arc;

use serenity::async_trait;
use serenity::Client;
use serenity::client::bridge::gateway::ShardManager;
use serenity::model::gateway::Activity;
use serenity::model::prelude::{Message, Ready};
use serenity::prelude::{Context, EventHandler, TypeMapKey};
use tokio::sync::Mutex;

mod dnd;
mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!roll") {
            commands::dnd(&ctx, &msg, msg.content[5..].trim()).await.expect("Error while running the roll command");
        } else if msg.content.starts_with("!logout") {
            commands::logout(&ctx, &msg).await.expect("Error while running the logout command");
        }
    }
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        println!("{:#?}", ctx.http.get_current_application_info().await.expect("Error while getting current application info"));
        ctx.set_activity(Activity::playing(&format!("the ready event fired at {}", chrono::Utc::now().date()))).await;
    }
}

struct MyKey<T: Sync + Send + 'static> {
    _phantom: PhantomData<T>
}

impl<T: Sync + Send + 'static> TypeMapKey for MyKey<T> {
    type Value = T;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to read .env file");
    let token = std::env::var("TOKEN").expect(".env file had no TOKEN key");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await.expect("Failed to create client");
    client.data.write().await.insert::<MyKey<Arc<Mutex<ShardManager>>>>(client.shard_manager.clone());
    // client.shard_manager.lock().await.shutdown_all()
    client.start().await.expect("Client error")
}