use std::env;
use serenity::{async_trait, model::gateway::Ready, prelude::*};
use serenity::framework::StandardFramework;
use serenity::model::gateway::{Activity};
use serenity::model::id::{ChannelId, GuildId};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(Activity::playing("Rosella")).await;
        let kiln_guild = GuildId(860320568318099496);

        let channels = kiln_guild.channels(&ctx).await.unwrap();
        let test_channel_id = ChannelId(908861808708837376);
        let test_channel = channels.get(&test_channel_id).expect("Failed to find the Test Log channel!");

        let telejigsawgruntpathicjigsawworldgenjson = env::args().skip(1).next().unwrap();
        let json: serde_json::Value = serde_json::from_str(&telejigsawgruntpathicjigsawworldgenjson).unwrap();

        let commit_hash = json.get("after").unwrap().as_str().unwrap();
        let short_hash = &commit_hash[0..7];

        let commiter_pfp = json.get("sender").unwrap().get("avatar_url").unwrap().as_str().unwrap();

        test_channel.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(0x3BE51D);
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
}

async fn start_bot() {
    let token = include_str!("token");
    let mut client =
        Client::builder(&token)
            .event_handler(Handler)
            .framework(StandardFramework::new())
            .await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[tokio::main]
async fn main() {
    let bot = start_bot();
    bot.await
}