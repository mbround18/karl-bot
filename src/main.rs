mod utils;

use log::{error, info};
use std::env;

use crate::utils::{get_name, setup_logger};
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::http::HttpBuilder;
use serenity::json::{JsonMap, Value};
use serenity::model::channel::Message;
use serenity::prelude::*;


#[group]
#[commands(eet)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    setup_logger();
    let framework = StandardFramework::new()
        .configure(|c| {
            c.case_insensitivity(true);
            c.prefix("y");
            c
        })
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("Discord Token expected to utilize the bot!");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents.union(GatewayIntents::GUILD_MEMBERS))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    info!("Invite URL: https://discord.com/oauth2/authorize?client_id={}&scope=bot&permissions=309439007808", env::var("DISCORD_CLIENT_ID").expect("App ID must be presented!"));
    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn eet(_ctx: &Context, msg: &Message) -> CommandResult {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let http = HttpBuilder::new(token).build();
    let mut payload = JsonMap::new();
    payload.insert("nick".to_string(), Value::from(get_name()));

    http.edit_member(
        msg.guild_id.expect("Guild ID is not present!").0,
        265150125532381197,
        &payload,
        Some("Its Karl..."),
    )
    .await
    .expect("Failed to set nickname");

    Ok(())
}
