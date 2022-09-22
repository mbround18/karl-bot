mod commands;
mod utils;

use log::{error, info};
use std::env;

use crate::commands::set_name::set_name;
use crate::utils::setup_logger;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(eet, all)]
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
async fn eet(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = env::var("USER_ID")
        .expect("Karls user ID needs to be present")
        .parse::<u64>()
        .expect("Failed to convert environment variable USER_ID to u64");
    info!("Yeet called! Changing nick of {}", user_id);
    set_name(ctx, msg, None, user_id).await
}

#[command]
async fn all(ctx: &Context, msg: &Message) -> CommandResult {
    let super_user_id = env::var("SUPER_USER_ID")
        .expect("Super Karls user ID needs to be present")
        .parse::<u64>()
        .expect("Failed to convert environment variable SUPER_USER_ID to u64");
    info!("Yall called! Changing nick of {}", super_user_id);
    set_name(ctx, msg, Some(String::from("Super")), super_user_id).await
}
