use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::dnd::{Args, Cursor, Parse};

pub async fn dnd(ctx: &Context, msg: &Message, content: &str) -> serenity::Result<Message> {
    msg.channel_id.send_message(&ctx.http, |m| m.content({
        let cursor = Cursor::new(content);
        let args = Args::parse(cursor);
        match args {
            Ok((_, args)) => {
                args.evaluate()
            }
            Err(loc) => {
                format!("Error parsing dice\n```\n| {}\n| {}^```", content, " ".repeat(loc))
            }
        }
    })).await
}