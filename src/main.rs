mod commands;

use commands::ask;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
      println!("Received command interaction: {:#?}", command);

      let content = match command.data.name.as_str() {
        ask::NAME => ask::run(&command.data.options).await,
        _ => "not implemented :(".to_string(),
      };

      if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
          response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(content))
        })
        .await
      {
        println!("Cannot respond to slash command: {}", why);
      }
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    Command::create_global_application_command(&ctx.http, |command| {
      commands::ask::register(command)
    })
    .await
    .unwrap();

    println!("{} is ready!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {
  // Configure the client with your Discord bot token in the environment.
  let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

  // Build our client.
  let mut client = Client::builder(token, GatewayIntents::empty())
    .event_handler(Handler)
    .await
    .expect("Error creating client");

  // Finally, start a single shard, and start listening to events.
  //
  // Shards will automatically attempt to reconnect, and will perform
  // exponential backoff until it reconnects.
  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
