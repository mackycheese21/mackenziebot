use std::marker::PhantomData;
use std::sync::Arc;

use serenity::client::bridge::gateway::ShardManager;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, TypeMapKey};
use tokio::sync::Mutex;

use crate::commands::utils;
use crate::MyKey;

pub async fn logout(ctx: &Context, msg: &Message) -> serenity::Result<Message> {
    if utils::is_owner(&msg.author) {
        let result = msg.channel_id.send_message(&ctx.http, |m| m.content("Logging out")).await;
        let mut map = ctx.data.write().await;
        let shard_manager = map.get_mut::<MyKey<Arc<Mutex<ShardManager>>>>().unwrap();
        shard_manager.lock().await.shutdown_all().await;
        result
    } else {
        msg.channel_id.send_message(&ctx.http, |m| m.content("You must be my owner to do that")).await
    }
}