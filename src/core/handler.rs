use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use tokio::task;

use serenity::{
    async_trait,
    builder::CreateApplicationCommands,
    http::CacheHttp,
    model::{
        gateway::Ready,
        id::{ChannelId, GuildId, UserId},
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

use crate::core::*;

pub struct BotHandler {
    app: App,
}

impl BotHandler {
    pub fn new(app: App) -> Self {
        Self { app }
    }

    async fn ping(&self, ctx: Context, command: ApplicationCommandInteraction) {
        info!("calling ping!");
        self.send_msg(&ctx, &command, "pong".to_string()).await;
    }

    async fn send_msg(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
        msg: impl std::string::ToString,
    ) {
        let res = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(msg.to_string()))
            })
            .await;

        if let Err(e) = res {
            error!("send message err: {}\n {:?}", e, command);
        }
    }

    async fn update(&self, ctx: Context, command: ApplicationCommandInteraction) {
        info!("calling update!");
        let user = get_caller_user_id(&command).expect("Couldn't get user id");
        let username = get_caller_user_name(&command).expect("Couldn't get user id");
        let msg = get_string_option(&command, 0).expect("Couldn't get message");

        let did_update = self.app.update(Update::new(user.0, msg.clone()));

        if let Err(e) = did_update {
            warn!("{}", e);
            self.send_msg(&ctx, &command, format!("Failed to send update: {}", e))
                .await;
            return;
        }

        self.send_msg(&ctx, &command, format!("{}: \"{}\"", username, msg))
            .await;
    }

    async fn list_updates(&self, ctx: Context, command: ApplicationCommandInteraction) {
        info!("calling list_updates!");

        let updates = self.app.list_updates();

        if let Err(e) = updates {
            warn!("{}", e);
            self.send_msg(&ctx, &command, format!("Can't get updates: {}", e))
                .await;
            return;
        }
        let updates = updates.unwrap();

        let msgs: Vec<String> = updates
            .iter()
            .map(|update| {
                format!(
                    "[{}]<@{}> \"{}\"",
                    parse_timestamp(update.time).to_rfc3339(),
                    UserId(update.user_id),
                    update.message
                )
            })
            .collect();

        let msgs = msgs.join("\n");

        self.send_msg(&ctx, &command, format!("All Updates\n{}", msgs))
            .await;
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "ping" => {
                    self.ping(ctx, command).await;
                }
                "update" => {
                    self.update(ctx, command).await;
                }
                "list_updates" => {
                    self.list_updates(ctx, command).await;
                }
                _ => error!("err unimplemented"),
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("[{}]{} is connected!", ready.user.id, ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            register_commands(commands)
        })
        .await
        .expect("couldn't create application commands");

        info!(
            "The following application commands are available: {:#?}",
            commands
        );

        let commands = GuildId(494671450985201665)
            .set_application_commands(&ctx.http, |commands| register_commands(commands))
            .await
            .expect("Couldn't create guild commands");

        info!(
            "The following guild commands are available: {:#?}",
            commands
        );

        // let _guild_command = GuildId(494671450985201665)
        //     .create_application_command(&ctx.http, |command| {
        //         command.name("ping").description("A test ping command")
        //     })
        //     .await
        //     .expect("failed to create guild command");
    }
}

fn register_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        .create_application_command(|command| command.name("ping").description("A ping command"))
        .create_application_command(|command| {
            command
                .name("update")
                .description("Send new update")
                .create_option(|option| {
                    option
                        .name("message")
                        .description("Your update message")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                })
        })
        .create_application_command(|command| {
            command.name("list_updates").description("Get Updates")
        })
}

fn get_caller_user_id(
    command: &ApplicationCommandInteraction,
) -> Option<serenity::model::id::UserId> {
    let user_id = match command.member.as_ref().map(|m| m.user.id) {
        Some(ref id) => id.clone(),
        None => {
            return None;
        }
    };
    return Some(user_id);
}

fn get_caller_user_name(command: &ApplicationCommandInteraction) -> Option<String> {
    let user_id = match command.member.as_ref().map(|m| m.user.name.clone()) {
        Some(ref id) => id.clone(),
        None => {
            return None;
        }
    };
    return Some(user_id);
}

fn get_string_option(command: &ApplicationCommandInteraction, index: usize) -> Option<String> {
    return command.data.options.get(index).and_then(|option| {
        if let Some(ApplicationCommandInteractionDataOptionValue::String(string)) = &option.resolved
        {
            return Some(string.clone());
        }
        None
    });
}
