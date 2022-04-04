use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

use serde_json::from_reader;
use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>().unwrap();
    #[cfg(target_family = "windows")]
    let path = std::env::var("APPDATA").unwrap() + "/pnp/database.json";
    #[cfg(target_family = "unix")]
    let path = std::env::var("HOME").unwrap() + "/.cache/pnp/database.json";
    let buf_reader = BufReader::new(File::open(path).unwrap());
    let json: HashMap<String, serde_json::Value> = from_reader(buf_reader).unwrap();
    let plugin = json.get_key_value(&name);
    for (_, metadata) in plugin.iter() {
        let repo = metadata["clone_url"].as_str().unwrap().replace(".git", "");
        let maintainer = metadata["owner"]["login"].as_str().unwrap();
        let description = metadata["description"]
            .as_str()
            .unwrap_or("No description available");
        let stars_count = metadata["stargazers_count"].as_u64().unwrap();
        let forks_count = metadata["forks_count"].as_u64().unwrap();
        let updated_date = metadata["updated_at"]
            .as_str()
            .unwrap()
            .replace('T', " ")
            .replace('Z', "");
        let topics_arr = metadata["topics"].as_array().unwrap();
        let mut topics: Vec<&str> = Vec::new();
        for topic in topics_arr {
            topics.push(topic.as_str().unwrap());
        }
        let message = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}",
            format!("{} {}", "Maintainer:", maintainer),
            format!("{} <{}>", "Repository:", repo),
            format!("{} {}", "Description:", description),
            format!("{} {}", "Topics:", topics.join(", ")),
            format!("{} {}", "Stars count:", stars_count),
            format!("{} {}", "Forks count:", forks_count),
            format!("{} {}", "Last update:", updated_date),
        );
        msg.reply(ctx, &message).await?;
    }

    Ok(())
}
