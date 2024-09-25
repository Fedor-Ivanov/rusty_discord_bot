mod commands;
mod tournament_state;

use dotenv::dotenv;
use std::env;

use serenity::all::Ready;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Bot is ready!!!");

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::id::register(),
                    commands::tournament::register_start(),
                    commands::tournament::register_info(),
                    commands::tournament::register_join(),
                ],
            )
            .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        println!("Incomming data => msg.content: {}", msg.content);

        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let command_name = command.data.name.as_str();
            println!("Received command interaction: {:?}", command.user);

            let content: Option<String> = match command_name {
                "id" => Some(commands::id::run(&command.data.options())),
                "start" => Some(commands::tournament::run_start(&command.data.options())),
                "info" => Some(commands::tournament::run_info()),
                "join" => Some(commands::tournament::run_join(&command.user)),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data: CreateInteractionResponseMessage =
                    CreateInteractionResponseMessage::new().content(content);
                let builder: CreateInteractionResponse = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client: Client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
