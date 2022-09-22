use crate::utils::get_name;
use serenity::framework::standard::CommandResult;
use serenity::http::HttpBuilder;
use serenity::json::{JsonMap, Value};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

pub async fn set_name(
    _ctx: &Context,
    msg: &Message,
    prefix: Option<String>,
    user_id: u64,
) -> CommandResult {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let http = HttpBuilder::new(token).build();
    let mut payload = JsonMap::new();

    payload.insert(
        "nick".to_string(),
        Value::from(match prefix {
            Some(pre) => format!("{} {}", pre, get_name()),
            None => get_name(),
        }),
    );

    http.edit_member(
        msg.guild_id.expect("Guild ID is not present!").0,
        user_id,
        &payload,
        Some("Its Karl bot..."),
    )
    .await
    .expect("Failed to set nickname");

    Ok(())
}
