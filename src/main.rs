mod webhook;

use std::env;
use std::sync::Arc;
use serenity::{async_trait, model::gateway::Ready, prelude::*};
use serenity::framework::StandardFramework;
use serenity::model::channel::GuildChannel;
use serenity::model::gateway::{Activity};
use serenity::model::id::{ChannelId, GuildId};
use serenity::utils::Color;

struct Handler {
    why: Arc<std::sync::RwLock<Option<Box<dyn Fn(String)>>>>,
}

pub const KILN_GUILD_ID: GuildId = GuildId(860320568318099496);
pub const TEST_CHANNEL_ID: ChannelId = ChannelId(908861808708837376);
pub const EMBED_COLOR: i32 = 0x3BE51D;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(Activity::playing("with Rosella")).await;
        let channels = KILN_GUILD_ID.channels(&ctx).await.unwrap();
        let test_channel = channels.get(&TEST_CHANNEL_ID).expect("Failed to find the Test Log channel!");
    }
}

async fn cope(json_str: &str, ctx: Context, channel: &GuildChannel) {
    let json: serde_json::Value = serde_json::from_str(json_str).unwrap();

    let commit_hash = json.get("after").unwrap().as_str().unwrap();
    let short_hash = &commit_hash[0..7];

    let commiter_pfp = json.get("sender").unwrap().get("avatar_url").unwrap().as_str().unwrap();

    channel.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.colour(EMBED_COLOR);
            e.title(format!("Testing Commit {}", short_hash));
            e.description("You're free to ignore this embed.");
            e.image(commiter_pfp);
            e.fields(vec![
                (":paperclip:Test Results (When Completed)", format!("https://hydos.cf/tests/{}", short_hash).as_str(), true),
                ("Tests Status", ":white_check_mark: Some Test Name Here \n:x: Some Other Test Name Here\n**2/2** Tests Completed.", false),
            ]);
            e.footer(|f| {
                f.text("Kiln Graphics v-gpu (Vulkan CI Testing)");
                f
            });
            e
        });
        m
    }).await;
}

#[tokio::main]
async fn main() {
    let arc = Arc::new(std::sync::RwLock::new(None));
    tokio::spawn(webhook::start(arc.clone()));

    let token = include_str!("token");
    let mut client =
        Client::builder(&token)
            .event_handler(Handler { why: arc })
            .framework(StandardFramework::new())
            .await.expect("Error Creating the Discord Bot");

    if let Err(why) = client.start().await {
        println!("Discord Bot Error: {:?}", why);
    }
}