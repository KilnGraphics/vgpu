use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::gateway::Activity;
use serenity::model::id::ChannelId;
use serenity::{async_trait, model::gateway::Ready, prelude::*};

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
    let json: serde_json::Value = serde_json::from_str(json_str).unwrap();

    let commit_hash = json.get("after").unwrap().as_str().unwrap();
    let short_hash = &commit_hash[0..7];

    let commiter_pfp = json
        .get("sender")
        .unwrap()
        .get("avatar_url")
        .unwrap()
        .as_str()
        .unwrap();

    TEST_CHANNEL_ID.send_message(http, |m| {
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
    }).await.unwrap();
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
        .expect("Error Creating the Discord Bot");

    while let Some(string) = rx.recv().await {
        process(&string, &client.cache_and_http.http).await;
    }
}
