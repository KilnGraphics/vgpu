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

async fn process(payload: &str, http: &Http) {
    if let Ok(event) = serde_json::from_str::<PushEvent>(payload) {
        let commit_hash = event.after;
        let short_hash = &commit_hash[0..7];

        // Unwrapping here is fine if we got this far
        TEST_CHANNEL_ID.send_message(http, |m|
            m.embed(|e| e.colour(EMBED_COLOR)
                .author(|a| a.name(event.sender.login).icon_url(event.sender.avatar_url))
                .description(event.commits.iter().map(|c| format!("[`{}`]({}) {} - {}\n", &c.id[0..7], c.url, c.message, c.author.username)).collect::<String>())
                .field("Compare changes", format!("[here]({})", event.compare), true)
                .field("Test Results", format!("https://hydos.cf/tests/{}", short_hash), true)
                .field("Tests Status", ":white_check_mark: Some Test Name Here \n:x: Some Other Test Name Here\n**2/2** Tests Completed.", false)
            )).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(8);

    // Dump a push event json there when you need to test formatting
    // tx.send(include_str!("test.json").to_string()).await.unwrap();

    tokio::spawn(webhook::start(tx));

    let token = include_str!("token");
    let client = Client::builder(&token)
        .event_handler(Handler)
        .framework(StandardFramework::new())
        .await
        .expect("Error creating the Discord bot");

    while let Some(payload) = rx.recv().await {
        process(&payload, &client.cache_and_http.http).await;
    }
}
