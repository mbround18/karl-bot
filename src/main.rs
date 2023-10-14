mod commands;
mod utils;

use log::{error, info};

use crate::commands::set_name::set_name;
use crate::utils::{get_var, setup_logger};
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::prelude::Message;
use serenity::prelude::*;

#[group]
#[commands(eet, all)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    setup_logger();
    let framework = StandardFramework::new()
        .configure(|c| {
            c.case_insensitivity(true);
            c.prefix("y");
            c
        })
        .group(&GENERAL_GROUP);

    let token = get_var("DISCORD_TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents.union(GatewayIntents::GUILD_MEMBERS))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    info!("Invite URL: https://discord.com/oauth2/authorize?client_id={}&scope=bot&permissions=309439007808", get_var("DISCORD_CLIENT_ID"));
    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn eet(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = get_var("USER_ID")
        .parse::<u64>()
        .expect("Failed to convert environment variable USER_ID to u64");
    info!("Yeet called! Changing nick of {}", user_id);
    set_name(ctx, msg, None, user_id).await
}

#[command]
async fn all(ctx: &Context, msg: &Message) -> CommandResult {
    let super_user_id = get_var("SUPER_USER_ID")
        .parse::<u64>()
        .expect("Failed to convert environment variable SUPER_USER_ID to u64");
    info!("Y'all called! Changing nick of {}", super_user_id);
    set_name(ctx, msg, Some(String::from("Super")), super_user_id).await
}
