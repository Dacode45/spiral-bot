#[macro_use]
extern crate lazy_static;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use serenity::{
    async_trait,
    builder::CreateApplicationCommands,
    http::CacheHttp,
    model::{
        gateway::Ready,
        id::{ChannelId, GuildId},
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteraction,
                ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
    prelude::*,
};

use std::env;

pub mod core;

use crate::core::*;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    pretty_env_logger::init();
    trace!("trace enabled");
    debug!("debug enabled");
    info!("info enabled");
    warn!("warn enabled");
    error!("error enabled");
    dotenv::dotenv().expect("Failed to read .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");
    let redis_url = env::var("REDIS_URL").expect("Expected a REDIS_URL in the environment");

    let db = DB::new(&redis_url).expect("Couldn't connect");
    let app = App::new(db);

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(BotHandler::new(app))
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
