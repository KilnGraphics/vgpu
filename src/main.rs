use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::gateway::Activity;
use serenity::model::id::ChannelId;
use serenity::{async_trait, model::gateway::Ready, prelude::*};

use crate::github::PushEvent;

mod github;
mod webhook;

struct Handler;

pub const TEST_CHANNEL_ID: ChannelId = ChannelId(908861808708837376);
pub const EMBED_COLOR: i32 = 0x3BE51D;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(Activity::playing("with Rosella")).await;
    }
}

async fn process(json_str: &str, http: &Http) {
    if let Ok(event) = serde_json::from_str::<PushEvent>(json_str) {
        let commit_hash = event.after;
        let short_hash = &commit_hash[0..7];

        // Unwrapping here is fine if we got this far
        TEST_CHANNEL_ID.send_message(http, |m|
            m.embed(|e| e.colour(EMBED_COLOR)
                .title(format!("{}\n\nTesting on commit {}", event.head_commit.message, commit_hash))
                .description(format!("[Compare changes]({})", event.compare))
                .fields(vec![
                    (":paperclip:Test Results (When Completed)", format!("https://hydos.cf/tests/{}", short_hash).as_str(), false),
                    ("Tests Status", ":white_check_mark: Some Test Name Here \n:x: Some Other Test Name Here\n**2/2** Tests Completed.", false),
                ])
                .footer(|f| f.text(event.sender.login).icon_url(event.sender.avatar_url)))
        ).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(8);
    tokio::spawn(webhook::start(tx));

    let token = include_str!("token");
    let client = Client::builder(&token)
        .event_handler(Handler)
        .framework(StandardFramework::new())
        .await
        .expect("Error creating the Discord bot");

    while let Some(string) = rx.recv().await {
        process(&string, &client.cache_and_http.http).await;
    }
}
