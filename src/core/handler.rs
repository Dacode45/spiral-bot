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

    async fn send_msg(&self, ctx: &Context, command: &ApplicationCommandInteraction, msg: String) {
        let res = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(msg))
            })
            .await;

        if let Err(e) = res {
            error!("send message err: {}\n {:?}", e, command);
        }
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
                _ => error!("err unimplemented"),
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("[{}]{} is connected!", ready.user.id, ready.user.name);

        // let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
        //     register_commands(commands)
        // })
        // .await
        // .expect("couldn't create application commands");

        let commands = GuildId(494671450985201665)
            .set_application_commands(&ctx.http, |commands| register_commands(commands))
            .await
            .expect("Couldn't create guild commands");

        info!(
            "The following guild commands are available: {:#?}",
            commands
        );
    }
}

fn register_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        .create_application_command(|command| command.name("ping").description("A ping command"))
}
